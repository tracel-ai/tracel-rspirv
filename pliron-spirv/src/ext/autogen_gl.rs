// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#![allow(clippy::let_and_return, unused_imports)]
use crate::prelude::*;
use crate::attrs::*;
#[pliron_op(
    name = "spirv.GL.round",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RoundOp;
mod spirv_gl_round {}
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
        builder.gl_round_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.round_even",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RoundEvenOp;
mod spirv_gl_round_even {}
impl RoundEvenOp {
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
impl ToSpirvOp for RoundEvenOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_round_even_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.trunc",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TruncOp;
mod spirv_gl_trunc {}
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
        builder.gl_trunc_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.f_abs",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FAbsOp;
mod spirv_gl_f_abs {}
impl FAbsOp {
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
impl ToSpirvOp for FAbsOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_f_abs_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.s_abs",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SAbsOp;
mod spirv_gl_s_abs {}
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
        builder.gl_s_abs_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.f_sign",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FSignOp;
mod spirv_gl_f_sign {}
impl FSignOp {
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
impl ToSpirvOp for FSignOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_f_sign_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.s_sign",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SSignOp;
mod spirv_gl_s_sign {}
impl SSignOp {
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
impl ToSpirvOp for SSignOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_s_sign_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.floor",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FloorOp;
mod spirv_gl_floor {}
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
        builder.gl_floor_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.ceil",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CeilOp;
mod spirv_gl_ceil {}
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
        builder.gl_ceil_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.fract",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FractOp;
mod spirv_gl_fract {}
impl FractOp {
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
impl ToSpirvOp for FractOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_fract_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.radians",
    format,
    operands = (degrees),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RadiansOp;
mod spirv_gl_radians {}
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
        builder.gl_radians_id(result_ty, Some(result), degrees).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.degrees",
    format,
    operands = (radians),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct DegreesOp;
mod spirv_gl_degrees {}
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
        builder.gl_degrees_id(result_ty, Some(result), radians).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.sin",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SinOp;
mod spirv_gl_sin {}
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
        builder.gl_sin_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.cos",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CosOp;
mod spirv_gl_cos {}
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
        builder.gl_cos_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.tan",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TanOp;
mod spirv_gl_tan {}
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
        builder.gl_tan_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.asin",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AsinOp;
mod spirv_gl_asin {}
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
        builder.gl_asin_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.acos",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AcosOp;
mod spirv_gl_acos {}
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
        builder.gl_acos_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.atan",
    format,
    operands = (y_over_x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AtanOp;
mod spirv_gl_atan {}
impl AtanOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, y_over_x: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![y_over_x],
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
        let y_over_x = builder.value_id(self.get_operand_y_over_x(ctx));
        builder.gl_atan_id(result_ty, Some(result), y_over_x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.sinh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SinhOp;
mod spirv_gl_sinh {}
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
        builder.gl_sinh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.cosh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CoshOp;
mod spirv_gl_cosh {}
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
        builder.gl_cosh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.tanh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct TanhOp;
mod spirv_gl_tanh {}
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
        builder.gl_tanh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.asinh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AsinhOp;
mod spirv_gl_asinh {}
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
        builder.gl_asinh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.acosh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AcoshOp;
mod spirv_gl_acosh {}
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
        builder.gl_acosh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.atanh",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct AtanhOp;
mod spirv_gl_atanh {}
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
        builder.gl_atanh_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.atan2",
    format,
    operands = (y, x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Atan2Op;
mod spirv_gl_atan2 {}
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
        builder.gl_atan2_id(result_ty, Some(result), y, x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.pow",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PowOp;
mod spirv_gl_pow {}
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
        builder.gl_pow_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.exp",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ExpOp;
mod spirv_gl_exp {}
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
        builder.gl_exp_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.log",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LogOp;
mod spirv_gl_log {}
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
        builder.gl_log_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.exp2",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Exp2Op;
mod spirv_gl_exp2 {}
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
        builder.gl_exp2_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.log2",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct Log2Op;
mod spirv_gl_log2 {}
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
        builder.gl_log2_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.sqrt",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SqrtOp;
mod spirv_gl_sqrt {}
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
        builder.gl_sqrt_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.inverse_sqrt",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct InverseSqrtOp;
mod spirv_gl_inverse_sqrt {}
impl InverseSqrtOp {
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
impl ToSpirvOp for InverseSqrtOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_inverse_sqrt_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.determinant",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct DeterminantOp;
mod spirv_gl_determinant {}
impl DeterminantOp {
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
impl ToSpirvOp for DeterminantOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_determinant_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.matrix_inverse",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct MatrixInverseOp;
mod spirv_gl_matrix_inverse {}
impl MatrixInverseOp {
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
impl ToSpirvOp for MatrixInverseOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_matrix_inverse_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.modf",
    format,
    operands = (x, i),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ModfOp;
mod spirv_gl_modf {}
impl ModfOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, x: Value, i: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, i],
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
        let i = builder.value_id(self.get_operand_i(ctx));
        builder.gl_modf_id(result_ty, Some(result), x, i).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.modf_struct",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ModfStructOp;
mod spirv_gl_modf_struct {}
impl ModfStructOp {
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
impl ToSpirvOp for ModfStructOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_modf_struct_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.f_min",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FMinOp;
mod spirv_gl_f_min {}
impl FMinOp {
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
impl ToSpirvOp for FMinOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.gl_f_min_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.u_min",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMinOp;
mod spirv_gl_u_min {}
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
        builder.gl_u_min_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.s_min",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMinOp;
mod spirv_gl_s_min {}
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
        builder.gl_s_min_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.f_max",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FMaxOp;
mod spirv_gl_f_max {}
impl FMaxOp {
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
impl ToSpirvOp for FMaxOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.gl_f_max_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.u_max",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UMaxOp;
mod spirv_gl_u_max {}
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
        builder.gl_u_max_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.s_max",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SMaxOp;
mod spirv_gl_s_max {}
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
        builder.gl_s_max_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.f_clamp",
    format,
    operands = (x, min_val, max_val),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FClampOp;
mod spirv_gl_f_clamp {}
impl FClampOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        min_val: Value,
        max_val: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, min_val, max_val],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FClampOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let min_val = builder.value_id(self.get_operand_min_val(ctx));
        let max_val = builder.value_id(self.get_operand_max_val(ctx));
        builder
            .gl_f_clamp_id(result_ty, Some(result), x, min_val, max_val)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.u_clamp",
    format,
    operands = (x, min_val, max_val),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UClampOp;
mod spirv_gl_u_clamp {}
impl UClampOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        min_val: Value,
        max_val: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, min_val, max_val],
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
        let min_val = builder.value_id(self.get_operand_min_val(ctx));
        let max_val = builder.value_id(self.get_operand_max_val(ctx));
        builder
            .gl_u_clamp_id(result_ty, Some(result), x, min_val, max_val)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.s_clamp",
    format,
    operands = (x, min_val, max_val),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SClampOp;
mod spirv_gl_s_clamp {}
impl SClampOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        min_val: Value,
        max_val: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, min_val, max_val],
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
        let min_val = builder.value_id(self.get_operand_min_val(ctx));
        let max_val = builder.value_id(self.get_operand_max_val(ctx));
        builder
            .gl_s_clamp_id(result_ty, Some(result), x, min_val, max_val)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.f_mix",
    format,
    operands = (x, y, a),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FMixOp;
mod spirv_gl_f_mix {}
impl FMixOp {
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
impl ToSpirvOp for FMixOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        let a = builder.value_id(self.get_operand_a(ctx));
        builder.gl_f_mix_id(result_ty, Some(result), x, y, a).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.i_mix",
    format,
    operands = (x, y, a),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct IMixOp;
mod spirv_gl_i_mix {}
impl IMixOp {
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
impl ToSpirvOp for IMixOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        let a = builder.value_id(self.get_operand_a(ctx));
        builder.gl_i_mix_id(result_ty, Some(result), x, y, a).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.step",
    format,
    operands = (edge, x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct StepOp;
mod spirv_gl_step {}
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
        builder.gl_step_id(result_ty, Some(result), edge, x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.smooth_step",
    format,
    operands = (edge0, edge1, x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct SmoothStepOp;
mod spirv_gl_smooth_step {}
impl SmoothStepOp {
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
impl ToSpirvOp for SmoothStepOp {
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
            .gl_smooth_step_id(result_ty, Some(result), edge0, edge1, x)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.fma",
    format,
    operands = (a, b, c),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FmaOp;
mod spirv_gl_fma {}
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
        builder.gl_fma_id(result_ty, Some(result), a, b, c).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.frexp",
    format,
    operands = (x, exp),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FrexpOp;
mod spirv_gl_frexp {}
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
        builder.gl_frexp_id(result_ty, Some(result), x, exp).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.frexp_struct",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FrexpStructOp;
mod spirv_gl_frexp_struct {}
impl FrexpStructOp {
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
impl ToSpirvOp for FrexpStructOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_frexp_struct_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.ldexp",
    format,
    operands = (x, exp),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LdexpOp;
mod spirv_gl_ldexp {}
impl LdexpOp {
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
impl ToSpirvOp for LdexpOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let exp = builder.value_id(self.get_operand_exp(ctx));
        builder.gl_ldexp_id(result_ty, Some(result), x, exp).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.pack_snorm4x8",
    format,
    operands = (v),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PackSnorm4x8Op;
mod spirv_gl_pack_snorm4x8 {}
impl PackSnorm4x8Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, v: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![v],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PackSnorm4x8Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let v = builder.value_id(self.get_operand_v(ctx));
        builder.gl_pack_snorm4x8_id(result_ty, Some(result), v).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.pack_unorm4x8",
    format,
    operands = (v),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PackUnorm4x8Op;
mod spirv_gl_pack_unorm4x8 {}
impl PackUnorm4x8Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, v: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![v],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PackUnorm4x8Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let v = builder.value_id(self.get_operand_v(ctx));
        builder.gl_pack_unorm4x8_id(result_ty, Some(result), v).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.pack_snorm2x16",
    format,
    operands = (v),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PackSnorm2x16Op;
mod spirv_gl_pack_snorm2x16 {}
impl PackSnorm2x16Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, v: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![v],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PackSnorm2x16Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let v = builder.value_id(self.get_operand_v(ctx));
        builder.gl_pack_snorm2x16_id(result_ty, Some(result), v).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.pack_unorm2x16",
    format,
    operands = (v),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PackUnorm2x16Op;
mod spirv_gl_pack_unorm2x16 {}
impl PackUnorm2x16Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, v: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![v],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PackUnorm2x16Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let v = builder.value_id(self.get_operand_v(ctx));
        builder.gl_pack_unorm2x16_id(result_ty, Some(result), v).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.pack_half2x16",
    format,
    operands = (v),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PackHalf2x16Op;
mod spirv_gl_pack_half2x16 {}
impl PackHalf2x16Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, v: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![v],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PackHalf2x16Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let v = builder.value_id(self.get_operand_v(ctx));
        builder.gl_pack_half2x16_id(result_ty, Some(result), v).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.pack_double2x32",
    format,
    operands = (v),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct PackDouble2x32Op;
mod spirv_gl_pack_double2x32 {}
impl PackDouble2x32Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, v: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![v],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for PackDouble2x32Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let v = builder.value_id(self.get_operand_v(ctx));
        builder.gl_pack_double2x32_id(result_ty, Some(result), v).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.unpack_snorm2x16",
    format,
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UnpackSnorm2x16Op;
mod spirv_gl_unpack_snorm2x16 {}
impl UnpackSnorm2x16Op {
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
impl ToSpirvOp for UnpackSnorm2x16Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder.gl_unpack_snorm2x16_id(result_ty, Some(result), p).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.unpack_unorm2x16",
    format,
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UnpackUnorm2x16Op;
mod spirv_gl_unpack_unorm2x16 {}
impl UnpackUnorm2x16Op {
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
impl ToSpirvOp for UnpackUnorm2x16Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder.gl_unpack_unorm2x16_id(result_ty, Some(result), p).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.unpack_half2x16",
    format,
    operands = (v),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UnpackHalf2x16Op;
mod spirv_gl_unpack_half2x16 {}
impl UnpackHalf2x16Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, v: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![v],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UnpackHalf2x16Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let v = builder.value_id(self.get_operand_v(ctx));
        builder.gl_unpack_half2x16_id(result_ty, Some(result), v).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.unpack_snorm4x8",
    format,
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UnpackSnorm4x8Op;
mod spirv_gl_unpack_snorm4x8 {}
impl UnpackSnorm4x8Op {
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
impl ToSpirvOp for UnpackSnorm4x8Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder.gl_unpack_snorm4x8_id(result_ty, Some(result), p).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.unpack_unorm4x8",
    format,
    operands = (p),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UnpackUnorm4x8Op;
mod spirv_gl_unpack_unorm4x8 {}
impl UnpackUnorm4x8Op {
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
impl ToSpirvOp for UnpackUnorm4x8Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let p = builder.value_id(self.get_operand_p(ctx));
        builder.gl_unpack_unorm4x8_id(result_ty, Some(result), p).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.unpack_double2x32",
    format,
    operands = (v),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct UnpackDouble2x32Op;
mod spirv_gl_unpack_double2x32 {}
impl UnpackDouble2x32Op {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, v: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![v],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for UnpackDouble2x32Op {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let v = builder.value_id(self.get_operand_v(ctx));
        builder
            .gl_unpack_double2x32_id(result_ty, Some(result), v)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.length",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct LengthOp;
mod spirv_gl_length {}
impl LengthOp {
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
impl ToSpirvOp for LengthOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_length_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.distance",
    format,
    operands = (p0, p1),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct DistanceOp;
mod spirv_gl_distance {}
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
        builder.gl_distance_id(result_ty, Some(result), p0, p1).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.cross",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct CrossOp;
mod spirv_gl_cross {}
impl CrossOp {
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
impl ToSpirvOp for CrossOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.gl_cross_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.normalize",
    format,
    operands = (x),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NormalizeOp;
mod spirv_gl_normalize {}
impl NormalizeOp {
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
impl ToSpirvOp for NormalizeOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        builder.gl_normalize_id(result_ty, Some(result), x).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.face_forward",
    format,
    operands = (n, i, nref),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FaceForwardOp;
mod spirv_gl_face_forward {}
impl FaceForwardOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        n: Value,
        i: Value,
        nref: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![n, i, nref],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FaceForwardOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let n = builder.value_id(self.get_operand_n(ctx));
        let i = builder.value_id(self.get_operand_i(ctx));
        let nref = builder.value_id(self.get_operand_nref(ctx));
        builder
            .gl_face_forward_id(result_ty, Some(result), n, i, nref)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.reflect",
    format,
    operands = (i, n),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct ReflectOp;
mod spirv_gl_reflect {}
impl ReflectOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, i: Value, n: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![i, n],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for ReflectOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let i = builder.value_id(self.get_operand_i(ctx));
        let n = builder.value_id(self.get_operand_n(ctx));
        builder.gl_reflect_id(result_ty, Some(result), i, n).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.refract",
    format,
    operands = (i, n, eta),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct RefractOp;
mod spirv_gl_refract {}
impl RefractOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        i: Value,
        n: Value,
        eta: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![i, n, eta],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for RefractOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let i = builder.value_id(self.get_operand_i(ctx));
        let n = builder.value_id(self.get_operand_n(ctx));
        let eta = builder.value_id(self.get_operand_eta(ctx));
        builder.gl_refract_id(result_ty, Some(result), i, n, eta).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.find_i_lsb",
    format,
    operands = (value),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FindILsbOp;
mod spirv_gl_find_i_lsb {}
impl FindILsbOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, value: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![value],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FindILsbOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let value = builder.value_id(self.get_operand_value(ctx));
        builder.gl_find_i_lsb_id(result_ty, Some(result), value).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.find_s_msb",
    format,
    operands = (value),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FindSMsbOp;
mod spirv_gl_find_s_msb {}
impl FindSMsbOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, value: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![value],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FindSMsbOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let value = builder.value_id(self.get_operand_value(ctx));
        builder.gl_find_s_msb_id(result_ty, Some(result), value).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.find_u_msb",
    format,
    operands = (value),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct FindUMsbOp;
mod spirv_gl_find_u_msb {}
impl FindUMsbOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, value: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![value],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for FindUMsbOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let value = builder.value_id(self.get_operand_value(ctx));
        builder.gl_find_u_msb_id(result_ty, Some(result), value).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.interpolate_at_centroid",
    format,
    operands = (interpolant),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct InterpolateAtCentroidOp;
mod spirv_gl_interpolate_at_centroid {}
impl InterpolateAtCentroidOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, result_ty: TypeHandle, interpolant: Value) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![interpolant],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for InterpolateAtCentroidOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let interpolant = builder.value_id(self.get_operand_interpolant(ctx));
        builder
            .gl_interpolate_at_centroid_id(result_ty, Some(result), interpolant)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.interpolate_at_sample",
    format,
    operands = (interpolant, sample),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct InterpolateAtSampleOp;
mod spirv_gl_interpolate_at_sample {}
impl InterpolateAtSampleOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        interpolant: Value,
        sample: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![interpolant, sample],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for InterpolateAtSampleOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let interpolant = builder.value_id(self.get_operand_interpolant(ctx));
        let sample = builder.value_id(self.get_operand_sample(ctx));
        builder
            .gl_interpolate_at_sample_id(result_ty, Some(result), interpolant, sample)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.interpolate_at_offset",
    format,
    operands = (interpolant, offset),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct InterpolateAtOffsetOp;
mod spirv_gl_interpolate_at_offset {}
impl InterpolateAtOffsetOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        interpolant: Value,
        offset: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![interpolant, offset],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for InterpolateAtOffsetOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let interpolant = builder.value_id(self.get_operand_interpolant(ctx));
        let offset = builder.value_id(self.get_operand_offset(ctx));
        builder
            .gl_interpolate_at_offset_id(result_ty, Some(result), interpolant, offset)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.n_min",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NMinOp;
mod spirv_gl_n_min {}
impl NMinOp {
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
impl ToSpirvOp for NMinOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.gl_n_min_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.n_max",
    format,
    operands = (x, y),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NMaxOp;
mod spirv_gl_n_max {}
impl NMaxOp {
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
impl ToSpirvOp for NMaxOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let y = builder.value_id(self.get_operand_y(ctx));
        builder.gl_n_max_id(result_ty, Some(result), x, y).into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
#[pliron_op(
    name = "spirv.GL.n_clamp",
    format,
    operands = (x, min_val, max_val),
    interfaces = [NResultsInterface<1>,
    OneResultInterface,
    DecoratableOp],
    verifier = "succ"
)]
pub struct NClampOp;
mod spirv_gl_n_clamp {}
impl NClampOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        x: Value,
        min_val: Value,
        max_val: Value,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![x, min_val, max_val],
                vec![],
                0,
            ),
        };
        op
    }
}
#[op_interface_impl]
impl ToSpirvOp for NClampOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let x = builder.value_id(self.get_operand_x(ctx));
        let min_val = builder.value_id(self.get_operand_min_val(ctx));
        let max_val = builder.value_id(self.get_operand_max_val(ctx));
        builder
            .gl_n_clamp_id(result_ty, Some(result), x, min_val, max_val)
            .into_pliron_result()?;
        crate::ops::apply_all_decorations(ctx, builder, self, result);
        Ok(())
    }
}
