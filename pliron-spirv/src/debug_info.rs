//! Conversion of op locations to SPIR-V debug data.
//!
//! [`PlironBuilder::with_debug_info`] enables the conversion. Each instruction of an op in a function
//! gets the location of the op:
//! - [`Location::SrcPos`] gives the file, the line and the column.
//! - [`Location::Named`] gives the location of its child. The name of the outermost frame of a
//!   location is the name of a function.
//! - [`Location::CallSite`] gives the location of its callee, inlined at the location of its
//!   caller. The name of the outermost frame of the callee gives the inlined function. If the
//!   callee has no name, the conversion uses the location of the caller.
//! - [`Location::Fused`] gives its first location that converts.
//! - [`Location::Unknown`] gives no line. The instructions stay in the scope of the function.
//!
//! These are the same rules as the `debug-info` feature of `pliron-llvm`.
//!
//! [`DebugInfoFormat`] selects the instructions:
//! - [`DebugInfoFormat::OpLine`] gives `OpLine` and `OpNoLine` for the innermost frame.
//! - [`DebugInfoFormat::NonSemantic`] gives the instructions of `NonSemantic.Shader.DebugInfo.100`.
//!   A debugger or a profiler then shows each inlined function as a separate frame.
//!
//! The conversion obeys these placement rules:
//! - No debug instruction goes before a terminator. `OpSelectionMerge` and `OpLoopMerge` come just
//!   before the terminator of their block, and no instruction can go between a merge instruction
//!   and its branch.
//! - A debug instruction goes just before the first instruction of its op. An op that gives no
//!   instruction in the block gets no debug instruction.
//! - The line and the scope end at the end of each block. Thus the conversion starts each block
//!   with no line and no scope.
//! - `NonSemantic.Shader.DebugInfo.100` instructions cannot go before an `OpPhi` or an `OpVariable`
//!   in a block. Thus the conversion gives no debug instruction to an op that gives an `OpVariable`,
//!   and `DebugFunctionDefinition` goes after the `OpVariable` instructions of the entry block.

use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
use pliron::{
    builtin::{
        op_interfaces::{IsTerminatorInterface, SymbolOpInterface},
        ops::FuncOp,
        types::{IntegerType, Signedness},
    },
    combine::stream::position::SourcePosition,
    context::{Context, Ptr},
    linked_list::ContainsLinkedList,
    location::{Located, Location, Source},
    op::{Op, op_impls},
    operation::Operation,
    result::Result,
    uniqued_any,
    utils::table::HMap,
};
use std::path::Path;
use tracel_rspirv::{
    dr::{InsertPoint, Instruction, ModuleHeader, Operand},
    spirv::{DebugInfoFlags, Op as SpirvOp, SourceLanguage, Word},
};

use crate::{IntoPlironResult, PlironBuilder};

/// The instructions that carry the debug data.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DebugInfoFormat {
    /// Core `OpLine` and `OpNoLine`. Each instruction gets the line of the innermost frame of its
    /// location, with its column. This format shows no inlined frames. All SPIR-V versions and
    /// devices support it.
    #[default]
    OpLine,
    /// The instructions of `NonSemantic.Shader.DebugInfo.100`. Each inlined function is a separate
    /// frame. Below SPIR-V 1.6, the module gets `OpExtension "SPV_KHR_non_semantic_info"`. A Vulkan
    /// device must support `VK_KHR_shader_non_semantic_info` or Vulkan 1.3.
    NonSemantic,
}

/// Options for [`PlironBuilder::with_debug_info`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct DebugInfoOptions {
    /// The instructions that carry the debug data.
    pub format: DebugInfoFormat,
    /// The source language of the compilation unit. Only [`DebugInfoFormat::NonSemantic`] uses it.
    pub language: SourceLanguage,
    /// The name and the version of the compiler. [`DebugInfoFormat::NonSemantic`] writes it in the
    /// `DebugEntryPoint` of each entry point.
    pub producer: String,
    /// The command-line arguments of the compiler. [`DebugInfoFormat::NonSemantic`] writes them in
    /// the `DebugEntryPoint` of each entry point. The default is empty.
    pub arguments: String,
    /// The directory for relative file names. The conversion joins it to each relative file name
    /// with `/`.
    /// An empty directory keeps the file names as they are.
    pub directory: String,
    /// The source text of each file, by the path of the file in the location. Only
    /// [`DebugInfoFormat::NonSemantic`] uses it: the `DebugSource` of the file gets the text.
    pub source_text: BTreeMap<String, String>,
}

