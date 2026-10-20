//! Tests for a loop header that contains a selection.
//!
//! A selection starts in the block that it is in, and a block has at most one merge instruction.
//! The header of such a loop keeps only its phis and its `OpLoopMerge`, and its ops start in the
//! next block.

use std::{
    io::Write,
    process::{Command, Stdio},
};

use pliron::{context::Context, op::verify_op, operation::Operation, parsable::parse_from_str, printable::Printable};
use pliron_spirv::{PlironBuilder, ToSpirvOp, ops::SpirvModuleOp};
use tracel_rspirv::{binary::Assemble, dr::Module, spirv::Op};

/// A loop whose header contains a selection:
///
/// ```text
/// let mut index = count;
/// while { if index != count { *count_ptr = index; } index < count } {
///     index = index + count;
/// }
/// ```
const MODULE: &str = r#"
spirv.module @module Logical GLSL450 requires : <v1.6, [Shader], []> {
  ^module_block():
    builtin.func @main: builtin.function <() -> (builtin.unit)> {
      ^entry():
        count_ptr = spirv.Variable Function : <spirv.ptr <builtin.integer ui32, Function>>;
        count = spirv.Load count_ptr : <builtin.integer ui32>;
        spirv_pliron.loop {
          ^loop_entry():
            spirv.Branch ^header(count)

          ^header(index: builtin.integer ui32):
            not_equal = spirv.INotEqual index, count : <builtin.integer i1>;
            spirv_pliron.selection {
              ^selection_entry():
                spirv.BranchConditional (not_equal) [^then, ^selection_merge] [builtin_operand_segment_sizes: builtin.operand_segment_sizes [1, 0, 0]]: <(builtin.integer i1) -> ()>

              ^then():
                spirv.Store count_ptr, index;
                spirv.Branch ^selection_merge()

              ^selection_merge():
                spirv_pliron.merge
            };
            less = spirv.ULessThan index, count : <builtin.integer i1>;
            spirv.BranchConditional (less) [^continue, ^loop_merge] [builtin_operand_segment_sizes: builtin.operand_segment_sizes [1, 0, 0]]: <(builtin.integer i1) -> ()>

          ^continue():
            next = spirv.IAdd index, count : <builtin.integer ui32>;
            spirv.Branch ^header(next)

          ^loop_merge():
            spirv_pliron.merge
        };
        spirv.Return
    };
    spirv.EntryPoint GLCompute, @main, "main";
    spirv.ExecutionMode @main, LocalSize, arguments = [1, 1, 1]
}
"#;

fn emit() -> Module {
    let ctx = &mut Context::new();
    let op = parse_from_str(Operation::top_level_parser(), ctx, MODULE).unwrap_or_else(|err| panic!("{err}"));
    let module = Operation::get_op::<SpirvModuleOp>(op, ctx).expect("Should be a SPIR-V module");
    verify_op(&module, ctx).unwrap_or_else(|err| panic!("{}", err.disp(ctx)));
    let mut builder = PlironBuilder::new();
    module.to_spirv(ctx, &mut builder).unwrap();
    builder.module()
}

/// The opcodes of each block of the function, in order.
fn block_opcodes(module: &Module) -> Vec<Vec<Op>> {
    module.functions[0]
        .blocks
        .iter()
        .map(|block| block.instructions.iter().map(|inst| inst.class.opcode).collect())
        .collect()
}

#[test]
fn header_with_selection_has_its_own_block() {
    let blocks = block_opcodes(&emit());
    let header = blocks
        .iter()
        .find(|block| block.contains(&Op::LoopMerge))
        .expect("Should have a loop header");
    assert_eq!(header, &[Op::Phi, Op::LoopMerge, Op::Branch]);
    for block in &blocks {
        let merges = block
            .iter()
            .filter(|op| matches!(op, Op::LoopMerge | Op::SelectionMerge))
            .count();
        assert!(merges <= 1, "A block has {merges} merge instructions: {block:?}");
    }
}

/// Validates the module with `spirv-val`, if it is installed.
#[test]
fn header_with_selection_is_valid() {
    let binary = emit()
        .assemble()
        .iter()
        .flat_map(|word| word.to_le_bytes())
        .collect::<Vec<_>>();
    let Ok(mut child) = Command::new("spirv-val")
        .args(["--target-env", "vulkan1.3", "-"])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    else {
        eprintln!("spirv-val is not installed; skipped");
        return;
    };
    child.stdin.take().unwrap().write_all(&binary).unwrap();
    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "spirv-val failed:\n{stderr}");
}
