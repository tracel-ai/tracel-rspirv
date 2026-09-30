//! Tests for the conversion of op locations to SPIR-V debug data.
//!
//! Each test uses the same small module. The module has a function, a callee that is inlined
//! twice, a loop, and a branch. The locations have the shape that cubecl gives: a `Named` frame
//! for each function, and a `CallSite` chain for each inlined call.

use pliron::{context::Context, op::verify_op, operation::Operation, parsable::parse_from_str, printable::Printable};
use pliron_spirv::{
    PlironBuilder,
    ToSpirvOp,
    debug_info::{DebugInfoFormat, DebugInfoOptions},
    ops::SpirvModuleOp,
};
use tracel_rspirv::{
    binary::Assemble,
    dr::{Instruction, Module, Operand},
    spirv::{DebugInfoOp, Op as SpirvOp, Word},
};

/// The binary of the test module, emitted by the base commit without debug data.
const BASE_BINARY: &[u8] = include_bytes!("data/debug_info_base.spv");

/// The test module. `inner` and `mid` are in `src/lib.rs`, and `mid` calls `inner`:
///
/// ```text
/// fn kernel() {                                   // kernel.rs:1
///     let count = ..; let value = ..;             // kernel.rs:4-5
///     for index in count.. {                      // kernel.rs:6
///         if index != count {                     // kernel.rs:7
///             value = mid(value);                 // kernel.rs:8
///         }
///         index = index + count;                  // kernel.rs:10
///     }
///     value = mid(value);                         // kernel.rs:12, only `inner` of `mid` remains
///     value = value * value;                      // no location
/// }
/// ```
const MODULE: &str = r#"
spirv.module @module Logical GLSL450 requires : <v1.6, [Shader], []> {
  ^module_block():
    builtin.func @main: builtin.function <() -> (builtin.unit)> {
      ^entry():
        count_ptr = spirv.Variable Function : <spirv.ptr <builtin.integer ui32, Function>> !0;
        value_ptr = spirv.Variable Function : <spirv.ptr <spirv.float 32, Function>> !1;
        count = spirv.Load count_ptr : <builtin.integer ui32> !2;
        value = spirv.Load value_ptr : <spirv.float 32> !3;
        spirv_pliron.loop {
          ^loop_entry():
            spirv.Branch ^header(count) !4

          ^header(index: builtin.integer ui32):
            less = spirv.ULessThan index, count : <builtin.integer i1> !5;
            spirv.BranchConditional (less) [^body, ^loop_merge] [builtin_operand_segment_sizes: builtin.operand_segment_sizes [1, 0, 0]]: <(builtin.integer i1) -> ()> !6

          ^body():
            not_equal = spirv.INotEqual index, count : <builtin.integer i1> !7;
            spirv_pliron.selection {
              ^selection_entry():
                spirv.BranchConditional (not_equal) [^then, ^selection_merge] [builtin_operand_segment_sizes: builtin.operand_segment_sizes [1, 0, 0]]: <(builtin.integer i1) -> ()> !8

              ^then():
                square = spirv.FMul value, value : <spirv.float 32> !9;
                cube = spirv.FMul square, value : <spirv.float 32> !10;
                spirv.Store value_ptr, cube !11;
                spirv.Branch ^selection_merge() !12

              ^selection_merge():
                spirv_pliron.merge !13
            } !14;
            next = spirv.IAdd index, count : <builtin.integer ui32> !15;
            spirv.Branch ^header(next) !16

          ^loop_merge():
            spirv_pliron.merge !17
        } !18;
        square_again = spirv.FMul value, value : <spirv.float 32> !19;
        spirv.Store value_ptr, square_again !20;
        unknown = spirv.FMul square_again, square_again : <spirv.float 32> !21;
        spirv.Store value_ptr, unknown !22;
        spirv.Return !23
    } !24;
    spirv.EntryPoint GLCompute, @main, "main";
    spirv.ExecutionMode @main, LocalSize, arguments = [1, 1, 1]
}