impl Default for DebugInfoOptions {
    fn default() -> Self {
        Self {
            format: DebugInfoFormat::default(),
            language: SourceLanguage::Unknown,
            producer: "pliron-spirv".to_string(),
            arguments: String::new(),
            directory: String::new(),
            source_text: BTreeMap::new(),
        }
    }
}

/// The version of the debug data format in `DebugCompilationUnit`. glslang uses the same value.
const DEBUG_INFO_VERSION: u32 = 1;
/// The DWARF version in `DebugCompilationUnit`. glslang uses the same value.
const DWARF_VERSION: u32 = 4;
/// The flags of the functions and of the function type. glslang uses the same value.
const FUNCTION_FLAGS: DebugInfoFlags = DebugInfoFlags::FLAG_IS_PUBLIC;
/// The maximum number of bytes of an `OpString` literal, without the nul terminator. An
/// instruction has at most 0xFFFF words, and `OpString` has two words before its literal.
const MAX_STRING_BYTES: usize = 4 * (0xFFFF - 2) - 1;
/// The extension that non-semantic instruction sets need below SPIR-V 1.6.
const NON_SEMANTIC_EXTENSION: &str = "SPV_KHR_non_semantic_info";

/// A source position: a file, a line and a column.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pos {
    src: Source,
    line: u32,
    column: u32,
}

impl Pos {
    /// The position for a function or a callee without a known position.
    const UNKNOWN: Self = Self {
        src: Source::InMemory,
        line: 0,
        column: 0,
    };

    fn new(src: Source, pos: SourcePosition) -> Self {
        Self {
            src,
            line: u32::try_from(pos.line).unwrap_or(0),
            column: u32::try_from(pos.column).unwrap_or(0),
        }
    }
}

/// A position of a location, and the function that contains it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Frame<'a> {
    /// The inlined function of the frame, as its name and the source of its outermost position.
    /// `None` for the function that contains the op.
    callee: Option<(&'a str, Source)>,
    pos: Pos,
}

/// The frames of `loc`. The first frame is in the function that contains the op. Each other frame
/// is inlined at the frame before it. Empty if `loc` has no position.
fn frames(loc: &Location) -> Vec<Frame<'_>> {
    let mut frames = Vec::new();
    push_frames(loc, None, &mut frames);
    frames
}

/// Pushes the frames of `loc` in the function `callee` to `frames`. Returns false, and pushes
/// nothing, if `loc` has no position.
fn push_frames<'a>(loc: &'a Location, callee: Option<(&'a str, Source)>, frames: &mut Vec<Frame<'a>>) -> bool {
    match loc {
        Location::SrcPos { src, pos } => {
            frames.push(Frame {
                callee,
                pos: Pos::new(*src, *pos),
            });
            true
        }
        Location::Named { child_loc, .. } => push_frames(child_loc, callee, frames),
        Location::CallSite { callee: inner, caller } => {
            if !push_frames(caller, callee, frames) {
                return push_frames(inner, callee, frames);
            }
            let frame = outermost(inner);
            if let Some(name) = frame_name(frame) {
                let src = frame_pos(frame).unwrap_or(Pos::UNKNOWN).src;
                push_frames(inner, Some((name, src)), frames);
            }
            true
        }
        Location::Fused { locations, .. } => locations.iter().any(|loc| push_frames(loc, callee, frames)),
        Location::Unknown => false,
    }
}

/// The outermost frame of `loc`: the last caller of a call site chain.
fn outermost(mut loc: &Location) -> &Location {
    while let Location::CallSite { caller, .. } = loc {
        loc = caller;
    }
    loc
}

/// The first name in a frame.
fn frame_name(loc: &Location) -> Option<&str> {
    match loc {
        Location::Named { name, .. } => Some(name),
        Location::Fused { locations, .. } => locations.iter().find_map(frame_name),
        Location::CallSite { caller, .. } => frame_name(caller),
        Location::SrcPos { .. } | Location::Unknown => None,
    }
}

