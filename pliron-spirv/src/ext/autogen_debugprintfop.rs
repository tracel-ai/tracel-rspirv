// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#![allow(clippy::let_and_return, unused_imports)]
use crate::prelude::*;
use crate::attrs::*;
#[pliron_op(
    name = "spirv.DebugPrintfOp.debug_printf",
    operands = (opd_1),
    interfaces = [DecoratableOp],
    verifier = "succ"
)]
pub struct DebugPrintfOp;
crate::format::canonical_format!(
    DebugPrintfOp; crate ::format::attr!(& spirv_debugprintfop_debug_printf::ATTR_FORMAT,
    LiteralStringAttr, "format", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("opd_1", crate ::format::Quantifier::ZeroOrMore)
);
mod spirv_debugprintfop_debug_printf {
    pub static ATTR_FORMAT: ::pliron::std_deps::sync::LazyLock<
        ::pliron::identifier::Identifier,
    > = ::pliron::std_deps::sync::LazyLock::new(|| {
        "spirv_debugprintfop_debug_printf_format".try_into().unwrap()
    });
}
impl DebugPrintfOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, format: impl Into<String>, opd_1: Vec<Value>) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![],
                flat_vec![opd_1],
                vec![],
                0,
            ),
        };
        op.set_attr_format(ctx, LiteralStringAttr::new(format.into()));
        op
    }
    ///Get a [Ref](core::cell::Ref) to the value of the attribute named `format`.
    pub fn get_attr_format<'a>(
        &self,
        ctx: &'a ::pliron::context::Context,
    ) -> ::core::cell::Ref<'a, LiteralStringAttr> {
        ::core::cell::Ref::map(
            self.op.deref(ctx),
            |op| {
                op
                    .attributes
                    .get::<
                        LiteralStringAttr,
                    >(&spirv_debugprintfop_debug_printf::ATTR_FORMAT)
                    .unwrap()
            },
        )
    }
    ///Set the value of the attribute named `format`.
    pub fn set_attr_format(
        &self,
        ctx: &::pliron::context::Context,
        value: LiteralStringAttr,
    ) {
        self.op
            .deref_mut(ctx)
            .attributes
            .set(spirv_debugprintfop_debug_printf::ATTR_FORMAT.clone(), value);
    }
}
#[op_interface_impl]
impl ToSpirvOp for DebugPrintfOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let format = builder.string_ref(self.get_attr_format(ctx).clone());
        let opd_1 = op
            .operands()
            .skip(0usize)
            .map(|opd| builder.value_id(opd))
            .collect::<Vec<_>>();
        builder.debug_printf(format, opd_1).into_pliron_result()?;
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for DebugPrintfOp {
    #[allow(unused_variables)]
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        #[allow(unused_mut)]
        let mut result: (u8, u8) = None?;
        Some(result)
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
    }
}
