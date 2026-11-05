//! Tests for `spirv.FPFastMathDefault`, the default fast-math mode of a float type.
//!
//! The mode takes a type id and a 32-bit constant, so it needs an op of its own.

use std::{
    io::Write,
    process::{Command, Stdio},
};

use pliron::{
    builtin::op_interfaces::SingleBlockRegionInterface,
    context::Context,
    identifier::Identifier,
    op::{Op as _, verify_op},
    operation::Operation,
    parsable::parse_from_str,
    printable::Printable,
};
use pliron_spirv::{
    PlironBuilder,
    ToSpirvOp,
    interfaces::VerCapExtOpInterface,
    ops::{FPFastMathDefaultOp, SpirvModuleOp},
    types::FloatType,
};
use tracel_rspirv::{
    binary::Assemble,
    dr::{Module, Operand},
    spirv::{Capability, ExecutionMode, FPFastMathMode, Op},
};

const MODULE: &str = r#"
spirv.module @module Logical GLSL450 requires : <v1.6, [Shader, Float64, FloatControls2], [SPV_KHR_float_controls2]> {
  ^module_block():
    builtin.func @main: builtin.function <() -> (builtin.unit)> {
      ^entry():
        spirv.Return
    };
    spirv.EntryPoint GLCompute, @main, "main";
    spirv.ExecutionMode @main, LocalSize, arguments = [1, 1, 1]
}
"#;

fn module_with_default(ctx: &mut Context, flags: FPFastMathMode) -> SpirvModuleOp {
    let op = parse_from_str(Operation::top_level_parser(), ctx, MODULE).unwrap_or_else(|err| panic!("{err}"));
    let module = Operation::get_op::<SpirvModuleOp>(op, ctx).expect("Should be a SPIR-V module");
    let f64_ty = FloatType::get(ctx, 64, None).to_handle();
    let default = FPFastMathDefaultOp::new(ctx, Identifier::try_from("main").unwrap(), f64_ty, flags);
    let block = module.get_body(ctx, 0);
    default.get_operation().insert_at_back(block, ctx);
    verify_op(&module, ctx).unwrap();
    module
}

fn emit(flags: FPFastMathMode) -> Module {
    let ctx = &mut Context::new();
    let module = module_with_default(ctx, flags);
    let mut builder = PlironBuilder::new();
    module.to_spirv(ctx, &mut builder).unwrap();
    builder.module()
}

#[test]
fn emits_an_execution_mode_id_with_a_type_and_a_constant() {
    let flags = FPFastMathMode::ALLOW_CONTRACT | FPFastMathMode::NOT_NAN;
    let module = emit(flags);

    let mode = module
        .execution_modes
        .iter()
        .find(|inst| inst.operands.get(1) == Some(&Operand::ExecutionMode(ExecutionMode::FPFastMathDefault)))
        .expect("Should have the execution mode");
    assert_eq!(mode.class.opcode, Op::ExecutionModeId);
    // The builder keeps the ids of an `OpExecutionModeId` as plain words.
    let [
        Operand::IdRef(_),
        _,
        Operand::LiteralBit32(target),
        Operand::LiteralBit32(constant),
    ] = mode.operands[..]
    else {
        panic!("Unexpected operands: {:?}", mode.operands);
    };

    let defined = |id| {
        module
            .types_global_values
            .iter()
            .find(|inst| inst.result_id == Some(id))
            .unwrap()
    };
    assert_eq!(defined(target).class.opcode, Op::TypeFloat);
    assert_eq!(defined(target).operands, [Operand::LiteralBit32(64)]);
    let constant = defined(constant);
    assert_eq!(constant.class.opcode, Op::Constant);
    assert_eq!(constant.operands, [Operand::LiteralBit32(flags.bits())]);
    let int = defined(constant.result_type.unwrap());
    assert_eq!(int.operands, [Operand::LiteralBit32(32), Operand::LiteralBit32(0)]);
}

#[test]
fn requires_float_controls_2() {
    let ctx = &mut Context::new();
    let f64_ty = FloatType::get(ctx, 64, None).to_handle();
    let op = FPFastMathDefaultOp::new(ctx, Identifier::try_from("main").unwrap(), f64_ty, FPFastMathMode::NONE);
    assert_eq!(op.required_capabilities(ctx), [[Capability::FloatControls2]]);
    // The capability carries the extension, and the module collects it from there.
    assert_eq!(op.required_extensions(ctx), Vec::<Vec<&str>>::new());
}

#[test]
fn survives_printing_and_parsing() {
    let flags = FPFastMathMode::ALLOW_CONTRACT | FPFastMathMode::NSZ;
    let ctx = &mut Context::new();
    let module = module_with_default(ctx, flags);
    let printed = module.get_operation().disp(ctx).to_string();
    assert!(
        printed.contains("spirv.FPFastMathDefault @main, spirv.float 64, NSZ | ALLOW_CONTRACT"),
        "{printed}"
    );

    let reparsed_ctx = &mut Context::new();
    let reparsed = parse_from_str(Operation::top_level_parser(), reparsed_ctx, &printed)
        .unwrap_or_else(|err| panic!("{err}\n{printed}"));
    let reparsed = Operation::get_op::<SpirvModuleOp>(reparsed, reparsed_ctx).unwrap();
    verify_op(&reparsed, reparsed_ctx).unwrap();
    let mut builder = PlironBuilder::new();
    reparsed.to_spirv(reparsed_ctx, &mut builder).unwrap();
    assert_eq!(builder.module().assemble(), emit(flags).assemble());
}

/// Validates the module with `spirv-val`, if it is installed.
#[test]
fn is_valid() {
    let binary = emit(FPFastMathMode::NONE)
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