/// The first source position in a frame.
fn frame_pos(loc: &Location) -> Option<Pos> {
    match loc {
        Location::SrcPos { src, pos } => Some(Pos::new(*src, *pos)),
        Location::Named { child_loc, .. } => frame_pos(child_loc),
        Location::Fused { locations, .. } => locations.iter().find_map(frame_pos),
        Location::CallSite { caller, .. } => frame_pos(caller),
        Location::Unknown => None,
    }
}

/// The location of `func`, or else the first known location in its body.
fn function_location(ctx: &Context, func: FuncOp) -> Option<Location> {
    Some(func.loc(ctx))
        .filter(|loc| !loc.is_unknown())
        .or_else(|| first_known_location(ctx, func.get_operation()))
}

/// The first known location of the ops in the regions of `op`, in pre-order.
fn first_known_location(ctx: &Context, op: Ptr<Operation>) -> Option<Location> {
    op.deref(ctx)
        .regions()
        .flat_map(|region| region.deref(ctx).iter(ctx))
        .flat_map(|block| block.deref(ctx).iter(ctx))
        .find_map(|child| {
            let loc = child.deref(ctx).loc();
            if loc.is_unknown() {
                first_known_location(ctx, child)
            } else {
                Some(loc)
            }
        })
}

/// The path of `src`, as the location gives it.
fn source_path(ctx: &Context, src: Source) -> String {
    match src {
        Source::File(key) => uniqued_any::get(ctx, key).display().to_string(),
        Source::InMemory => "<in-memory>".to_string(),
    }
}

/// `text` in parts that each fit in one `OpString`.
fn split_text(mut text: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    while text.len() > MAX_STRING_BYTES {
        let (part, rest) = text.split_at(text.floor_char_boundary(MAX_STRING_BYTES));
        parts.push(part);
        text = rest;
    }
    parts.push(text);
    parts
}

/// The id of a 32-bit unsigned integer constant.
fn constant(ctx: &Context, builder: &mut PlironBuilder, value: u32) -> Result<Word> {
    let ty = IntegerType::get(ctx, 32, Signedness::Unsigned).to_handle();
    builder.constant_bit32(ctx, ty, value)
}

/// The instructions of a block of the module under construction, by the indices of its function
/// and of the block.
fn block_instructions(builder: &PlironBuilder, (function, block): (usize, usize)) -> &[Instruction] {
    &builder.module_ref().functions[function].blocks[block].instructions
}

/// The line that a debug instruction gives. `file` is an `OpString` or a `DebugSource`.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Line {
    file: Word,
    number: u32,
    column: u32,
}

/// The scope that a `DebugScope` gives.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Scope {
    scope: Word,
    inlined_at: Option<Word>,
}

/// State for converting op locations to debug data.
#[derive(Default)]
pub(crate) struct DebugInfo {
    options: DebugInfoOptions,
    // The compilation unit. It is created with the first function that has debug data.
    unit: Option<Word>,
    // The `DebugTypeFunction` of all functions.
    function_type: Option<Word>,
    // The `DebugSource` of each file.
    sources: HMap<Source, Word>,
    // The `OpString` of the file name of each file, for `OpLine`.
    files: HMap<Source, Word>,
    // The `DebugFunction` of each inlined function, by name and file.
    callees: HMap<(String, Source), Word>,
    // The `DebugInlinedAt` of each line, scope and outer `DebugInlinedAt`.
    inlined_at: HMap<(u32, Word, Option<Word>), Word>,
    // The `DebugFunction` of each function definition, by the id of its `OpFunction`.
    definitions: HMap<Word, Word>,
    // The `DebugFunction` of the current function, if it has debug data.
    function: Option<Word>,
    // The function and the block of the line and the scope below, as indices in the module.
    block: Option<(usize, usize)>,
    // The current line in the block.
    line: Option<Line>,
    // The current scope in the block.
    scope: Option<Scope>,
}

impl DebugInfo {
    pub(crate) fn new(options: DebugInfoOptions) -> Self {
        Self {
            options,
            ..Default::default()
        }
    }

    fn non_semantic(&self) -> bool {
        self.options.format == DebugInfoFormat::NonSemantic
    }

