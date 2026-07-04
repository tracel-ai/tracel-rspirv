// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#![allow(clippy::let_and_return, unused_imports)]
use crate::prelude::*;
use crate::attrs::*;
#[pliron_op(
    name = "spirv.DebugPrintfOp.debug_printf",
    format,
    operands = (format, opd_1),
    interfaces = [DecoratableOp],
    verifier = "succ"
)]
pub struct DebugPrintfOp;
mod spirv_debugprintfop_debug_printf {}
impl DebugPrintfOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, format: Value, opd_1: Vec<Value>) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![],
                flat_vec![format, opd_1],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for DebugPrintfOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let format = builder.value_id(self.get_operand_format(ctx));
        let opd_1 = op
            .operands()
            .skip(1usize)
            .map(|opd| builder.value_id(opd))
            .collect::<Vec<_>>();
        builder.debug_printf(format, opd_1).into_pliron_result()?;
        Ok(())
    }
}