outlined_attributes:
!0 = @[name: "kernel", loc: ("src/kernel.rs": line: 2, column: 9)], []
!1 = @[name: "kernel", loc: ("src/kernel.rs": line: 3, column: 9)], []
!2 = @[name: "kernel", loc: ("src/kernel.rs": line: 4, column: 13)], []
!3 = @[name: "kernel", loc: ("src/kernel.rs": line: 5, column: 13)], []
!4 = @[name: "kernel", loc: ("src/kernel.rs": line: 6, column: 5)], []
!5 = @[name: "kernel", loc: ("src/kernel.rs": line: 6, column: 15)], []
!6 = @[name: "kernel", loc: ("src/kernel.rs": line: 6, column: 15)], []
!7 = @[name: "kernel", loc: ("src/kernel.rs": line: 7, column: 12)], []
!8 = @[name: "kernel", loc: ("src/kernel.rs": line: 7, column: 9)], []
!9 = @[callsite(name: "inner", loc: ("src/lib.rs": line: 3, column: 9) at callsite(name: "mid", loc: ("src/lib.rs": line: 7, column: 5) at name: "kernel", loc: ("src/kernel.rs": line: 8, column: 13)))], []
!10 = @[callsite(name: "mid", loc: ("src/lib.rs": line: 7, column: 5) at name: "kernel", loc: ("src/kernel.rs": line: 8, column: 13))], []
!11 = @[name: "kernel", loc: ("src/kernel.rs": line: 8, column: 13)], []
!12 = @[name: "kernel", loc: ("src/kernel.rs": line: 8, column: 13)], []
!13 = @[name: "kernel", loc: ("src/kernel.rs": line: 9, column: 9)], []
!14 = @[name: "kernel", loc: ("src/kernel.rs": line: 7, column: 9)], []
!15 = @[name: "kernel", loc: ("src/kernel.rs": line: 10, column: 9)], []
!16 = @[name: "kernel", loc: ("src/kernel.rs": line: 10, column: 9)], []
!17 = @[name: "kernel", loc: ("src/kernel.rs": line: 11, column: 5)], []
!18 = @[name: "kernel", loc: ("src/kernel.rs": line: 6, column: 5)], []
!19 = @[callsite(name: "inner", loc: ("src/lib.rs": line: 3, column: 9) at callsite(name: "mid", loc: ("src/lib.rs": line: 7, column: 5) at name: "kernel", loc: ("src/kernel.rs": line: 12, column: 13)))], []
!20 = @[name: "kernel", loc: ("src/kernel.rs": line: 12, column: 5)], []
!21 = @[?], []
!22 = @[?], []
!23 = @[name: "kernel", loc: ("src/kernel.rs": line: 14, column: 1)], []
!24 = @[name: "kernel", loc: ("src/kernel.rs": line: 1, column: 1)], []
"#;

/// Emits the test module for the SPIR-V version `version` with `builder`.
fn emit(mut builder: PlironBuilder, version: (u8, u8)) -> Module {
    let ctx = &mut Context::new();
    let op = parse_from_str(Operation::top_level_parser(), ctx, MODULE).unwrap_or_else(|err| panic!("{err}"));
    let module = Operation::get_op::<SpirvModuleOp>(op, ctx).expect("Should be a SPIR-V module");
    verify_op(&module, ctx).unwrap_or_else(|err| panic!("{}", err.disp(ctx)));
    let mut vce = module.get_vce(ctx);
    vce.version = version;
    module.set_vce(ctx, vce);
    module.to_spirv(ctx, &mut builder).unwrap();
    builder.module()
}

fn words_to_bytes(words: &[u32]) -> Vec<u8> {
    words.iter().flat_map(|word| word.to_le_bytes()).collect()
}

/// Without the option, the output is byte-identical to the output of the base commit.
#[test]
fn no_option_is_byte_identical_to_base() {
    let binary = words_to_bytes(&emit(PlironBuilder::new(), (1, 6)).assemble());
    if std::env::var_os("PLIRON_SPIRV_BLESS").is_some() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/debug_info_base.spv");
        std::fs::write(path, &binary).unwrap();
    }
    assert!(binary == BASE_BINARY, "The output without debug data changed");
    tools::validate(&binary, "vulkan1.3");
}