    /// The file name of `src`, with the directory of the options for a relative path.
    ///
    /// The separator is always `/`, so the output does not depend on the host.
    fn file_name(&self, ctx: &Context, src: Source) -> String {
        let path = source_path(ctx, src);
        let directory = self.options.directory.trim_end_matches(['/', '\\']);
        let is_absolute = path.starts_with('/') || Path::new(&path).is_absolute();
        if directory.is_empty() || src == Source::InMemory || is_absolute {
            path
        } else {
            format!("{directory}/{path}")
        }
    }

    /// The `OpString` of the file name of `src`.
    fn file(&mut self, ctx: &Context, builder: &mut PlironBuilder, src: Source) -> Word {
        if let Some(&file) = self.files.get(&src) {
            return file;
        }
        let file = builder.string_ref(self.file_name(ctx, src));
        self.files.insert(src, file);
        file
    }

    /// The `DebugSource` of `src`, with the source text if the options give it.
    fn source(&mut self, ctx: &Context, builder: &mut PlironBuilder, src: Source) -> Word {
        if let Some(&source) = self.sources.get(&src) {
            return source;
        }
        let file = self.file(ctx, builder, src);
        // The `OpString`s go to a different section, so `DebugSourceContinued` still follows its
        // `DebugSource` directly.
        let mut parts = self
            .options
            .source_text
            .get(&source_path(ctx, src))
            .map(|text| split_text(text))
            .unwrap_or_default()
            .into_iter();
        let text = parts.next().map(|part| builder.string_ref(part));
        let source = builder.shader_debug_source(file, text);
        for part in parts {
            let part = builder.string_ref(part);
            builder.shader_debug_source_continued(part);
        }
        self.sources.insert(src, source);
        source
    }

    /// The compilation unit. The first call creates it with the source `src`.
    fn unit(&mut self, ctx: &Context, builder: &mut PlironBuilder, src: Source) -> Result<Word> {
        if let Some(unit) = self.unit {
            return Ok(unit);
        }
        let version = constant(ctx, builder, DEBUG_INFO_VERSION)?;
        let dwarf_version = constant(ctx, builder, DWARF_VERSION)?;
        let source = self.source(ctx, builder, src);
        let language = constant(ctx, builder, self.options.language as u32)?;
        let unit = builder.shader_debug_compilation_unit(version, dwarf_version, source, language);
        Ok(*self.unit.insert(unit))
    }

    /// The `DebugTypeFunction` of all functions. The functions have no debug types, so it has a
    /// void return type and no parameters.
    fn function_type(&mut self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        if let Some(ty) = self.function_type {
            return Ok(ty);
        }
        let flags = constant(ctx, builder, FUNCTION_FLAGS.bits())?;
        let void = builder.type_void();
        let ty = builder.shader_debug_type_function(flags, void, []);
        Ok(*self.function_type.insert(ty))
    }

    /// Creates a `DebugFunction` at the position `pos`.
    fn create_function(
        &mut self,
        ctx: &Context,
        builder: &mut PlironBuilder,
        name: &str,
        linkage_name: &str,
        pos: Pos,
    ) -> Result<Word> {
        let unit = self.unit(ctx, builder, pos.src)?;
        let ty = self.function_type(ctx, builder)?;
        let name = builder.string_ref(name);
        let linkage_name = builder.string_ref(linkage_name);
        let source = self.source(ctx, builder, pos.src);
        let line = constant(ctx, builder, pos.line)?;
        let column = constant(ctx, builder, pos.column)?;
        let flags = constant(ctx, builder, FUNCTION_FLAGS.bits())?;
        Ok(builder.shader_debug_function(name, ty, source, line, column, unit, linkage_name, flags, line, None))
    }

    /// The `DebugFunction` of an inlined function.
    fn callee(&mut self, ctx: &Context, builder: &mut PlironBuilder, name: &str, src: Source) -> Result<Word> {
        let key = (name.to_string(), src);
        if let Some(&callee) = self.callees.get(&key) {
            return Ok(callee);
        }
        let pos = Pos { src, ..Pos::UNKNOWN };
        let callee = self.create_function(ctx, builder, name, name, pos)?;
        self.callees.insert(key, callee);
        Ok(callee)
    }

