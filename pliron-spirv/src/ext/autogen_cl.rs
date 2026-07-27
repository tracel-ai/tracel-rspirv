// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#![allow(clippy::let_and_return, unused_imports)]
use crate::prelude::*;
use crate::attrs::*;
#[pliron_op(
    name = "spirv.CL_acos",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AcosOp;
crate::format::canonical_format!(
    AcosOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_acos {}
impl AcosOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for AcosOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_acos_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for AcosOp {
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
#[pliron_op(
    name = "spirv.CL_acosh",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AcoshOp;
crate::format::canonical_format!(
    AcoshOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_acosh {}
impl AcoshOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for AcoshOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_acosh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for AcoshOp {
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
#[pliron_op(
    name = "spirv.CL_acospi",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AcospiOp;
crate::format::canonical_format!(
    AcospiOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_acospi {}
impl AcospiOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for AcospiOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_acospi_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for AcospiOp {
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
#[pliron_op(
    name = "spirv.CL_asin",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AsinOp;
crate::format::canonical_format!(
    AsinOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_asin {}
impl AsinOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for AsinOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_asin_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for AsinOp {
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
#[pliron_op(
    name = "spirv.CL_asinh",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AsinhOp;
crate::format::canonical_format!(
    AsinhOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_asinh {}
impl AsinhOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for AsinhOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_asinh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for AsinhOp {
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
#[pliron_op(
    name = "spirv.CL_asinpi",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AsinpiOp;
crate::format::canonical_format!(
    AsinpiOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_asinpi {}
impl AsinpiOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for AsinpiOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_asinpi_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for AsinpiOp {
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
#[pliron_op(
    name = "spirv.CL_atan",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AtanOp;
crate::format::canonical_format!(
    AtanOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_atan {}
impl AtanOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for AtanOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_atan_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for AtanOp {
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
#[pliron_op(
    name = "spirv.CL_atan2",
    operands = (y, x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Atan2Op;
crate::format::canonical_format!(
    Atan2Op; crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_atan2 {}
impl Atan2Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, y: Value, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![y, x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for Atan2Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_atan2_id(result_ty, Some(result), y, x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for Atan2Op {
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
#[pliron_op(
    name = "spirv.CL_atanh",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AtanhOp;
crate::format::canonical_format!(
    AtanhOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_atanh {}
impl AtanhOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for AtanhOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_atanh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for AtanhOp {
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
#[pliron_op(
    name = "spirv.CL_atanpi",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AtanpiOp;
crate::format::canonical_format!(
    AtanpiOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_atanpi {}
impl AtanpiOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for AtanpiOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_atanpi_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for AtanpiOp {
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
#[pliron_op(
    name = "spirv.CL_atan2pi",
    operands = (y, x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Atan2piOp;
crate::format::canonical_format!(
    Atan2piOp; crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_atan2pi {}
impl Atan2piOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, y: Value, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![y, x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for Atan2piOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_atan2pi_id(result_ty, Some(result), y, x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for Atan2piOp {
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
#[pliron_op(
    name = "spirv.CL_cbrt",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CbrtOp;
crate::format::canonical_format!(
    CbrtOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_cbrt {}
impl CbrtOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for CbrtOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_cbrt_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for CbrtOp {
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
#[pliron_op(
    name = "spirv.CL_ceil",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CeilOp;
crate::format::canonical_format!(
    CeilOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_ceil {}
impl CeilOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for CeilOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_ceil_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for CeilOp {
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
#[pliron_op(
    name = "spirv.CL_copysign",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CopysignOp;
crate::format::canonical_format!(
    CopysignOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_copysign {}
impl CopysignOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for CopysignOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_copysign_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for CopysignOp {
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
#[pliron_op(
    name = "spirv.CL_cos",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CosOp;
crate::format::canonical_format!(
    CosOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_cos {}
impl CosOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for CosOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_cos_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for CosOp {
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
#[pliron_op(
    name = "spirv.CL_cosh",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CoshOp;
crate::format::canonical_format!(
    CoshOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_cosh {}
impl CoshOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for CoshOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_cosh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for CoshOp {
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
#[pliron_op(
    name = "spirv.CL_cospi",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CospiOp;
crate::format::canonical_format!(
    CospiOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_cospi {}
impl CospiOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for CospiOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_cospi_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for CospiOp {
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
#[pliron_op(
    name = "spirv.CL_erfc",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ErfcOp;
crate::format::canonical_format!(
    ErfcOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_erfc {}
impl ErfcOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for ErfcOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_erfc_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for ErfcOp {
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
#[pliron_op(
    name = "spirv.CL_erf",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ErfOp;
crate::format::canonical_format!(
    ErfOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_erf {}
impl ErfOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for ErfOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_erf_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for ErfOp {
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
#[pliron_op(
    name = "spirv.CL_exp",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ExpOp;
crate::format::canonical_format!(
    ExpOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_exp {}
impl ExpOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for ExpOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_exp_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for ExpOp {
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
#[pliron_op(
    name = "spirv.CL_exp2",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Exp2Op;
crate::format::canonical_format!(
    Exp2Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_exp2 {}
impl Exp2Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for Exp2Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_exp2_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for Exp2Op {
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
#[pliron_op(
    name = "spirv.CL_exp10",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Exp10Op;
crate::format::canonical_format!(
    Exp10Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_exp10 {}
impl Exp10Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for Exp10Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_exp10_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for Exp10Op {
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
#[pliron_op(
    name = "spirv.CL_expm1",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Expm1Op;
crate::format::canonical_format!(
    Expm1Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_expm1 {}
impl Expm1Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for Expm1Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_expm1_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for Expm1Op {
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
#[pliron_op(
    name = "spirv.CL_fabs",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FabsOp;
crate::format::canonical_format!(
    FabsOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_fabs {}
impl FabsOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FabsOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_fabs_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FabsOp {
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
#[pliron_op(
    name = "spirv.CL_fdim",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FdimOp;
crate::format::canonical_format!(
    FdimOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_fdim {}
impl FdimOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FdimOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_fdim_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FdimOp {
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
#[pliron_op(
    name = "spirv.CL_floor",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FloorOp;
crate::format::canonical_format!(
    FloorOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_floor {}
impl FloorOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FloorOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_floor_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FloorOp {
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
#[pliron_op(
    name = "spirv.CL_fma",
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FmaOp;
crate::format::canonical_format!(
    FmaOp; crate ::format::FormatVar::Value("a", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("b", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("c", crate ::format::Quantifier::One)
);
mod spirv_cl_fma {}
impl FmaOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        a: Value,
        b: Value,
        c: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![a, b, c],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FmaOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let a = builder.value_id(self.get_operand_a(ctx));
        let b = builder.value_id(self.get_operand_b(ctx));
        let c = builder.value_id(self.get_operand_c(ctx));
        builder.cl_fma_id(result_ty, Some(result), a, b, c).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FmaOp {
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
#[pliron_op(
    name = "spirv.CL_fmax",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FmaxOp;
crate::format::canonical_format!(
    FmaxOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_fmax {}
impl FmaxOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FmaxOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_fmax_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FmaxOp {
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
#[pliron_op(
    name = "spirv.CL_fmin",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FminOp;
crate::format::canonical_format!(
    FminOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_fmin {}
impl FminOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FminOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_fmin_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FminOp {
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
#[pliron_op(
    name = "spirv.CL_fmod",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FmodOp;
crate::format::canonical_format!(
    FmodOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_fmod {}
impl FmodOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FmodOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_fmod_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FmodOp {
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
#[pliron_op(
    name = "spirv.CL_fract",
    operands = (x, ptr),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FractOp;
crate::format::canonical_format!(
    FractOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("ptr", crate ::format::Quantifier::One)
);
mod spirv_cl_fract {}
impl FractOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, ptr: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, ptr],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FractOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let ptr = builder.value_id(self.get_operand_ptr(ctx));
        builder.cl_fract_id(result_ty, Some(result), x, ptr).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FractOp {
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
#[pliron_op(
    name = "spirv.CL_frexp",
    operands = (x, exp),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FrexpOp;
crate::format::canonical_format!(
    FrexpOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("exp", crate ::format::Quantifier::One)
);
mod spirv_cl_frexp {}
impl FrexpOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, exp: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, exp],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FrexpOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let exp = builder.value_id(self.get_operand_exp(ctx));
        builder.cl_frexp_id(result_ty, Some(result), x, exp).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FrexpOp {
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
#[pliron_op(
    name = "spirv.CL_hypot",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HypotOp;
crate::format::canonical_format!(
    HypotOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_hypot {}
impl HypotOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HypotOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_hypot_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HypotOp {
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
#[pliron_op(
    name = "spirv.CL_ilogb",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct IlogbOp;
crate::format::canonical_format!(
    IlogbOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_ilogb {}
impl IlogbOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for IlogbOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_ilogb_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for IlogbOp {
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
#[pliron_op(
    name = "spirv.CL_ldexp",
    operands = (x, k),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LdexpOp;
crate::format::canonical_format!(
    LdexpOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("k", crate ::format::Quantifier::One)
);
mod spirv_cl_ldexp {}
impl LdexpOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, k: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, k],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for LdexpOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let k = builder.value_id(self.get_operand_k(ctx));
        builder.cl_ldexp_id(result_ty, Some(result), x, k).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for LdexpOp {
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
#[pliron_op(
    name = "spirv.CL_lgamma",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LgammaOp;
crate::format::canonical_format!(
    LgammaOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_lgamma {}
impl LgammaOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for LgammaOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_lgamma_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for LgammaOp {
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
#[pliron_op(
    name = "spirv.CL_lgamma_r",
    operands = (x, signp),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LgammaROp;
crate::format::canonical_format!(
    LgammaROp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("signp", crate ::format::Quantifier::One)
);
mod spirv_cl_lgamma_r {}
impl LgammaROp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        signp: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, signp],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for LgammaROp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let signp = builder.value_id(self.get_operand_signp(ctx));
        builder.cl_lgamma_r_id(result_ty, Some(result), x, signp).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for LgammaROp {
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
#[pliron_op(
    name = "spirv.CL_log",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LogOp;
crate::format::canonical_format!(
    LogOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_log {}
impl LogOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for LogOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_log_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for LogOp {
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
#[pliron_op(
    name = "spirv.CL_log2",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Log2Op;
crate::format::canonical_format!(
    Log2Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_log2 {}
impl Log2Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for Log2Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_log2_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for Log2Op {
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
#[pliron_op(
    name = "spirv.CL_log10",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Log10Op;
crate::format::canonical_format!(
    Log10Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_log10 {}
impl Log10Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for Log10Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_log10_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for Log10Op {
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
#[pliron_op(
    name = "spirv.CL_log1p",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Log1pOp;
crate::format::canonical_format!(
    Log1pOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_log1p {}
impl Log1pOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for Log1pOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_log1p_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for Log1pOp {
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
#[pliron_op(
    name = "spirv.CL_logb",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LogbOp;
crate::format::canonical_format!(
    LogbOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_logb {}
impl LogbOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for LogbOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_logb_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for LogbOp {
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
#[pliron_op(
    name = "spirv.CL_mad",
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct MadOp;
crate::format::canonical_format!(
    MadOp; crate ::format::FormatVar::Value("a", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("b", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("c", crate ::format::Quantifier::One)
);
mod spirv_cl_mad {}
impl MadOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        a: Value,
        b: Value,
        c: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![a, b, c],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for MadOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let a = builder.value_id(self.get_operand_a(ctx));
        let b = builder.value_id(self.get_operand_b(ctx));
        let c = builder.value_id(self.get_operand_c(ctx));
        builder.cl_mad_id(result_ty, Some(result), a, b, c).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for MadOp {
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
#[pliron_op(
    name = "spirv.CL_maxmag",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct MaxmagOp;
crate::format::canonical_format!(
    MaxmagOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_maxmag {}
impl MaxmagOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for MaxmagOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_maxmag_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for MaxmagOp {
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
#[pliron_op(
    name = "spirv.CL_minmag",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct MinmagOp;
crate::format::canonical_format!(
    MinmagOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_minmag {}
impl MinmagOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for MinmagOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_minmag_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for MinmagOp {
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
#[pliron_op(
    name = "spirv.CL_modf",
    operands = (x, iptr),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ModfOp;
crate::format::canonical_format!(
    ModfOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("iptr", crate ::format::Quantifier::One)
);
mod spirv_cl_modf {}
impl ModfOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, iptr: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, iptr],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for ModfOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let iptr = builder.value_id(self.get_operand_iptr(ctx));
        builder.cl_modf_id(result_ty, Some(result), x, iptr).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for ModfOp {
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
#[pliron_op(
    name = "spirv.CL_nan",
    operands = (nancode),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NanOp;
crate::format::canonical_format!(
    NanOp; crate ::format::FormatVar::Value("nancode", crate ::format::Quantifier::One)
);
mod spirv_cl_nan {}
impl NanOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, nancode: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![nancode],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NanOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let nancode = builder.value_id(self.get_operand_nancode(ctx));
        builder.cl_nan_id(result_ty, Some(result), nancode).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NanOp {
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
#[pliron_op(
    name = "spirv.CL_nextafter",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NextafterOp;
crate::format::canonical_format!(
    NextafterOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_nextafter {}
impl NextafterOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NextafterOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_nextafter_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NextafterOp {
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
#[pliron_op(
    name = "spirv.CL_pow",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PowOp;
crate::format::canonical_format!(
    PowOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_pow {}
impl PowOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PowOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_pow_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for PowOp {
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
#[pliron_op(
    name = "spirv.CL_pown",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PownOp;
crate::format::canonical_format!(
    PownOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_pown {}
impl PownOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PownOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_pown_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for PownOp {
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
#[pliron_op(
    name = "spirv.CL_powr",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PowrOp;
crate::format::canonical_format!(
    PowrOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_powr {}
impl PowrOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PowrOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_powr_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for PowrOp {
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
#[pliron_op(
    name = "spirv.CL_remainder",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RemainderOp;
crate::format::canonical_format!(
    RemainderOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_remainder {}
impl RemainderOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for RemainderOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_remainder_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for RemainderOp {
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
#[pliron_op(
    name = "spirv.CL_remquo",
    operands = (x, y, quo),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RemquoOp;
crate::format::canonical_format!(
    RemquoOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("quo", crate ::format::Quantifier::One)
);
mod spirv_cl_remquo {}
impl RemquoOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        y: Value,
        quo: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y, quo],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for RemquoOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        let quo = builder.value_id(self.get_operand_quo(ctx));
        builder.cl_remquo_id(result_ty, Some(result), x, y, quo).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for RemquoOp {
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
#[pliron_op(
    name = "spirv.CL_rint",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RintOp;
crate::format::canonical_format!(
    RintOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_rint {}
impl RintOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for RintOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_rint_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for RintOp {
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
#[pliron_op(
    name = "spirv.CL_rootn",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RootnOp;
crate::format::canonical_format!(
    RootnOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_rootn {}
impl RootnOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for RootnOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_rootn_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for RootnOp {
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
#[pliron_op(
    name = "spirv.CL_round",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RoundOp;
crate::format::canonical_format!(
    RoundOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_round {}
impl RoundOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for RoundOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_round_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for RoundOp {
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
#[pliron_op(
    name = "spirv.CL_rsqrt",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RsqrtOp;
crate::format::canonical_format!(
    RsqrtOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_rsqrt {}
impl RsqrtOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for RsqrtOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_rsqrt_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for RsqrtOp {
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
#[pliron_op(
    name = "spirv.CL_sin",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SinOp;
crate::format::canonical_format!(
    SinOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_sin {}
impl SinOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SinOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_sin_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SinOp {
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
#[pliron_op(
    name = "spirv.CL_sincos",
    operands = (x, cosval),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SincosOp;
crate::format::canonical_format!(
    SincosOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("cosval", crate ::format::Quantifier::One)
);
mod spirv_cl_sincos {}
impl SincosOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        cosval: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, cosval],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SincosOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let cosval = builder.value_id(self.get_operand_cosval(ctx));
        builder.cl_sincos_id(result_ty, Some(result), x, cosval).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SincosOp {
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
#[pliron_op(
    name = "spirv.CL_sinh",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SinhOp;
crate::format::canonical_format!(
    SinhOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_sinh {}
impl SinhOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SinhOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_sinh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SinhOp {
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
#[pliron_op(
    name = "spirv.CL_sinpi",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SinpiOp;
crate::format::canonical_format!(
    SinpiOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_sinpi {}
impl SinpiOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SinpiOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_sinpi_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SinpiOp {
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
#[pliron_op(
    name = "spirv.CL_sqrt",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SqrtOp;
crate::format::canonical_format!(
    SqrtOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_sqrt {}
impl SqrtOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SqrtOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_sqrt_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SqrtOp {
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
#[pliron_op(
    name = "spirv.CL_tan",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TanOp;
crate::format::canonical_format!(
    TanOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_tan {}
impl TanOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for TanOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_tan_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for TanOp {
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
#[pliron_op(
    name = "spirv.CL_tanh",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TanhOp;
crate::format::canonical_format!(
    TanhOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_tanh {}
impl TanhOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for TanhOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_tanh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for TanhOp {
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
#[pliron_op(
    name = "spirv.CL_tanpi",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TanpiOp;
crate::format::canonical_format!(
    TanpiOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_tanpi {}
impl TanpiOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for TanpiOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_tanpi_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for TanpiOp {
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
#[pliron_op(
    name = "spirv.CL_tgamma",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TgammaOp;
crate::format::canonical_format!(
    TgammaOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_tgamma {}
impl TgammaOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for TgammaOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_tgamma_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for TgammaOp {
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
#[pliron_op(
    name = "spirv.CL_trunc",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TruncOp;
crate::format::canonical_format!(
    TruncOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_trunc {}
impl TruncOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for TruncOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_trunc_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for TruncOp {
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
#[pliron_op(
    name = "spirv.CL_half_cos",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfCosOp;
crate::format::canonical_format!(
    HalfCosOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_cos {}
impl HalfCosOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfCosOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_cos_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfCosOp {
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
#[pliron_op(
    name = "spirv.CL_half_divide",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfDivideOp;
crate::format::canonical_format!(
    HalfDivideOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_half_divide {}
impl HalfDivideOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfDivideOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_half_divide_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfDivideOp {
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
#[pliron_op(
    name = "spirv.CL_half_exp",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfExpOp;
crate::format::canonical_format!(
    HalfExpOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_exp {}
impl HalfExpOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfExpOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_exp_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfExpOp {
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
#[pliron_op(
    name = "spirv.CL_half_exp2",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfExp2Op;
crate::format::canonical_format!(
    HalfExp2Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_exp2 {}
impl HalfExp2Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfExp2Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_exp2_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfExp2Op {
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
#[pliron_op(
    name = "spirv.CL_half_exp10",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfExp10Op;
crate::format::canonical_format!(
    HalfExp10Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_exp10 {}
impl HalfExp10Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfExp10Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_exp10_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfExp10Op {
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
#[pliron_op(
    name = "spirv.CL_half_log",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfLogOp;
crate::format::canonical_format!(
    HalfLogOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_log {}
impl HalfLogOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfLogOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_log_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfLogOp {
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
#[pliron_op(
    name = "spirv.CL_half_log2",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfLog2Op;
crate::format::canonical_format!(
    HalfLog2Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_log2 {}
impl HalfLog2Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfLog2Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_log2_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfLog2Op {
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
#[pliron_op(
    name = "spirv.CL_half_log10",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfLog10Op;
crate::format::canonical_format!(
    HalfLog10Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_log10 {}
impl HalfLog10Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfLog10Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_log10_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfLog10Op {
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
#[pliron_op(
    name = "spirv.CL_half_powr",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfPowrOp;
crate::format::canonical_format!(
    HalfPowrOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_half_powr {}
impl HalfPowrOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfPowrOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_half_powr_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfPowrOp {
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
#[pliron_op(
    name = "spirv.CL_half_recip",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfRecipOp;
crate::format::canonical_format!(
    HalfRecipOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_recip {}
impl HalfRecipOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfRecipOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_recip_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfRecipOp {
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
#[pliron_op(
    name = "spirv.CL_half_rsqrt",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfRsqrtOp;
crate::format::canonical_format!(
    HalfRsqrtOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_rsqrt {}
impl HalfRsqrtOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfRsqrtOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_rsqrt_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfRsqrtOp {
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
#[pliron_op(
    name = "spirv.CL_half_sin",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfSinOp;
crate::format::canonical_format!(
    HalfSinOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_sin {}
impl HalfSinOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfSinOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_sin_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfSinOp {
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
#[pliron_op(
    name = "spirv.CL_half_sqrt",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfSqrtOp;
crate::format::canonical_format!(
    HalfSqrtOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_sqrt {}
impl HalfSqrtOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfSqrtOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_sqrt_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfSqrtOp {
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
#[pliron_op(
    name = "spirv.CL_half_tan",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfTanOp;
crate::format::canonical_format!(
    HalfTanOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_half_tan {}
impl HalfTanOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for HalfTanOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_half_tan_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for HalfTanOp {
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
#[pliron_op(
    name = "spirv.CL_native_cos",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeCosOp;
crate::format::canonical_format!(
    NativeCosOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_cos {}
impl NativeCosOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeCosOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_cos_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeCosOp {
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
#[pliron_op(
    name = "spirv.CL_native_divide",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeDivideOp;
crate::format::canonical_format!(
    NativeDivideOp; crate ::format::FormatVar::Value("x", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("y", crate
    ::format::Quantifier::One)
);
mod spirv_cl_native_divide {}
impl NativeDivideOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeDivideOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_native_divide_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeDivideOp {
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
#[pliron_op(
    name = "spirv.CL_native_exp",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeExpOp;
crate::format::canonical_format!(
    NativeExpOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_exp {}
impl NativeExpOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeExpOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_exp_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeExpOp {
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
#[pliron_op(
    name = "spirv.CL_native_exp2",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeExp2Op;
crate::format::canonical_format!(
    NativeExp2Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_exp2 {}
impl NativeExp2Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeExp2Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_exp2_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeExp2Op {
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
#[pliron_op(
    name = "spirv.CL_native_exp10",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeExp10Op;
crate::format::canonical_format!(
    NativeExp10Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_exp10 {}
impl NativeExp10Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeExp10Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_exp10_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeExp10Op {
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
#[pliron_op(
    name = "spirv.CL_native_log",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeLogOp;
crate::format::canonical_format!(
    NativeLogOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_log {}
impl NativeLogOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeLogOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_log_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeLogOp {
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
#[pliron_op(
    name = "spirv.CL_native_log2",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeLog2Op;
crate::format::canonical_format!(
    NativeLog2Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_log2 {}
impl NativeLog2Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeLog2Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_log2_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeLog2Op {
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
#[pliron_op(
    name = "spirv.CL_native_log10",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeLog10Op;
crate::format::canonical_format!(
    NativeLog10Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_log10 {}
impl NativeLog10Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeLog10Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_log10_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeLog10Op {
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
#[pliron_op(
    name = "spirv.CL_native_powr",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativePowrOp;
crate::format::canonical_format!(
    NativePowrOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_native_powr {}
impl NativePowrOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativePowrOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_native_powr_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativePowrOp {
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
#[pliron_op(
    name = "spirv.CL_native_recip",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeRecipOp;
crate::format::canonical_format!(
    NativeRecipOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_recip {}
impl NativeRecipOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeRecipOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_recip_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeRecipOp {
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
#[pliron_op(
    name = "spirv.CL_native_rsqrt",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeRsqrtOp;
crate::format::canonical_format!(
    NativeRsqrtOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_rsqrt {}
impl NativeRsqrtOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeRsqrtOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_rsqrt_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeRsqrtOp {
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
#[pliron_op(
    name = "spirv.CL_native_sin",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeSinOp;
crate::format::canonical_format!(
    NativeSinOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_sin {}
impl NativeSinOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeSinOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_sin_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeSinOp {
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
#[pliron_op(
    name = "spirv.CL_native_sqrt",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeSqrtOp;
crate::format::canonical_format!(
    NativeSqrtOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_sqrt {}
impl NativeSqrtOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeSqrtOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_sqrt_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeSqrtOp {
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
#[pliron_op(
    name = "spirv.CL_native_tan",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeTanOp;
crate::format::canonical_format!(
    NativeTanOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_native_tan {}
impl NativeTanOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NativeTanOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_native_tan_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NativeTanOp {
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
#[pliron_op(
    name = "spirv.CL_fclamp",
    operands = (x, minval, maxval),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FclampOp;
crate::format::canonical_format!(
    FclampOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("minval", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("maxval", crate ::format::Quantifier::One)
);
mod spirv_cl_fclamp {}
impl FclampOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        minval: Value,
        maxval: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, minval, maxval],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FclampOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let minval = builder.value_id(self.get_operand_minval(ctx));
        let maxval = builder.value_id(self.get_operand_maxval(ctx));
        builder
            .cl_fclamp_id(result_ty, Some(result), x, minval, maxval)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FclampOp {
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
#[pliron_op(
    name = "spirv.CL_degrees",
    operands = (radians),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct DegreesOp;
crate::format::canonical_format!(
    DegreesOp; crate ::format::FormatVar::Value("radians", crate
    ::format::Quantifier::One)
);
mod spirv_cl_degrees {}
impl DegreesOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, radians: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![radians],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for DegreesOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let radians = builder.value_id(self.get_operand_radians(ctx));
        builder.cl_degrees_id(result_ty, Some(result), radians).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for DegreesOp {
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
#[pliron_op(
    name = "spirv.CL_fmax_common",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FmaxCommonOp;
crate::format::canonical_format!(
    FmaxCommonOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_fmax_common {}
impl FmaxCommonOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FmaxCommonOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_fmax_common_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FmaxCommonOp {
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
#[pliron_op(
    name = "spirv.CL_fmin_common",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FminCommonOp;
crate::format::canonical_format!(
    FminCommonOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_fmin_common {}
impl FminCommonOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FminCommonOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_fmin_common_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FminCommonOp {
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
#[pliron_op(
    name = "spirv.CL_mix",
    operands = (x, y, a),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct MixOp;
crate::format::canonical_format!(
    MixOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("a", crate ::format::Quantifier::One)
);
mod spirv_cl_mix {}
impl MixOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        y: Value,
        a: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y, a],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for MixOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        let a = builder.value_id(self.get_operand_a(ctx));
        builder.cl_mix_id(result_ty, Some(result), x, y, a).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for MixOp {
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
#[pliron_op(
    name = "spirv.CL_radians",
    operands = (degrees),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RadiansOp;
crate::format::canonical_format!(
    RadiansOp; crate ::format::FormatVar::Value("degrees", crate
    ::format::Quantifier::One)
);
mod spirv_cl_radians {}
impl RadiansOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, degrees: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![degrees],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for RadiansOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let degrees = builder.value_id(self.get_operand_degrees(ctx));
        builder.cl_radians_id(result_ty, Some(result), degrees).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for RadiansOp {
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
#[pliron_op(
    name = "spirv.CL_step",
    operands = (edge, x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct StepOp;
crate::format::canonical_format!(
    StepOp; crate ::format::FormatVar::Value("edge", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_step {}
impl StepOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, edge: Value, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![edge, x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for StepOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let edge = builder.value_id(self.get_operand_edge(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_step_id(result_ty, Some(result), edge, x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for StepOp {
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
#[pliron_op(
    name = "spirv.CL_smoothstep",
    operands = (edge0, edge1, x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SmoothstepOp;
crate::format::canonical_format!(
    SmoothstepOp; crate ::format::FormatVar::Value("edge0", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("edge1", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("x", crate
    ::format::Quantifier::One)
);
mod spirv_cl_smoothstep {}
impl SmoothstepOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        edge0: Value,
        edge1: Value,
        x: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![edge0, edge1, x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SmoothstepOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let edge0 = builder.value_id(self.get_operand_edge0(ctx));
        let edge1 = builder.value_id(self.get_operand_edge1(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder
            .cl_smoothstep_id(result_ty, Some(result), edge0, edge1, x)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SmoothstepOp {
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
#[pliron_op(
    name = "spirv.CL_sign",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SignOp;
crate::format::canonical_format!(
    SignOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_sign {}
impl SignOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SignOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_sign_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SignOp {
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
#[pliron_op(
    name = "spirv.CL_cross",
    operands = (p0, p1),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CrossOp;
crate::format::canonical_format!(
    CrossOp; crate ::format::FormatVar::Value("p0", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("p1", crate ::format::Quantifier::One)
);
mod spirv_cl_cross {}
impl CrossOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, p0: Value, p1: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![p0, p1],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for CrossOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let p0 = builder.value_id(self.get_operand_p0(ctx));
        let p1 = builder.value_id(self.get_operand_p1(ctx));
        builder.cl_cross_id(result_ty, Some(result), p0, p1).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for CrossOp {
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
#[pliron_op(
    name = "spirv.CL_distance",
    operands = (p0, p1),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct DistanceOp;
crate::format::canonical_format!(
    DistanceOp; crate ::format::FormatVar::Value("p0", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("p1", crate ::format::Quantifier::One)
);
mod spirv_cl_distance {}
impl DistanceOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, p0: Value, p1: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![p0, p1],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for DistanceOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let p0 = builder.value_id(self.get_operand_p0(ctx));
        let p1 = builder.value_id(self.get_operand_p1(ctx));
        builder.cl_distance_id(result_ty, Some(result), p0, p1).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for DistanceOp {
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
#[pliron_op(
    name = "spirv.CL_length",
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LengthOp;
crate::format::canonical_format!(
    LengthOp; crate ::format::FormatVar::Value("p", crate ::format::Quantifier::One)
);
mod spirv_cl_length {}
impl LengthOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, p: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![p],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for LengthOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder.cl_length_id(result_ty, Some(result), p).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for LengthOp {
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
#[pliron_op(
    name = "spirv.CL_normalize",
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NormalizeOp;
crate::format::canonical_format!(
    NormalizeOp; crate ::format::FormatVar::Value("p", crate ::format::Quantifier::One)
);
mod spirv_cl_normalize {}
impl NormalizeOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, p: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![p],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NormalizeOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder.cl_normalize_id(result_ty, Some(result), p).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for NormalizeOp {
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
#[pliron_op(
    name = "spirv.CL_fast_distance",
    operands = (p0, p1),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FastDistanceOp;
crate::format::canonical_format!(
    FastDistanceOp; crate ::format::FormatVar::Value("p0", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("p1", crate
    ::format::Quantifier::One)
);
mod spirv_cl_fast_distance {}
impl FastDistanceOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, p0: Value, p1: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![p0, p1],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FastDistanceOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let p0 = builder.value_id(self.get_operand_p0(ctx));
        let p1 = builder.value_id(self.get_operand_p1(ctx));
        builder
            .cl_fast_distance_id(result_ty, Some(result), p0, p1)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FastDistanceOp {
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
#[pliron_op(
    name = "spirv.CL_fast_length",
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FastLengthOp;
crate::format::canonical_format!(
    FastLengthOp; crate ::format::FormatVar::Value("p", crate ::format::Quantifier::One)
);
mod spirv_cl_fast_length {}
impl FastLengthOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, p: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![p],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FastLengthOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder.cl_fast_length_id(result_ty, Some(result), p).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FastLengthOp {
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
#[pliron_op(
    name = "spirv.CL_fast_normalize",
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FastNormalizeOp;
crate::format::canonical_format!(
    FastNormalizeOp; crate ::format::FormatVar::Value("p", crate
    ::format::Quantifier::One)
);
mod spirv_cl_fast_normalize {}
impl FastNormalizeOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, p: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![p],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FastNormalizeOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder.cl_fast_normalize_id(result_ty, Some(result), p).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for FastNormalizeOp {
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
#[pliron_op(
    name = "spirv.CL_s_abs",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SAbsOp;
crate::format::canonical_format!(
    SAbsOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_s_abs {}
impl SAbsOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SAbsOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_s_abs_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SAbsOp {
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
#[pliron_op(
    name = "spirv.CL_s_abs_diff",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SAbsDiffOp;
crate::format::canonical_format!(
    SAbsDiffOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_s_abs_diff {}
impl SAbsDiffOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SAbsDiffOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_s_abs_diff_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SAbsDiffOp {
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
#[pliron_op(
    name = "spirv.CL_s_add_sat",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SAddSatOp;
crate::format::canonical_format!(
    SAddSatOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_s_add_sat {}
impl SAddSatOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SAddSatOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_s_add_sat_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SAddSatOp {
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
#[pliron_op(
    name = "spirv.CL_u_add_sat",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UAddSatOp;
crate::format::canonical_format!(
    UAddSatOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_u_add_sat {}
impl UAddSatOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UAddSatOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_u_add_sat_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UAddSatOp {
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
#[pliron_op(
    name = "spirv.CL_s_hadd",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SHaddOp;
crate::format::canonical_format!(
    SHaddOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_s_hadd {}
impl SHaddOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SHaddOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_s_hadd_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SHaddOp {
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
#[pliron_op(
    name = "spirv.CL_u_hadd",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UHaddOp;
crate::format::canonical_format!(
    UHaddOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_u_hadd {}
impl UHaddOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UHaddOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_u_hadd_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UHaddOp {
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
#[pliron_op(
    name = "spirv.CL_s_rhadd",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SRhaddOp;
crate::format::canonical_format!(
    SRhaddOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_s_rhadd {}
impl SRhaddOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SRhaddOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_s_rhadd_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SRhaddOp {
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
#[pliron_op(
    name = "spirv.CL_u_rhadd",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct URhaddOp;
crate::format::canonical_format!(
    URhaddOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_u_rhadd {}
impl URhaddOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for URhaddOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_u_rhadd_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for URhaddOp {
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
#[pliron_op(
    name = "spirv.CL_s_clamp",
    operands = (x, minval, maxval),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SClampOp;
crate::format::canonical_format!(
    SClampOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("minval", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("maxval", crate ::format::Quantifier::One)
);
mod spirv_cl_s_clamp {}
impl SClampOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        minval: Value,
        maxval: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, minval, maxval],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SClampOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let minval = builder.value_id(self.get_operand_minval(ctx));
        let maxval = builder.value_id(self.get_operand_maxval(ctx));
        builder
            .cl_s_clamp_id(result_ty, Some(result), x, minval, maxval)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SClampOp {
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
#[pliron_op(
    name = "spirv.CL_u_clamp",
    operands = (x, minval, maxval),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UClampOp;
crate::format::canonical_format!(
    UClampOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("minval", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("maxval", crate ::format::Quantifier::One)
);
mod spirv_cl_u_clamp {}
impl UClampOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        minval: Value,
        maxval: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, minval, maxval],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UClampOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let minval = builder.value_id(self.get_operand_minval(ctx));
        let maxval = builder.value_id(self.get_operand_maxval(ctx));
        builder
            .cl_u_clamp_id(result_ty, Some(result), x, minval, maxval)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UClampOp {
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
#[pliron_op(
    name = "spirv.CL_clz",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ClzOp;
crate::format::canonical_format!(
    ClzOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_clz {}
impl ClzOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for ClzOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_clz_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for ClzOp {
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
#[pliron_op(
    name = "spirv.CL_ctz",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CtzOp;
crate::format::canonical_format!(
    CtzOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_ctz {}
impl CtzOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for CtzOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_ctz_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for CtzOp {
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
#[pliron_op(
    name = "spirv.CL_s_mad_hi",
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMadHiOp;
crate::format::canonical_format!(
    SMadHiOp; crate ::format::FormatVar::Value("a", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("b", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("c", crate ::format::Quantifier::One)
);
mod spirv_cl_s_mad_hi {}
impl SMadHiOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        a: Value,
        b: Value,
        c: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![a, b, c],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SMadHiOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let a = builder.value_id(self.get_operand_a(ctx));
        let b = builder.value_id(self.get_operand_b(ctx));
        let c = builder.value_id(self.get_operand_c(ctx));
        builder.cl_s_mad_hi_id(result_ty, Some(result), a, b, c).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SMadHiOp {
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
#[pliron_op(
    name = "spirv.CL_u_mad_sat",
    operands = (x, y, z),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMadSatOp;
crate::format::canonical_format!(
    UMadSatOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("z", crate ::format::Quantifier::One)
);
mod spirv_cl_u_mad_sat {}
impl UMadSatOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        y: Value,
        z: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y, z],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UMadSatOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        let z = builder.value_id(self.get_operand_z(ctx));
        builder.cl_u_mad_sat_id(result_ty, Some(result), x, y, z).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UMadSatOp {
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
#[pliron_op(
    name = "spirv.CL_s_mad_sat",
    operands = (x, y, z),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMadSatOp;
crate::format::canonical_format!(
    SMadSatOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("z", crate ::format::Quantifier::One)
);
mod spirv_cl_s_mad_sat {}
impl SMadSatOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        y: Value,
        z: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y, z],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SMadSatOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        let z = builder.value_id(self.get_operand_z(ctx));
        builder.cl_s_mad_sat_id(result_ty, Some(result), x, y, z).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SMadSatOp {
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
#[pliron_op(
    name = "spirv.CL_s_max",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMaxOp;
crate::format::canonical_format!(
    SMaxOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_s_max {}
impl SMaxOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SMaxOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_s_max_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SMaxOp {
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
#[pliron_op(
    name = "spirv.CL_u_max",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMaxOp;
crate::format::canonical_format!(
    UMaxOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_u_max {}
impl UMaxOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UMaxOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_u_max_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UMaxOp {
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
#[pliron_op(
    name = "spirv.CL_s_min",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMinOp;
crate::format::canonical_format!(
    SMinOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_s_min {}
impl SMinOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SMinOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_s_min_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SMinOp {
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
#[pliron_op(
    name = "spirv.CL_u_min",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMinOp;
crate::format::canonical_format!(
    UMinOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_u_min {}
impl UMinOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UMinOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_u_min_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UMinOp {
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
#[pliron_op(
    name = "spirv.CL_s_mul_hi",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMulHiOp;
crate::format::canonical_format!(
    SMulHiOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_s_mul_hi {}
impl SMulHiOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SMulHiOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_s_mul_hi_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SMulHiOp {
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
#[pliron_op(
    name = "spirv.CL_rotate",
    operands = (v, i),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RotateOp;
crate::format::canonical_format!(
    RotateOp; crate ::format::FormatVar::Value("v", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("i", crate ::format::Quantifier::One)
);
mod spirv_cl_rotate {}
impl RotateOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, v: Value, i: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![v, i],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for RotateOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let v = builder.value_id(self.get_operand_v(ctx));
        let i = builder.value_id(self.get_operand_i(ctx));
        builder.cl_rotate_id(result_ty, Some(result), v, i).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for RotateOp {
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
#[pliron_op(
    name = "spirv.CL_s_sub_sat",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SSubSatOp;
crate::format::canonical_format!(
    SSubSatOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_s_sub_sat {}
impl SSubSatOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SSubSatOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_s_sub_sat_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SSubSatOp {
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
#[pliron_op(
    name = "spirv.CL_u_sub_sat",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct USubSatOp;
crate::format::canonical_format!(
    USubSatOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_u_sub_sat {}
impl USubSatOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for USubSatOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_u_sub_sat_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for USubSatOp {
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
#[pliron_op(
    name = "spirv.CL_u_upsample",
    operands = (hi, lo),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UUpsampleOp;
crate::format::canonical_format!(
    UUpsampleOp; crate ::format::FormatVar::Value("hi", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("lo", crate ::format::Quantifier::One)
);
mod spirv_cl_u_upsample {}
impl UUpsampleOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, hi: Value, lo: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![hi, lo],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UUpsampleOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let hi = builder.value_id(self.get_operand_hi(ctx));
        let lo = builder.value_id(self.get_operand_lo(ctx));
        builder.cl_u_upsample_id(result_ty, Some(result), hi, lo).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UUpsampleOp {
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
#[pliron_op(
    name = "spirv.CL_s_upsample",
    operands = (hi, lo),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SUpsampleOp;
crate::format::canonical_format!(
    SUpsampleOp; crate ::format::FormatVar::Value("hi", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("lo", crate ::format::Quantifier::One)
);
mod spirv_cl_s_upsample {}
impl SUpsampleOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, hi: Value, lo: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![hi, lo],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SUpsampleOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let hi = builder.value_id(self.get_operand_hi(ctx));
        let lo = builder.value_id(self.get_operand_lo(ctx));
        builder.cl_s_upsample_id(result_ty, Some(result), hi, lo).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SUpsampleOp {
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
#[pliron_op(
    name = "spirv.CL_popcount",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PopcountOp;
crate::format::canonical_format!(
    PopcountOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_popcount {}
impl PopcountOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PopcountOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_popcount_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for PopcountOp {
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
#[pliron_op(
    name = "spirv.CL_s_mad24",
    operands = (x, y, z),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMad24Op;
crate::format::canonical_format!(
    SMad24Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("z", crate ::format::Quantifier::One)
);
mod spirv_cl_s_mad24 {}
impl SMad24Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        y: Value,
        z: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y, z],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SMad24Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        let z = builder.value_id(self.get_operand_z(ctx));
        builder.cl_s_mad24_id(result_ty, Some(result), x, y, z).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SMad24Op {
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
#[pliron_op(
    name = "spirv.CL_u_mad24",
    operands = (x, y, z),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMad24Op;
crate::format::canonical_format!(
    UMad24Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("z", crate ::format::Quantifier::One)
);
mod spirv_cl_u_mad24 {}
impl UMad24Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        y: Value,
        z: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y, z],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UMad24Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        let z = builder.value_id(self.get_operand_z(ctx));
        builder.cl_u_mad24_id(result_ty, Some(result), x, y, z).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UMad24Op {
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
#[pliron_op(
    name = "spirv.CL_s_mul24",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMul24Op;
crate::format::canonical_format!(
    SMul24Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_s_mul24 {}
impl SMul24Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SMul24Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_s_mul24_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SMul24Op {
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
#[pliron_op(
    name = "spirv.CL_u_mul24",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMul24Op;
crate::format::canonical_format!(
    UMul24Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_u_mul24 {}
impl UMul24Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UMul24Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_u_mul24_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UMul24Op {
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
#[pliron_op(
    name = "spirv.CL_vloadn",
    operands = (offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VloadnOp;
crate::format::canonical_format!(
    VloadnOp; crate ::format::FormatVar::Value("offset", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("p", crate
    ::format::Quantifier::One), crate ::format::attr!(& spirv_cl_vloadn::ATTR_N,
    LiteralIntegerAttr, "n", crate ::format::Quantifier::One)
);
mod spirv_cl_vloadn {
    pub static ATTR_N: ::pliron::std_deps::sync::LazyLock<
        ::pliron::identifier::Identifier,
    > = ::pliron::std_deps::sync::LazyLock::new(|| {
        "spirv_cl_vloadn_n".try_into().unwrap()
    });
}
impl VloadnOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        offset: Value,
        p: Value,
        n: impl Into<LiteralIntegerAttr>,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![offset, p],
                vec![],
                0,
            ),
        };
        op.set_attr_n(ctx, n.into());
        op
    }
    ///Get a [Ref](core::cell::Ref) to the value of the attribute named `n`.
    pub fn get_attr_n<'a>(
        &self,
        ctx: &'a ::pliron::context::Context,
    ) -> ::core::cell::Ref<'a, LiteralIntegerAttr> {
        ::core::cell::Ref::map(
            self.op.deref(ctx),
            |op| {
                op
                    .attributes
                    .get::<LiteralIntegerAttr>(&spirv_cl_vloadn::ATTR_N)
                    .unwrap()
            },
        )
    }
    ///Set the value of the attribute named `n`.
    pub fn set_attr_n(
        &self,
        ctx: &::pliron::context::Context,
        value: LiteralIntegerAttr,
    ) {
        self.op.deref_mut(ctx).attributes.set(spirv_cl_vloadn::ATTR_N.clone(), value);
    }
}
#[op_interface_impl]
impl ToSpirvOp for VloadnOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        let n = self.get_attr_n(ctx).clone().0;
        builder
            .cl_vloadn_id(result_ty, Some(result), offset, p, n)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for VloadnOp {
    #[allow(unused_variables)]
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        #[allow(unused_mut)]
        let mut result: (u8, u8) = None?;
        result = result
            .max(Operand::from(self.get_attr_n(ctx).clone().0).minimum_version()?);
        Some(result)
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(Operand::from(self.get_attr_n(ctx).clone().0).required_extensions());
        result
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(
                Operand::from(self.get_attr_n(ctx).clone().0).required_capabilities(),
            );
        result
    }
}
#[pliron_op(
    name = "spirv.CL_vstoren",
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstorenOp;
crate::format::canonical_format!(
    VstorenOp; crate ::format::FormatVar::Value("data", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("offset", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("p", crate ::format::Quantifier::One)
);
mod spirv_cl_vstoren {}
impl VstorenOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        data: Value,
        offset: Value,
        p: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![data, offset, p],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for VstorenOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let data = builder.value_id(self.get_operand_data(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder
            .cl_vstoren_id(result_ty, Some(result), data, offset, p)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for VstorenOp {
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
#[pliron_op(
    name = "spirv.CL_vload_half",
    operands = (offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VloadHalfOp;
crate::format::canonical_format!(
    VloadHalfOp; crate ::format::FormatVar::Value("offset", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("p", crate
    ::format::Quantifier::One)
);
mod spirv_cl_vload_half {}
impl VloadHalfOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        offset: Value,
        p: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![offset, p],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for VloadHalfOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder
            .cl_vload_half_id(result_ty, Some(result), offset, p)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for VloadHalfOp {
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
#[pliron_op(
    name = "spirv.CL_vload_halfn",
    operands = (offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VloadHalfnOp;
crate::format::canonical_format!(
    VloadHalfnOp; crate ::format::FormatVar::Value("offset", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("p", crate
    ::format::Quantifier::One), crate ::format::attr!(& spirv_cl_vload_halfn::ATTR_N,
    LiteralIntegerAttr, "n", crate ::format::Quantifier::One)
);
mod spirv_cl_vload_halfn {
    pub static ATTR_N: ::pliron::std_deps::sync::LazyLock<
        ::pliron::identifier::Identifier,
    > = ::pliron::std_deps::sync::LazyLock::new(|| {
        "spirv_cl_vload_halfn_n".try_into().unwrap()
    });
}
impl VloadHalfnOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        offset: Value,
        p: Value,
        n: impl Into<LiteralIntegerAttr>,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![offset, p],
                vec![],
                0,
            ),
        };
        op.set_attr_n(ctx, n.into());
        op
    }
    ///Get a [Ref](core::cell::Ref) to the value of the attribute named `n`.
    pub fn get_attr_n<'a>(
        &self,
        ctx: &'a ::pliron::context::Context,
    ) -> ::core::cell::Ref<'a, LiteralIntegerAttr> {
        ::core::cell::Ref::map(
            self.op.deref(ctx),
            |op| {
                op
                    .attributes
                    .get::<LiteralIntegerAttr>(&spirv_cl_vload_halfn::ATTR_N)
                    .unwrap()
            },
        )
    }
    ///Set the value of the attribute named `n`.
    pub fn set_attr_n(
        &self,
        ctx: &::pliron::context::Context,
        value: LiteralIntegerAttr,
    ) {
        self.op
            .deref_mut(ctx)
            .attributes
            .set(spirv_cl_vload_halfn::ATTR_N.clone(), value);
    }
}
#[op_interface_impl]
impl ToSpirvOp for VloadHalfnOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        let n = self.get_attr_n(ctx).clone().0;
        builder
            .cl_vload_halfn_id(result_ty, Some(result), offset, p, n)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for VloadHalfnOp {
    #[allow(unused_variables)]
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        #[allow(unused_mut)]
        let mut result: (u8, u8) = None?;
        result = result
            .max(Operand::from(self.get_attr_n(ctx).clone().0).minimum_version()?);
        Some(result)
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(Operand::from(self.get_attr_n(ctx).clone().0).required_extensions());
        result
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(
                Operand::from(self.get_attr_n(ctx).clone().0).required_capabilities(),
            );
        result
    }
}
#[pliron_op(
    name = "spirv.CL_vstore_half",
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreHalfOp;
crate::format::canonical_format!(
    VstoreHalfOp; crate ::format::FormatVar::Value("data", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("offset", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("p", crate
    ::format::Quantifier::One)
);
mod spirv_cl_vstore_half {}
impl VstoreHalfOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        data: Value,
        offset: Value,
        p: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![data, offset, p],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for VstoreHalfOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let data = builder.value_id(self.get_operand_data(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder
            .cl_vstore_half_id(result_ty, Some(result), data, offset, p)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for VstoreHalfOp {
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
#[pliron_op(
    name = "spirv.CL_vstore_half_r",
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreHalfROp;
crate::format::canonical_format!(
    VstoreHalfROp; crate ::format::FormatVar::Value("data", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("offset", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("p", crate
    ::format::Quantifier::One), crate ::format::attr!(&
    spirv_cl_vstore_half_r::ATTR_MODE, FPRoundingModeAttr, "mode", crate
    ::format::Quantifier::One)
);
mod spirv_cl_vstore_half_r {
    pub static ATTR_MODE: ::pliron::std_deps::sync::LazyLock<
        ::pliron::identifier::Identifier,
    > = ::pliron::std_deps::sync::LazyLock::new(|| {
        "spirv_cl_vstore_half_r_mode".try_into().unwrap()
    });
}
impl VstoreHalfROp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        data: Value,
        offset: Value,
        p: Value,
        mode: impl Into<FPRoundingModeAttr>,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![data, offset, p],
                vec![],
                0,
            ),
        };
        op.set_attr_mode(ctx, mode.into());
        op
    }
    ///Get a [Ref](core::cell::Ref) to the value of the attribute named `mode`.
    pub fn get_attr_mode<'a>(
        &self,
        ctx: &'a ::pliron::context::Context,
    ) -> ::core::cell::Ref<'a, FPRoundingModeAttr> {
        ::core::cell::Ref::map(
            self.op.deref(ctx),
            |op| {
                op
                    .attributes
                    .get::<FPRoundingModeAttr>(&spirv_cl_vstore_half_r::ATTR_MODE)
                    .unwrap()
            },
        )
    }
    ///Set the value of the attribute named `mode`.
    pub fn set_attr_mode(
        &self,
        ctx: &::pliron::context::Context,
        value: FPRoundingModeAttr,
    ) {
        self.op
            .deref_mut(ctx)
            .attributes
            .set(spirv_cl_vstore_half_r::ATTR_MODE.clone(), value);
    }
}
#[op_interface_impl]
impl ToSpirvOp for VstoreHalfROp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let data = builder.value_id(self.get_operand_data(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        let mode = self.get_attr_mode(ctx).clone().0;
        builder
            .cl_vstore_half_r_id(result_ty, Some(result), data, offset, p, mode)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for VstoreHalfROp {
    #[allow(unused_variables)]
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        #[allow(unused_mut)]
        let mut result: (u8, u8) = None?;
        result = result
            .max(Operand::from(self.get_attr_mode(ctx).clone().0).minimum_version()?);
        Some(result)
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(
                Operand::from(self.get_attr_mode(ctx).clone().0).required_extensions(),
            );
        result
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(
                Operand::from(self.get_attr_mode(ctx).clone().0).required_capabilities(),
            );
        result
    }
}
#[pliron_op(
    name = "spirv.CL_vstore_halfn",
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreHalfnOp;
crate::format::canonical_format!(
    VstoreHalfnOp; crate ::format::FormatVar::Value("data", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("offset", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("p", crate
    ::format::Quantifier::One)
);
mod spirv_cl_vstore_halfn {}
impl VstoreHalfnOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        data: Value,
        offset: Value,
        p: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![data, offset, p],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for VstoreHalfnOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let data = builder.value_id(self.get_operand_data(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder
            .cl_vstore_halfn_id(result_ty, Some(result), data, offset, p)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for VstoreHalfnOp {
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
#[pliron_op(
    name = "spirv.CL_vstore_halfn_r",
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreHalfnROp;
crate::format::canonical_format!(
    VstoreHalfnROp; crate ::format::FormatVar::Value("data", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("offset", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("p", crate
    ::format::Quantifier::One), crate ::format::attr!(&
    spirv_cl_vstore_halfn_r::ATTR_MODE, FPRoundingModeAttr, "mode", crate
    ::format::Quantifier::One)
);
mod spirv_cl_vstore_halfn_r {
    pub static ATTR_MODE: ::pliron::std_deps::sync::LazyLock<
        ::pliron::identifier::Identifier,
    > = ::pliron::std_deps::sync::LazyLock::new(|| {
        "spirv_cl_vstore_halfn_r_mode".try_into().unwrap()
    });
}
impl VstoreHalfnROp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        data: Value,
        offset: Value,
        p: Value,
        mode: impl Into<FPRoundingModeAttr>,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![data, offset, p],
                vec![],
                0,
            ),
        };
        op.set_attr_mode(ctx, mode.into());
        op
    }
    ///Get a [Ref](core::cell::Ref) to the value of the attribute named `mode`.
    pub fn get_attr_mode<'a>(
        &self,
        ctx: &'a ::pliron::context::Context,
    ) -> ::core::cell::Ref<'a, FPRoundingModeAttr> {
        ::core::cell::Ref::map(
            self.op.deref(ctx),
            |op| {
                op
                    .attributes
                    .get::<FPRoundingModeAttr>(&spirv_cl_vstore_halfn_r::ATTR_MODE)
                    .unwrap()
            },
        )
    }
    ///Set the value of the attribute named `mode`.
    pub fn set_attr_mode(
        &self,
        ctx: &::pliron::context::Context,
        value: FPRoundingModeAttr,
    ) {
        self.op
            .deref_mut(ctx)
            .attributes
            .set(spirv_cl_vstore_halfn_r::ATTR_MODE.clone(), value);
    }
}
#[op_interface_impl]
impl ToSpirvOp for VstoreHalfnROp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let data = builder.value_id(self.get_operand_data(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        let mode = self.get_attr_mode(ctx).clone().0;
        builder
            .cl_vstore_halfn_r_id(result_ty, Some(result), data, offset, p, mode)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for VstoreHalfnROp {
    #[allow(unused_variables)]
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        #[allow(unused_mut)]
        let mut result: (u8, u8) = None?;
        result = result
            .max(Operand::from(self.get_attr_mode(ctx).clone().0).minimum_version()?);
        Some(result)
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(
                Operand::from(self.get_attr_mode(ctx).clone().0).required_extensions(),
            );
        result
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(
                Operand::from(self.get_attr_mode(ctx).clone().0).required_capabilities(),
            );
        result
    }
}
#[pliron_op(
    name = "spirv.CL_vloada_halfn",
    operands = (offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VloadaHalfnOp;
crate::format::canonical_format!(
    VloadaHalfnOp; crate ::format::FormatVar::Value("offset", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("p", crate
    ::format::Quantifier::One), crate ::format::attr!(& spirv_cl_vloada_halfn::ATTR_N,
    LiteralIntegerAttr, "n", crate ::format::Quantifier::One)
);
mod spirv_cl_vloada_halfn {
    pub static ATTR_N: ::pliron::std_deps::sync::LazyLock<
        ::pliron::identifier::Identifier,
    > = ::pliron::std_deps::sync::LazyLock::new(|| {
        "spirv_cl_vloada_halfn_n".try_into().unwrap()
    });
}
impl VloadaHalfnOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        offset: Value,
        p: Value,
        n: impl Into<LiteralIntegerAttr>,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![offset, p],
                vec![],
                0,
            ),
        };
        op.set_attr_n(ctx, n.into());
        op
    }
    ///Get a [Ref](core::cell::Ref) to the value of the attribute named `n`.
    pub fn get_attr_n<'a>(
        &self,
        ctx: &'a ::pliron::context::Context,
    ) -> ::core::cell::Ref<'a, LiteralIntegerAttr> {
        ::core::cell::Ref::map(
            self.op.deref(ctx),
            |op| {
                op
                    .attributes
                    .get::<LiteralIntegerAttr>(&spirv_cl_vloada_halfn::ATTR_N)
                    .unwrap()
            },
        )
    }
    ///Set the value of the attribute named `n`.
    pub fn set_attr_n(
        &self,
        ctx: &::pliron::context::Context,
        value: LiteralIntegerAttr,
    ) {
        self.op
            .deref_mut(ctx)
            .attributes
            .set(spirv_cl_vloada_halfn::ATTR_N.clone(), value);
    }
}
#[op_interface_impl]
impl ToSpirvOp for VloadaHalfnOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        let n = self.get_attr_n(ctx).clone().0;
        builder
            .cl_vloada_halfn_id(result_ty, Some(result), offset, p, n)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for VloadaHalfnOp {
    #[allow(unused_variables)]
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        #[allow(unused_mut)]
        let mut result: (u8, u8) = None?;
        result = result
            .max(Operand::from(self.get_attr_n(ctx).clone().0).minimum_version()?);
        Some(result)
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(Operand::from(self.get_attr_n(ctx).clone().0).required_extensions());
        result
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(
                Operand::from(self.get_attr_n(ctx).clone().0).required_capabilities(),
            );
        result
    }
}
#[pliron_op(
    name = "spirv.CL_vstorea_halfn",
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreaHalfnOp;
crate::format::canonical_format!(
    VstoreaHalfnOp; crate ::format::FormatVar::Value("data", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("offset", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("p", crate
    ::format::Quantifier::One)
);
mod spirv_cl_vstorea_halfn {}
impl VstoreaHalfnOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        data: Value,
        offset: Value,
        p: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![data, offset, p],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for VstoreaHalfnOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let data = builder.value_id(self.get_operand_data(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder
            .cl_vstorea_halfn_id(result_ty, Some(result), data, offset, p)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for VstoreaHalfnOp {
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
#[pliron_op(
    name = "spirv.CL_vstorea_halfn_r",
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreaHalfnROp;
crate::format::canonical_format!(
    VstoreaHalfnROp; crate ::format::FormatVar::Value("data", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("offset", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("p", crate
    ::format::Quantifier::One), crate ::format::attr!(&
    spirv_cl_vstorea_halfn_r::ATTR_MODE, FPRoundingModeAttr, "mode", crate
    ::format::Quantifier::One)
);
mod spirv_cl_vstorea_halfn_r {
    pub static ATTR_MODE: ::pliron::std_deps::sync::LazyLock<
        ::pliron::identifier::Identifier,
    > = ::pliron::std_deps::sync::LazyLock::new(|| {
        "spirv_cl_vstorea_halfn_r_mode".try_into().unwrap()
    });
}
impl VstoreaHalfnROp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        data: Value,
        offset: Value,
        p: Value,
        mode: impl Into<FPRoundingModeAttr>,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![data, offset, p],
                vec![],
                0,
            ),
        };
        op.set_attr_mode(ctx, mode.into());
        op
    }
    ///Get a [Ref](core::cell::Ref) to the value of the attribute named `mode`.
    pub fn get_attr_mode<'a>(
        &self,
        ctx: &'a ::pliron::context::Context,
    ) -> ::core::cell::Ref<'a, FPRoundingModeAttr> {
        ::core::cell::Ref::map(
            self.op.deref(ctx),
            |op| {
                op
                    .attributes
                    .get::<FPRoundingModeAttr>(&spirv_cl_vstorea_halfn_r::ATTR_MODE)
                    .unwrap()
            },
        )
    }
    ///Set the value of the attribute named `mode`.
    pub fn set_attr_mode(
        &self,
        ctx: &::pliron::context::Context,
        value: FPRoundingModeAttr,
    ) {
        self.op
            .deref_mut(ctx)
            .attributes
            .set(spirv_cl_vstorea_halfn_r::ATTR_MODE.clone(), value);
    }
}
#[op_interface_impl]
impl ToSpirvOp for VstoreaHalfnROp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let data = builder.value_id(self.get_operand_data(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        let mode = self.get_attr_mode(ctx).clone().0;
        builder
            .cl_vstorea_halfn_r_id(result_ty, Some(result), data, offset, p, mode)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for VstoreaHalfnROp {
    #[allow(unused_variables)]
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        #[allow(unused_mut)]
        let mut result: (u8, u8) = None?;
        result = result
            .max(Operand::from(self.get_attr_mode(ctx).clone().0).minimum_version()?);
        Some(result)
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(
                Operand::from(self.get_attr_mode(ctx).clone().0).required_extensions(),
            );
        result
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        #[allow(unused_mut)]
        let mut result = vec![];
        result
            .extend(
                Operand::from(self.get_attr_mode(ctx).clone().0).required_capabilities(),
            );
        result
    }
}
#[pliron_op(
    name = "spirv.CL_shuffle",
    operands = (x, shuffle_mask),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ShuffleOp;
crate::format::canonical_format!(
    ShuffleOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("shuffle_mask", crate ::format::Quantifier::One)
);
mod spirv_cl_shuffle {}
impl ShuffleOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        shuffle_mask: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, shuffle_mask],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for ShuffleOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let shuffle_mask = builder.value_id(self.get_operand_shuffle_mask(ctx));
        builder
            .cl_shuffle_id(result_ty, Some(result), x, shuffle_mask)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for ShuffleOp {
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
#[pliron_op(
    name = "spirv.CL_shuffle2",
    operands = (x, y, shuffle_mask),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Shuffle2Op;
crate::format::canonical_format!(
    Shuffle2Op; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("shuffle_mask", crate ::format::Quantifier::One)
);
mod spirv_cl_shuffle2 {}
impl Shuffle2Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        y: Value,
        shuffle_mask: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y, shuffle_mask],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for Shuffle2Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        let shuffle_mask = builder.value_id(self.get_operand_shuffle_mask(ctx));
        builder
            .cl_shuffle2_id(result_ty, Some(result), x, y, shuffle_mask)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for Shuffle2Op {
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
#[pliron_op(
    name = "spirv.CL_printf",
    operands = (format, additional_arguments),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PrintfOp;
crate::format::canonical_format!(
    PrintfOp; crate ::format::FormatVar::Value("format", crate
    ::format::Quantifier::One), crate ::format::FormatVar::Value("additional_arguments",
    crate ::format::Quantifier::ZeroOrMore)
);
mod spirv_cl_printf {}
impl PrintfOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        format: Value,
        additional_arguments: Vec<Value>,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![format, additional_arguments],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PrintfOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let format = builder.value_id(self.get_operand_format(ctx));
        let additional_arguments = op
            .operands()
            .skip(1usize)
            .map(|opd| builder.value_id(opd))
            .collect::<Vec<_>>();
        builder
            .cl_printf_id(result_ty, Some(result), format, additional_arguments)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for PrintfOp {
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
#[pliron_op(
    name = "spirv.CL_prefetch",
    operands = (ptr, num_elements),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PrefetchOp;
crate::format::canonical_format!(
    PrefetchOp; crate ::format::FormatVar::Value("ptr", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("num_elements", crate ::format::Quantifier::One)
);
mod spirv_cl_prefetch {}
impl PrefetchOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        ptr: Value,
        num_elements: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![ptr, num_elements],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PrefetchOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let ptr = builder.value_id(self.get_operand_ptr(ctx));
        let num_elements = builder.value_id(self.get_operand_num_elements(ctx));
        builder
            .cl_prefetch_id(result_ty, Some(result), ptr, num_elements)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for PrefetchOp {
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
#[pliron_op(
    name = "spirv.CL_bitselect",
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct BitselectOp;
crate::format::canonical_format!(
    BitselectOp; crate ::format::FormatVar::Value("a", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("b", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("c", crate ::format::Quantifier::One)
);
mod spirv_cl_bitselect {}
impl BitselectOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        a: Value,
        b: Value,
        c: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![a, b, c],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for BitselectOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let a = builder.value_id(self.get_operand_a(ctx));
        let b = builder.value_id(self.get_operand_b(ctx));
        let c = builder.value_id(self.get_operand_c(ctx));
        builder.cl_bitselect_id(result_ty, Some(result), a, b, c).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for BitselectOp {
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
#[pliron_op(
    name = "spirv.CL_select",
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SelectOp;
crate::format::canonical_format!(
    SelectOp; crate ::format::FormatVar::Value("a", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("b", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("c", crate ::format::Quantifier::One)
);
mod spirv_cl_select {}
impl SelectOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        a: Value,
        b: Value,
        c: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![a, b, c],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for SelectOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let a = builder.value_id(self.get_operand_a(ctx));
        let b = builder.value_id(self.get_operand_b(ctx));
        let c = builder.value_id(self.get_operand_c(ctx));
        builder.cl_select_id(result_ty, Some(result), a, b, c).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for SelectOp {
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
#[pliron_op(
    name = "spirv.CL_u_abs",
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UAbsOp;
crate::format::canonical_format!(
    UAbsOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One)
);
mod spirv_cl_u_abs {}
impl UAbsOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UAbsOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.cl_u_abs_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UAbsOp {
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
#[pliron_op(
    name = "spirv.CL_u_abs_diff",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UAbsDiffOp;
crate::format::canonical_format!(
    UAbsDiffOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_u_abs_diff {}
impl UAbsDiffOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UAbsDiffOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_u_abs_diff_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UAbsDiffOp {
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
#[pliron_op(
    name = "spirv.CL_u_mul_hi",
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMulHiOp;
crate::format::canonical_format!(
    UMulHiOp; crate ::format::FormatVar::Value("x", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("y", crate ::format::Quantifier::One)
);
mod spirv_cl_u_mul_hi {}
impl UMulHiOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, y: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, y],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UMulHiOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.cl_u_mul_hi_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UMulHiOp {
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
#[pliron_op(
    name = "spirv.CL_u_mad_hi",
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMadHiOp;
crate::format::canonical_format!(
    UMadHiOp; crate ::format::FormatVar::Value("a", crate ::format::Quantifier::One),
    crate ::format::FormatVar::Value("b", crate ::format::Quantifier::One), crate
    ::format::FormatVar::Value("c", crate ::format::Quantifier::One)
);
mod spirv_cl_u_mad_hi {}
impl UMadHiOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        a: Value,
        b: Value,
        c: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![a, b, c],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UMadHiOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let a = builder.value_id(self.get_operand_a(ctx));
        let b = builder.value_id(self.get_operand_b(ctx));
        let c = builder.value_id(self.get_operand_c(ctx));
        builder.cl_u_mad_hi_id(result_ty, Some(result), a, b, c).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[op_interface_impl]
impl VerCapExtOpInterface for UMadHiOp {
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