/// Runs the SPIRV-Tools programs, if they are installed.
///
/// The Linux and macOS CI jobs install SPIRV-Tools. Without it, the tests check the module structure
/// only.
mod tools {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };

    /// Runs `tool` with `args` on `binary`. Does nothing if `tool` is not installed.
    fn run(tool: &str, args: &[&str], binary: &[u8]) {
        let Ok(mut child) = Command::new(tool)
            .args(args)
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        else {
            eprintln!("{tool} is not installed; skipped");
            return;
        };
        child.stdin.take().unwrap().write_all(binary).unwrap();
        let output = child.wait_with_output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{tool} failed:\n{stderr}\n{stdout}");
    }

    /// Validates `binary` with `spirv-val` for the target environment `env`.
    pub(crate) fn validate(binary: &[u8], env: &str) {
        run("spirv-val", &["--target-env", env], binary);
    }
}

fn options(format: DebugInfoFormat) -> DebugInfoOptions {
    let mut options = DebugInfoOptions::default();
    options.format = format;
    options
}

/// Emits and validates the test module with `options` for `version` and the target environment
/// `env`.
fn emit_valid(options: DebugInfoOptions, version: (u8, u8), env: &str) -> Module {
    let module = emit(PlironBuilder::with_debug_info(options), version);
    let binary = words_to_bytes(&module.assemble());
    tools::validate(&binary, env);
    module
}

/// The instructions of the functions of `module`.
fn function_instructions(module: &Module) -> impl Iterator<Item = &Instruction> {
    module
        .functions
        .iter()
        .flat_map(|func| func.blocks.iter())
        .flat_map(|block| block.label.iter().chain(block.instructions.iter()))
}

/// All instructions of `module` that are `op` of `NonSemantic.Shader.DebugInfo.100`.
fn debug_instructions(module: &Module, op: DebugInfoOp) -> Vec<&Instruction> {
    module
        .types_global_values
        .iter()
        .chain(function_instructions(module))
        .filter(|inst| is_debug(inst, op))
        .collect()
}

fn is_debug(inst: &Instruction, op: DebugInfoOp) -> bool {
    inst.class.opcode == SpirvOp::ExtInst && inst.operands[1] == Operand::LiteralExtInstInteger(op as u32)
}

/// The `IdRef` operand `index` of the extended instruction `inst`, after the set and the opcode.
fn id_operand(inst: &Instruction, index: usize) -> Option<Word> {
    match inst.operands.get(index + 2) {
        Some(Operand::IdRef(id)) => Some(*id),
        _ => None,
    }
}

fn definition(module: &Module, id: Word) -> &Instruction {
    module
        .types_global_values
        .iter()
        .chain(module.debug_string_source.iter())
        .find(|inst| inst.result_id == Some(id))
        .unwrap_or_else(|| panic!("No definition of %{id}"))
}

fn string(module: &Module, id: Word) -> &str {
    match &definition(module, id).operands[0] {
        Operand::LiteralString(text) => text,
        operand => panic!("%{id} is not a string: {operand:?}"),
    }
}

fn constant(module: &Module, id: Word) -> u32 {
    match definition(module, id).operands[0] {
        Operand::LiteralBit32(value) => value,
        ref operand => panic!("%{id} is not a constant: {operand:?}"),
    }
}

/// The name of the `DebugFunction` `id`.
fn function_name(module: &Module, id: Word) -> &str {
    string(module, id_operand(definition(module, id), 0).unwrap())
}

fn has_extension(module: &Module, name: &str) -> bool {
    module
        .extensions
        .iter()
        .any(|inst| inst.operands == [Operand::LiteralString(name.to_string())])
}

/// The frames of a `DebugScope`, innermost first, as function names and lines.
fn scope_frames(module: &Module, scope: &Instruction) -> Vec<(String, Option<u32>)> {
    let mut frames = vec![(function_name(module, id_operand(scope, 0).unwrap()).to_string(), None)];
    let mut inlined = id_operand(scope, 1);
    while let Some(id) = inlined {
        let inlined_at = definition(module, id);
        assert!(is_debug(inlined_at, DebugInfoOp::DebugInlinedAt));
        let line = constant(module, id_operand(inlined_at, 0).unwrap());
        let name = function_name(module, id_operand(inlined_at, 1).unwrap());
        frames.push((name.to_string(), Some(line)));
        inlined = id_operand(inlined_at, 2);
    }
    frames
}