    /// The `DebugInlinedAt` of `line` in `scope`.
    fn inlined_at(&mut self, ctx: &Context, builder: &mut PlironBuilder, line: u32, scope: Scope) -> Result<Word> {
        let key = (line, scope.scope, scope.inlined_at);
        if let Some(&inlined_at) = self.inlined_at.get(&key) {
            return Ok(inlined_at);
        }
        let line = constant(ctx, builder, line)?;
        let inlined_at = builder.shader_debug_inlined_at(line, scope.scope, scope.inlined_at);
        self.inlined_at.insert(key, inlined_at);
        Ok(inlined_at)
    }

    /// Inserts the debug instructions that give the location `loc` into the selected block at the
    /// index `at`, if the block does not already have that location. `block` gives the indices of
    /// the function and of the selected block.
    fn set_location(
        &mut self,
        ctx: &Context,
        builder: &mut PlironBuilder,
        block: (usize, usize),
        at: usize,
        loc: &Location,
    ) -> Result<()> {
        if self.block != Some(block) {
            self.block = Some(block);
            self.line = None;
            self.scope = None;
        }
        let frames = frames(loc);
        match (self.options.format, self.function) {
            (DebugInfoFormat::OpLine, _) => self.set_op_line(ctx, builder, at, frames.last()),
            (DebugInfoFormat::NonSemantic, Some(function)) => self.set_debug_line(ctx, builder, at, function, &frames),
            (DebugInfoFormat::NonSemantic, None) => Ok(()),
        }
    }

    /// Inserts an `OpLine` for `frame`, or an `OpNoLine` if there is no frame, at the index `at`.
    fn set_op_line(
        &mut self,
        ctx: &Context,
        builder: &mut PlironBuilder,
        at: usize,
        frame: Option<&Frame>,
    ) -> Result<()> {
        let line = frame.map(|frame| Line {
            file: self.file(ctx, builder, frame.pos.src),
            number: frame.pos.line,
            column: frame.pos.column,
        });
        if self.line == line {
            return Ok(());
        }
        let at = InsertPoint::FromBegin(at);
        match line {
            Some(Line { file, number, column }) => builder.insert_line(at, file, number, column),
            None => builder.insert_no_line(at),
        }
        .into_pliron_result()?;
        self.line = line;
        Ok(())
    }

    /// Inserts a `DebugScope` and a `DebugLine` (or a `DebugNoLine`) for `frames` in the function
    /// `function`, from the index `at`.
    fn set_debug_line(
        &mut self,
        ctx: &Context,
        builder: &mut PlironBuilder,
        at: usize,
        function: Word,
        frames: &[Frame],
    ) -> Result<()> {
        let root = Scope {
            scope: function,
            inlined_at: None,
        };
        let scope = frames.windows(2).try_fold(root, |scope, pair| {
            let (caller, Some((name, src))) = (pair[0], pair[1].callee) else {
                return Ok(scope);
            };
            Ok::<_, pliron::result::Error>(Scope {
                inlined_at: Some(self.inlined_at(ctx, builder, caller.pos.line, scope)?),
                scope: self.callee(ctx, builder, name, src)?,
            })
        })?;
        let at = if self.scope == Some(scope) {
            at
        } else {
            builder
                .insert_shader_debug_scope(InsertPoint::FromBegin(at), scope.scope, scope.inlined_at)
                .into_pliron_result()?;
            self.scope = Some(scope);
            at + 1
        };

        let line = frames.last().map(|frame| Line {
            file: self.source(ctx, builder, frame.pos.src),
            number: frame.pos.line,
            column: frame.pos.column,
        });
        if self.line == line {
            return Ok(());
        }
        let at = InsertPoint::FromBegin(at);
        match line {
            Some(Line { file, number, column }) => {
                let number = constant(ctx, builder, number)?;
                let column = constant(ctx, builder, column)?;
                builder.insert_shader_debug_line(at, file, number, number, column, column)
            }
            None => builder.insert_shader_debug_no_line(at),
        }
        .into_pliron_result()?;
        self.line = line;
        Ok(())
    }
}

