// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#![allow(clippy::let_and_return, unused_imports)]
use crate::prelude::*;
use crate::attrs::*;
#[pliron_op(
    name = "spirv.CL.acos",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AcosOp;
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
#[pliron_op(
    name = "spirv.CL.acosh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AcoshOp;
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
#[pliron_op(
    name = "spirv.CL.acospi",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AcospiOp;
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
#[pliron_op(
    name = "spirv.CL.asin",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AsinOp;
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
#[pliron_op(
    name = "spirv.CL.asinh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AsinhOp;
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
#[pliron_op(
    name = "spirv.CL.asinpi",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AsinpiOp;
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
#[pliron_op(
    name = "spirv.CL.atan",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AtanOp;
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
#[pliron_op(
    name = "spirv.CL.atan2",
    format,
    operands = (y, x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Atan2Op;
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
#[pliron_op(
    name = "spirv.CL.atanh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AtanhOp;
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
#[pliron_op(
    name = "spirv.CL.atanpi",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AtanpiOp;
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
#[pliron_op(
    name = "spirv.CL.atan2pi",
    format,
    operands = (y, x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Atan2piOp;
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
#[pliron_op(
    name = "spirv.CL.cbrt",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CbrtOp;
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
#[pliron_op(
    name = "spirv.CL.ceil",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CeilOp;
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
#[pliron_op(
    name = "spirv.CL.copysign",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CopysignOp;
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
#[pliron_op(
    name = "spirv.CL.cos",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CosOp;
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
#[pliron_op(
    name = "spirv.CL.cosh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CoshOp;
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
#[pliron_op(
    name = "spirv.CL.cospi",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CospiOp;
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
#[pliron_op(
    name = "spirv.CL.erfc",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ErfcOp;
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
#[pliron_op(
    name = "spirv.CL.erf",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ErfOp;
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
#[pliron_op(
    name = "spirv.CL.exp",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ExpOp;
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
#[pliron_op(
    name = "spirv.CL.exp2",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Exp2Op;
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
#[pliron_op(
    name = "spirv.CL.exp10",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Exp10Op;
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
#[pliron_op(
    name = "spirv.CL.expm1",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Expm1Op;
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
#[pliron_op(
    name = "spirv.CL.fabs",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FabsOp;
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
#[pliron_op(
    name = "spirv.CL.fdim",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FdimOp;
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
#[pliron_op(
    name = "spirv.CL.floor",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FloorOp;
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
#[pliron_op(
    name = "spirv.CL.fma",
    format,
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FmaOp;
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
#[pliron_op(
    name = "spirv.CL.fmax",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FmaxOp;
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
#[pliron_op(
    name = "spirv.CL.fmin",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FminOp;
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
#[pliron_op(
    name = "spirv.CL.fmod",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FmodOp;
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
#[pliron_op(
    name = "spirv.CL.fract",
    format,
    operands = (x, ptr),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FractOp;
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
#[pliron_op(
    name = "spirv.CL.frexp",
    format,
    operands = (x, exp),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FrexpOp;
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
#[pliron_op(
    name = "spirv.CL.hypot",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HypotOp;
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
#[pliron_op(
    name = "spirv.CL.ilogb",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct IlogbOp;
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
#[pliron_op(
    name = "spirv.CL.ldexp",
    format,
    operands = (x, k),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LdexpOp;
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
#[pliron_op(
    name = "spirv.CL.lgamma",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LgammaOp;
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
#[pliron_op(
    name = "spirv.CL.lgamma_r",
    format,
    operands = (x, signp),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LgammaROp;
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
#[pliron_op(
    name = "spirv.CL.log",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LogOp;
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
#[pliron_op(
    name = "spirv.CL.log2",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Log2Op;
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
#[pliron_op(
    name = "spirv.CL.log10",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Log10Op;
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
#[pliron_op(
    name = "spirv.CL.log1p",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Log1pOp;
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
#[pliron_op(
    name = "spirv.CL.logb",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LogbOp;
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
#[pliron_op(
    name = "spirv.CL.mad",
    format,
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct MadOp;
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
#[pliron_op(
    name = "spirv.CL.maxmag",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct MaxmagOp;
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
#[pliron_op(
    name = "spirv.CL.minmag",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct MinmagOp;
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
#[pliron_op(
    name = "spirv.CL.modf",
    format,
    operands = (x, iptr),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ModfOp;
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
#[pliron_op(
    name = "spirv.CL.nan",
    format,
    operands = (nancode),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NanOp;
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
#[pliron_op(
    name = "spirv.CL.nextafter",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NextafterOp;
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
#[pliron_op(
    name = "spirv.CL.pow",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PowOp;
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
#[pliron_op(
    name = "spirv.CL.pown",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PownOp;
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
#[pliron_op(
    name = "spirv.CL.powr",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PowrOp;
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
#[pliron_op(
    name = "spirv.CL.remainder",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RemainderOp;
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
#[pliron_op(
    name = "spirv.CL.remquo",
    format,
    operands = (x, y, quo),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RemquoOp;
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
#[pliron_op(
    name = "spirv.CL.rint",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RintOp;
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
#[pliron_op(
    name = "spirv.CL.rootn",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RootnOp;
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
#[pliron_op(
    name = "spirv.CL.round",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RoundOp;
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
#[pliron_op(
    name = "spirv.CL.rsqrt",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RsqrtOp;
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
#[pliron_op(
    name = "spirv.CL.sin",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SinOp;
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
#[pliron_op(
    name = "spirv.CL.sincos",
    format,
    operands = (x, cosval),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SincosOp;
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
#[pliron_op(
    name = "spirv.CL.sinh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SinhOp;
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
#[pliron_op(
    name = "spirv.CL.sinpi",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SinpiOp;
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
#[pliron_op(
    name = "spirv.CL.sqrt",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SqrtOp;
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
#[pliron_op(
    name = "spirv.CL.tan",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TanOp;
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
#[pliron_op(
    name = "spirv.CL.tanh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TanhOp;
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
#[pliron_op(
    name = "spirv.CL.tanpi",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TanpiOp;
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
#[pliron_op(
    name = "spirv.CL.tgamma",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TgammaOp;
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
#[pliron_op(
    name = "spirv.CL.trunc",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TruncOp;
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
#[pliron_op(
    name = "spirv.CL.half_cos",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfCosOp;
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
#[pliron_op(
    name = "spirv.CL.half_divide",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfDivideOp;
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
#[pliron_op(
    name = "spirv.CL.half_exp",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfExpOp;
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
#[pliron_op(
    name = "spirv.CL.half_exp2",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfExp2Op;
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
#[pliron_op(
    name = "spirv.CL.half_exp10",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfExp10Op;
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
#[pliron_op(
    name = "spirv.CL.half_log",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfLogOp;
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
#[pliron_op(
    name = "spirv.CL.half_log2",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfLog2Op;
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
#[pliron_op(
    name = "spirv.CL.half_log10",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfLog10Op;
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
#[pliron_op(
    name = "spirv.CL.half_powr",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfPowrOp;
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
#[pliron_op(
    name = "spirv.CL.half_recip",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfRecipOp;
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
#[pliron_op(
    name = "spirv.CL.half_rsqrt",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfRsqrtOp;
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
#[pliron_op(
    name = "spirv.CL.half_sin",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfSinOp;
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
#[pliron_op(
    name = "spirv.CL.half_sqrt",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfSqrtOp;
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
#[pliron_op(
    name = "spirv.CL.half_tan",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct HalfTanOp;
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
#[pliron_op(
    name = "spirv.CL.native_cos",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeCosOp;
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
#[pliron_op(
    name = "spirv.CL.native_divide",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeDivideOp;
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
#[pliron_op(
    name = "spirv.CL.native_exp",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeExpOp;
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
#[pliron_op(
    name = "spirv.CL.native_exp2",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeExp2Op;
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
#[pliron_op(
    name = "spirv.CL.native_exp10",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeExp10Op;
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
#[pliron_op(
    name = "spirv.CL.native_log",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeLogOp;
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
#[pliron_op(
    name = "spirv.CL.native_log2",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeLog2Op;
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
#[pliron_op(
    name = "spirv.CL.native_log10",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeLog10Op;
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
#[pliron_op(
    name = "spirv.CL.native_powr",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativePowrOp;
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
#[pliron_op(
    name = "spirv.CL.native_recip",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeRecipOp;
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
#[pliron_op(
    name = "spirv.CL.native_rsqrt",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeRsqrtOp;
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
#[pliron_op(
    name = "spirv.CL.native_sin",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeSinOp;
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
#[pliron_op(
    name = "spirv.CL.native_sqrt",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeSqrtOp;
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
#[pliron_op(
    name = "spirv.CL.native_tan",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NativeTanOp;
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
#[pliron_op(
    name = "spirv.CL.fclamp",
    format,
    operands = (x, minval, maxval),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FclampOp;
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
#[pliron_op(
    name = "spirv.CL.degrees",
    format,
    operands = (radians),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct DegreesOp;
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
#[pliron_op(
    name = "spirv.CL.fmax_common",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FmaxCommonOp;
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
#[pliron_op(
    name = "spirv.CL.fmin_common",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FminCommonOp;
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
#[pliron_op(
    name = "spirv.CL.mix",
    format,
    operands = (x, y, a),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct MixOp;
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
#[pliron_op(
    name = "spirv.CL.radians",
    format,
    operands = (degrees),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RadiansOp;
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
#[pliron_op(
    name = "spirv.CL.step",
    format,
    operands = (edge, x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct StepOp;
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
#[pliron_op(
    name = "spirv.CL.smoothstep",
    format,
    operands = (edge0, edge1, x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SmoothstepOp;
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
#[pliron_op(
    name = "spirv.CL.sign",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SignOp;
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
#[pliron_op(
    name = "spirv.CL.cross",
    format,
    operands = (p0, p1),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CrossOp;
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
#[pliron_op(
    name = "spirv.CL.distance",
    format,
    operands = (p0, p1),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct DistanceOp;
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
#[pliron_op(
    name = "spirv.CL.length",
    format,
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LengthOp;
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
#[pliron_op(
    name = "spirv.CL.normalize",
    format,
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NormalizeOp;
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
#[pliron_op(
    name = "spirv.CL.fast_distance",
    format,
    operands = (p0, p1),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FastDistanceOp;
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
#[pliron_op(
    name = "spirv.CL.fast_length",
    format,
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FastLengthOp;
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
#[pliron_op(
    name = "spirv.CL.fast_normalize",
    format,
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FastNormalizeOp;
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
#[pliron_op(
    name = "spirv.CL.s_abs",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SAbsOp;
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
#[pliron_op(
    name = "spirv.CL.s_abs_diff",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SAbsDiffOp;
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
#[pliron_op(
    name = "spirv.CL.s_add_sat",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SAddSatOp;
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
#[pliron_op(
    name = "spirv.CL.u_add_sat",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UAddSatOp;
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
#[pliron_op(
    name = "spirv.CL.s_hadd",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SHaddOp;
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
#[pliron_op(
    name = "spirv.CL.u_hadd",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UHaddOp;
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
#[pliron_op(
    name = "spirv.CL.s_rhadd",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SRhaddOp;
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
#[pliron_op(
    name = "spirv.CL.u_rhadd",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct URhaddOp;
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
#[pliron_op(
    name = "spirv.CL.s_clamp",
    format,
    operands = (x, minval, maxval),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SClampOp;
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
#[pliron_op(
    name = "spirv.CL.u_clamp",
    format,
    operands = (x, minval, maxval),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UClampOp;
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
#[pliron_op(
    name = "spirv.CL.clz",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ClzOp;
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
#[pliron_op(
    name = "spirv.CL.ctz",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CtzOp;
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
#[pliron_op(
    name = "spirv.CL.s_mad_hi",
    format,
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMadHiOp;
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
#[pliron_op(
    name = "spirv.CL.u_mad_sat",
    format,
    operands = (x, y, z),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMadSatOp;
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
#[pliron_op(
    name = "spirv.CL.s_mad_sat",
    format,
    operands = (x, y, z),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMadSatOp;
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
#[pliron_op(
    name = "spirv.CL.s_max",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMaxOp;
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
#[pliron_op(
    name = "spirv.CL.u_max",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMaxOp;
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
#[pliron_op(
    name = "spirv.CL.s_min",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMinOp;
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
#[pliron_op(
    name = "spirv.CL.u_min",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMinOp;
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
#[pliron_op(
    name = "spirv.CL.s_mul_hi",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMulHiOp;
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
#[pliron_op(
    name = "spirv.CL.rotate",
    format,
    operands = (v, i),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RotateOp;
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
#[pliron_op(
    name = "spirv.CL.s_sub_sat",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SSubSatOp;
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
#[pliron_op(
    name = "spirv.CL.u_sub_sat",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct USubSatOp;
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
#[pliron_op(
    name = "spirv.CL.u_upsample",
    format,
    operands = (hi, lo),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UUpsampleOp;
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
#[pliron_op(
    name = "spirv.CL.s_upsample",
    format,
    operands = (hi, lo),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SUpsampleOp;
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
#[pliron_op(
    name = "spirv.CL.popcount",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PopcountOp;
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
#[pliron_op(
    name = "spirv.CL.s_mad24",
    format,
    operands = (x, y, z),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMad24Op;
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
#[pliron_op(
    name = "spirv.CL.u_mad24",
    format,
    operands = (x, y, z),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMad24Op;
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
#[pliron_op(
    name = "spirv.CL.s_mul24",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMul24Op;
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
#[pliron_op(
    name = "spirv.CL.u_mul24",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMul24Op;
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
#[pliron_op(
    name = "spirv.CL.vloadn",
    format,
    operands = (offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VloadnOp;
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
                    .get::<LiteralIntegerAttr>(&*spirv_cl_vloadn::ATTR_N)
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
#[pliron_op(
    name = "spirv.CL.vstoren",
    format,
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstorenOp;
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
#[pliron_op(
    name = "spirv.CL.vload_half",
    format,
    operands = (offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VloadHalfOp;
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
#[pliron_op(
    name = "spirv.CL.vload_halfn",
    format,
    operands = (offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VloadHalfnOp;
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
                    .get::<LiteralIntegerAttr>(&*spirv_cl_vload_halfn::ATTR_N)
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
#[pliron_op(
    name = "spirv.CL.vstore_half",
    format,
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreHalfOp;
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
#[pliron_op(
    name = "spirv.CL.vstore_half_r",
    format,
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreHalfROp;
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
                    .get::<FPRoundingModeAttr>(&*spirv_cl_vstore_half_r::ATTR_MODE)
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
#[pliron_op(
    name = "spirv.CL.vstore_halfn",
    format,
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreHalfnOp;
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
#[pliron_op(
    name = "spirv.CL.vstore_halfn_r",
    format,
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreHalfnROp;
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
                    .get::<FPRoundingModeAttr>(&*spirv_cl_vstore_halfn_r::ATTR_MODE)
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
#[pliron_op(
    name = "spirv.CL.vloada_halfn",
    format,
    operands = (offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VloadaHalfnOp;
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
                    .get::<LiteralIntegerAttr>(&*spirv_cl_vloada_halfn::ATTR_N)
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
#[pliron_op(
    name = "spirv.CL.vstorea_halfn",
    format,
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreaHalfnOp;
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
#[pliron_op(
    name = "spirv.CL.vstorea_halfn_r",
    format,
    operands = (data, offset, p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct VstoreaHalfnROp;
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
                    .get::<FPRoundingModeAttr>(&*spirv_cl_vstorea_halfn_r::ATTR_MODE)
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
#[pliron_op(
    name = "spirv.CL.shuffle",
    format,
    operands = (x, shuffle_mask),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ShuffleOp;
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
#[pliron_op(
    name = "spirv.CL.shuffle2",
    format,
    operands = (x, y, shuffle_mask),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Shuffle2Op;
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
#[pliron_op(
    name = "spirv.CL.printf",
    format,
    operands = (format, additional_arguments),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PrintfOp;
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
#[pliron_op(
    name = "spirv.CL.prefetch",
    format,
    operands = (ptr, num_elements),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PrefetchOp;
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
#[pliron_op(
    name = "spirv.CL.bitselect",
    format,
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct BitselectOp;
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
#[pliron_op(
    name = "spirv.CL.select",
    format,
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SelectOp;
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
#[pliron_op(
    name = "spirv.CL.u_abs",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UAbsOp;
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
#[pliron_op(
    name = "spirv.CL.u_abs_diff",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UAbsDiffOp;
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
#[pliron_op(
    name = "spirv.CL.u_mul_hi",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMulHiOp;
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
#[pliron_op(
    name = "spirv.CL.u_mad_hi",
    format,
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMadHiOp;
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