/// `NonSemantic` gives one `DebugFunction` for the function and one for each distinct callee, and
/// a `DebugInlinedAt` chain for each inlined op.
#[test]
fn non_semantic_frames() {
    let module = emit_valid(options(DebugInfoFormat::NonSemantic), (1, 6), "vulkan1.3");

    let mut names = debug_instructions(&module, DebugInfoOp::DebugFunction)
        .into_iter()
        .map(|inst| function_name(&module, inst.result_id.unwrap()))
        .collect::<Vec<_>>();
    names.sort_unstable();
    assert_eq!(names, ["inner", "kernel", "mid"]);

    let mut scopes = debug_instructions(&module, DebugInfoOp::DebugScope)
        .into_iter()
        .map(|scope| scope_frames(&module, scope))
        .collect::<Vec<_>>();
    scopes.sort();
    scopes.dedup();
    let frame = |name: &str, line: Option<u32>| (name.to_string(), line);
    assert_eq!(
        scopes,
        [
            vec![frame("inner", None), frame("mid", Some(7)), frame("kernel", Some(8))],
            vec![frame("inner", None), frame("mid", Some(7)), frame("kernel", Some(12))],
            vec![frame("kernel", None)],
            vec![frame("mid", None), frame("kernel", Some(8))],
        ]
    );

    assert_eq!(
        debug_instructions(&module, DebugInfoOp::DebugFunctionDefinition).len(),
        1
    );
    assert_eq!(debug_instructions(&module, DebugInfoOp::DebugCompilationUnit).len(), 1);
    // The ops without a location have no line.
    assert_eq!(debug_instructions(&module, DebugInfoOp::DebugNoLine).len(), 1);
    assert!(!function_instructions(&module).any(|inst| inst.class.opcode == SpirvOp::Line));
    assert!(!has_extension(&module, "SPV_KHR_non_semantic_info"));
}

/// The `DebugEntryPoint` of the entry point gets the producer and the arguments of the options.
/// The arguments are empty by default.
#[test]
fn non_semantic_entry_point() {
    let entry_point_strings = |options: DebugInfoOptions| {
        let module = emit_valid(options, (1, 6), "vulkan1.3");
        let entry_points = debug_instructions(&module, DebugInfoOp::DebugEntryPoint);
        let [entry_point] = entry_points.as_slice() else {
            panic!("Expected one DebugEntryPoint, got {}", entry_points.len());
        };
        let [signature, arguments] = [2, 3].map(|index| string(&module, id_operand(entry_point, index).unwrap()));
        (signature.to_string(), arguments.to_string())
    };

    let defaults = entry_point_strings(options(DebugInfoFormat::NonSemantic));
    assert_eq!(defaults, ("pliron-spirv".to_string(), String::new()));

    let mut options = options(DebugInfoFormat::NonSemantic);
    options.producer = "cubecl 0.9".to_string();
    options.arguments = "--opt-level 3".to_string();
    assert_eq!(
        entry_point_strings(options),
        ("cubecl 0.9".to_string(), "--opt-level 3".to_string())
    );
}

/// Below SPIR-V 1.6, `NonSemantic` adds the extension for non-semantic instruction sets.
#[test]
fn non_semantic_spirv_1_3_has_extension() {
    let module = emit_valid(options(DebugInfoFormat::NonSemantic), (1, 3), "vulkan1.1");
    assert!(has_extension(&module, "SPV_KHR_non_semantic_info"));
}