/// Runs `f` with the debug state of `builder`. Does nothing if `builder` has no debug state.
fn with_state(
    builder: &mut PlironBuilder,
    f: impl FnOnce(&mut DebugInfo, &mut PlironBuilder) -> Result<()>,
) -> Result<()> {
    let Some(mut state) = builder.debug.take() else {
        return Ok(());
    };
    let result = f(&mut state, builder);
    builder.debug = Some(state);
    result
}

/// Converts `op` with `convert`, and gives its instructions the location of `op`.
pub(crate) fn convert_op(
    ctx: &Context,
    builder: &mut PlironBuilder,
    op: Ptr<Operation>,
    convert: impl FnOnce(&mut PlironBuilder) -> Result<()>,
) -> Result<()> {
    let (Some(state), Some(block)) = (
        builder.debug.as_deref(),
        builder.selected_function().zip(builder.selected_block()),
    ) else {
        return convert(builder);
    };
    let non_semantic = state.non_semantic();
    let is_terminator = op_impls::<dyn IsTerminatorInterface>(&*Operation::get_op_dyn(op, ctx));
    if is_terminator || (non_semantic && state.function.is_none()) {
        return convert(builder);
    }
    let loc = op.deref(ctx).loc();
    let start = block_instructions(builder, block).len();
    let set_location = |builder: &mut PlironBuilder| {
        with_state(builder, |state, builder| {
            state.set_location(ctx, builder, block, start, &loc)
        })
    };
    if op.deref(ctx).num_regions() > 0 {
        // The instructions of an op with regions go into more than one block.
        set_location(builder)?;
        return convert(builder);
    }

    // Only the conversion of the op tells if the op gives an instruction in the block. Thus the
    // debug instructions go in after the conversion, before the first instruction of the op.
    convert(builder)?;
    let in_block = builder.selected_function().zip(builder.selected_block()) == Some(block);
    let skip = block_instructions(builder, block)
        .get(start)
        .is_none_or(|first| non_semantic && matches!(first.class.opcode, SpirvOp::Variable | SpirvOp::Phi));
    if in_block && !skip {
        set_location(builder)
    } else {
        Ok(())
    }
}

/// Creates the `DebugFunction` of `func`, whose `OpFunction` the builder just began.
///
/// A function without a location gets no `DebugFunction`, and its instructions get no
/// `NonSemantic.Shader.DebugInfo.100` instructions.
pub(crate) fn begin_function(ctx: &Context, builder: &mut PlironBuilder, func: FuncOp) -> Result<()> {
    with_state(builder, |state, builder| {
        state.function = None;
        state.block = None;
        if !state.non_semantic() {
            return Ok(());
        }
        let Some(loc) = function_location(ctx, func) else {
            return Ok(());
        };
        let loc = outermost(&loc);
        let symbol = func.get_symbol_name(ctx).to_string();
        let name = frame_name(loc).unwrap_or(&symbol);
        let pos = frame_pos(loc).unwrap_or(Pos::UNKNOWN);
        state.function = Some(state.create_function(ctx, builder, name, &symbol, pos)?);
        Ok(())
    })
}

/// Adds the `DebugFunctionDefinition` of the function `function_id` to its entry block, after the
/// `OpVariable` instructions. The builder must not have a selected block.
pub(crate) fn end_function(builder: &mut PlironBuilder, function_id: Word) -> Result<()> {
    with_state(builder, |state, builder| {
        let Some(function) = state.function.take() else {
            return Ok(());
        };
        let index = builder.selected_function().expect("Should be in a function");
        let variables = block_instructions(builder, (index, 0))
            .iter()
            .take_while(|inst| inst.class.opcode == SpirvOp::Variable)
            .count();
        builder.select_block(Some(0)).into_pliron_result()?;
        builder
            .insert_shader_debug_function_definition(InsertPoint::FromBegin(variables), function, function_id)
            .into_pliron_result()?;
        builder.select_block(None).into_pliron_result()?;
        state.definitions.insert(function_id, function);
        Ok(())
    })
}

/// Adds the module-level debug data that needs the complete module: a `DebugEntryPoint` for each
/// entry point with debug data, and the extension for non-semantic instruction sets.
pub(crate) fn finish(builder: &mut PlironBuilder) {
    let Some(DebugInfo {
        unit: Some(unit),
        definitions,
        options,
        ..
    }) = builder.debug.take().map(|state| *state)
    else {
        return;
    };
    let entry_points = builder
        .module_ref()
        .entry_points
        .iter()
        .filter_map(|inst| match inst.operands.get(1) {
            Some(Operand::IdRef(id)) => definitions.get(id).copied(),
            _ => None,
        })
        .collect::<Vec<_>>();
    if !entry_points.is_empty() {
        let signature = builder.string_ref(options.producer);
        let arguments = builder.string_ref(options.arguments);
        for function in entry_points {
            builder.shader_debug_entry_point(function, unit, signature, arguments);
        }
    }

    // A builder without a header emits the default header version.
    let version = builder.version().unwrap_or_else(|| ModuleHeader::new(0).version());
    let has_extension = builder.module_ref().extensions.iter().any(
        |inst| matches!(inst.operands.as_slice(), [Operand::LiteralString(name)] if name == NON_SEMANTIC_EXTENSION),
    );
    if version < (1, 6) && !has_extension {
        builder.extension(NON_SEMANTIC_EXTENSION);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{boxed::Box, vec};

    fn named(name: &str, src: Source, line: i32) -> Location {
        Location::Named {
            name: name.to_string(),
            child_loc: Box::new(Location::SrcPos {
                src,
                pos: SourcePosition { line, column: 1 },
            }),
        }
    }

    fn call_site(callee: Location, at: Location) -> Location {
        Location::CallSite {
            callee: Box::new(callee),
            caller: Box::new(at),
        }
    }

    fn frame(callee: Option<&str>, line: u32) -> Frame<'_> {
        Frame {
            callee: callee.map(|name| (name, Source::InMemory)),
            pos: Pos {
                src: Source::InMemory,
                line,
                column: 1,
            },
        }
    }

    /// A call site chain gives one frame for each function, outermost first.
    #[test]
    fn call_site_chain_gives_frames() {
        let src = Source::InMemory;
        let mid = call_site(named("mid", src, 7), named("kernel", src, 12));
        let inner = call_site(named("inner", src, 3), mid);
        assert_eq!(
            frames(&inner),
            vec![frame(None, 12), frame(Some("mid"), 7), frame(Some("inner"), 3)]
        );
    }

    /// A callee that is itself a call site chain nests in the same way.
    #[test]
    fn nested_callee_gives_frames() {
        let src = Source::InMemory;
        let callee = call_site(named("inner", src, 3), named("mid", src, 7));
        let loc = call_site(callee, named("kernel", src, 12));
        assert_eq!(
            frames(&loc),
            vec![frame(None, 12), frame(Some("mid"), 7), frame(Some("inner"), 3)]
        );
    }

    /// A callee without a name gives the location of the caller.
    #[test]
    fn unnamed_callee_gives_caller() {
        let src = Source::InMemory;
        let callee = Location::SrcPos {
            src,
            pos: SourcePosition { line: 3, column: 1 },
        };
        let loc = call_site(callee, named("kernel", src, 12));
        assert_eq!(frames(&loc), vec![frame(None, 12)]);
    }

    /// A caller without a position gives the callee in the scope of the caller.
    #[test]
    fn unknown_caller_gives_callee() {
        let loc = call_site(named("inner", Source::InMemory, 3), Location::Unknown);
        assert_eq!(frames(&loc), vec![frame(None, 3)]);
    }

    /// `Fused` gives its first location that converts, and `Unknown` gives no frame.
    #[test]
    fn fused_and_unknown() {
        let loc = Location::Fused {
            metadata: None,
            locations: vec![Location::Unknown, named("kernel", Source::InMemory, 5)],
        };
        assert_eq!(frames(&loc), vec![frame(None, 5)]);
        assert!(frames(&Location::Unknown).is_empty());
    }

    /// A long text splits at character boundaries into parts that fit in an `OpString`.
    #[test]
    fn long_text_splits() {
        let text = "é".repeat(MAX_STRING_BYTES);
        let parts = split_text(&text);
        assert_eq!(parts.concat(), text);
        assert!(parts.iter().all(|part| part.len() <= MAX_STRING_BYTES));
        assert_eq!(parts.len(), 3);
        assert_eq!(split_text(""), vec![""]);
    }
}