/// The `DebugSource` of a file gets the text of the options. A long text continues in
/// `DebugSourceContinued`. The directory of the options goes before relative file names.
#[test]
fn non_semantic_source_text_and_directory() {
    let mut options = options(DebugInfoFormat::NonSemantic);
    // `spirv-val` checks each column against the length of its line in the text.
    let text = format!("// {}\n", "kernel ".repeat(8)).repeat(5_000);
    options.source_text.insert("src/kernel.rs".to_string(), text.clone());
    options.directory = "/work/".to_string();
    let module = emit_valid(options, (1, 6), "vulkan1.3");

    let sources = debug_instructions(&module, DebugInfoOp::DebugSource);
    let mut files = sources
        .iter()
        .map(|inst| string(&module, id_operand(inst, 0).unwrap()))
        .collect::<Vec<_>>();
    files.sort_unstable();
    assert_eq!(files, ["/work/src/kernel.rs", "/work/src/lib.rs"]);

    let kernel = sources
        .iter()
        .find(|inst| string(&module, id_operand(inst, 0).unwrap()) == "/work/src/kernel.rs")
        .unwrap();
    let mut joined = string(&module, id_operand(kernel, 1).unwrap()).to_string();
    let globals = &module.types_global_values;
    let start = globals.iter().position(|inst| inst == *kernel).unwrap();
    for inst in globals[start + 1..]
        .iter()
        .take_while(|inst| is_debug(inst, DebugInfoOp::DebugSourceContinued))
    {
        joined.push_str(string(&module, id_operand(inst, 0).unwrap()));
    }
    assert_eq!(joined, text);
    assert_eq!(debug_instructions(&module, DebugInfoOp::DebugSourceContinued).len(), 1);
}

/// An instruction and the file and the line that `OpLine` gives it.
type InstructionLine<'a> = (SpirvOp, Option<(&'a str, u32)>);

/// The source line of each instruction of the functions, as `OpLine` gives it, by block.
fn instruction_lines(module: &Module) -> Vec<Vec<InstructionLine<'_>>> {
    let blocks = module.functions.iter().flat_map(|func| func.blocks.iter());
    blocks
        .map(|block| {
            let mut current = None;
            block
                .instructions
                .iter()
                .filter_map(|inst| match (inst.class.opcode, inst.operands.as_slice()) {
                    (SpirvOp::Line, [Operand::IdRef(file), Operand::LiteralBit32(line), ..]) => {
                        current = Some((string(module, *file), *line));
                        None
                    }
                    (SpirvOp::NoLine, _) => {
                        current = None;
                        None
                    }
                    (opcode, _) => Some((opcode, current)),
                })
                .collect()
        })
        .collect()
}

/// `OpLine` gives each instruction the innermost line of its op. Terminators and merge
/// instructions keep the line before them, and each block starts without a line.
#[test]
fn op_line_gives_innermost_lines() {
    let module = emit_valid(options(DebugInfoFormat::OpLine), (1, 6), "vulkan1.3");
    assert!(module.ext_inst_imports.is_empty());
    emit_valid(options(DebugInfoFormat::OpLine), (1, 3), "vulkan1.1");

    let kernel = |line| Some(("src/kernel.rs", line));
    let lib = |line| Some(("src/lib.rs", line));
    assert_eq!(
        instruction_lines(&module),
        [
            // The entry block, up to the loop.
            vec![
                (SpirvOp::Variable, kernel(2)),
                (SpirvOp::Variable, kernel(3)),
                (SpirvOp::Load, kernel(4)),
                (SpirvOp::Load, kernel(5)),
                (SpirvOp::Branch, kernel(6)),
            ],
            // The loop header. The `OpPhi` of the block argument goes before the first line.
            vec![
                (SpirvOp::Phi, None),
                (SpirvOp::ULessThan, kernel(6)),
                (SpirvOp::LoopMerge, kernel(6)),
                (SpirvOp::BranchConditional, kernel(6)),
            ],
            // The loop body, up to the branch.
            vec![
                (SpirvOp::INotEqual, kernel(7)),
                (SpirvOp::SelectionMerge, kernel(7)),
                (SpirvOp::BranchConditional, kernel(7)),
            ],
            // The inlined calls of `inner` and `mid`.
            vec![
                (SpirvOp::FMul, lib(3)),
                (SpirvOp::FMul, lib(7)),
                (SpirvOp::Store, kernel(8)),
                (SpirvOp::Branch, kernel(8)),
            ],
            // After the branch, the rest of the loop body.
            vec![(SpirvOp::IAdd, kernel(10)), (SpirvOp::Branch, kernel(10))],
            // After the loop. The ops without a location have no line.
            vec![
                (SpirvOp::FMul, lib(3)),
                (SpirvOp::Store, kernel(12)),
                (SpirvOp::FMul, None),
                (SpirvOp::Store, None),
                (SpirvOp::Return, None),
            ],
        ]
    );
}
