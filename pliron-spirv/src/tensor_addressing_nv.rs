use pliron::builtin::attributes::IdentifierAttr;
use tracel_rspirv::dr::Operand;

use crate::{
    autogen_attrs::{MemoryAccessAttr, TensorAddressingOperandsAttr},
    prelude::*,
};

#[pliron_op(
    name = "spirv.cooperative_matrix_load_tensor_nv",
    format,
    operands = (pointer, object, tensor_layout, tensor_view),
    attributes = (
        spirv_cooperative_matrix_load_tensor_nv_memory_operand:MemoryAccessAttr,
        spirv_cooperative_matrix_load_tensor_nv_memory_operand_align:LiteralIntegerAttr,
        spirv_cooperative_matrix_load_tensor_nv_tensor_addressing_operands:TensorAddressingOperandsAttr,
        spirv_cooperative_matrix_load_tensor_nv_decode_func:IdentifierAttr,
    ),
    verifier = "succ"
)]
#[derive_op_interface_impl(NResultsInterface<1>, OneResultInterface)]
pub struct CooperativeMatrixLoadTensorNVOp;
impl CooperativeMatrixLoadTensorNVOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        pointer: Value,
        object: Value,
        tensor_layout: Value,
        memory_operand: impl Into<MemoryAccessAttr>,
        memory_operand_align: Option<u32>,
        tensor_addressing_operands: impl Into<TensorAddressingOperandsAttr>,
        tensor_view: Option<Value>,
        decode_func: Option<impl Into<IdentifierAttr>>,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![pointer, object, tensor_layout, tensor_view],
                vec![],
                0,
            ),
        };
        op.set_attr_spirv_cooperative_matrix_load_tensor_nv_memory_operand(ctx, memory_operand.into());
        if let Some(align) = memory_operand_align {
            op.set_attr_spirv_cooperative_matrix_load_tensor_nv_memory_operand_align(ctx, align.into());
        }
        op.set_attr_spirv_cooperative_matrix_load_tensor_nv_tensor_addressing_operands(
            ctx,
            tensor_addressing_operands.into(),
        );
        if let Some(decode_func) = decode_func {
            op.set_attr_spirv_cooperative_matrix_load_tensor_nv_decode_func(ctx, decode_func.into());
        }
        op
    }
}

#[op_interface_impl]
impl ToSpirvOp for CooperativeMatrixLoadTensorNVOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let pointer = builder.value_id(self.get_operand_pointer(ctx));
        let object = builder.value_id(self.get_operand_object(ctx));
        let tensor_layout = builder.value_id(self.get_operand_tensor_layout(ctx));
        let memory_operand = self
            .get_attr_spirv_cooperative_matrix_load_tensor_nv_memory_operand(ctx)
            .map(|it| it.0)
            .unwrap();
        let memory_operand_align = self
            .get_attr_spirv_cooperative_matrix_load_tensor_nv_memory_operand_align(ctx)
            .map(|it| it.0.into());
        let tensor_addressing_operands = self
            .get_attr_spirv_cooperative_matrix_load_tensor_nv_tensor_addressing_operands(ctx)
            .unwrap()
            .0;
        let tensor_view = op
            .operands()
            .skip(3)
            .next()
            .map(|opd| Operand::IdRef(builder.value_id(opd)));
        let decode_func = self
            .get_attr_spirv_cooperative_matrix_load_tensor_nv_decode_func(ctx)
            .map(|func| Operand::IdRef(builder.symbol_id(func.clone())));

        builder
            .cooperative_matrix_load_tensor_nv(
                result_ty,
                Some(result),
                pointer,
                object,
                tensor_layout,
                memory_operand,
                memory_operand_align,
                tensor_addressing_operands,
                tensor_view.into_iter().chain(decode_func),
            )
            .into_pliron_result()?;
        Ok(())
    }
}

#[pliron_op(
    name = "spirv.cooperative_matrix_store_tensor_nv",
    format,
    operands = (pointer, object, tensor_layout),
    attributes = (
        spirv_cooperative_matrix_store_tensor_nv_memory_operand:MemoryAccessAttr,
        spirv_cooperative_matrix_store_tensor_nv_memory_operand_align:LiteralIntegerAttr,
        spirv_cooperative_matrix_store_tensor_nv_tensor_addressing_operands:TensorAddressingOperandsAttr
    ),
    verifier = "succ"
)]
pub struct CooperativeMatrixStoreTensorNVOp;
impl CooperativeMatrixStoreTensorNVOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        pointer: Value,
        object: Value,
        tensor_layout: Value,
        memory_operand: impl Into<MemoryAccessAttr>,
        memory_operand_align: Option<u32>,
        tensor_addressing_operands: impl Into<TensorAddressingOperandsAttr>,
        tensor_view: Option<Value>,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![],
                flat_vec![pointer, object, tensor_layout, tensor_view],
                vec![],
                0,
            ),
        };
        op.set_attr_spirv_cooperative_matrix_store_tensor_nv_memory_operand(ctx, memory_operand.into());
        if let Some(align) = memory_operand_align {
            op.set_attr_spirv_cooperative_matrix_store_tensor_nv_memory_operand_align(ctx, align.into());
        }
        op.set_attr_spirv_cooperative_matrix_store_tensor_nv_tensor_addressing_operands(
            ctx,
            tensor_addressing_operands.into(),
        );
        op
    }
}

#[op_interface_impl]
impl ToSpirvOp for CooperativeMatrixStoreTensorNVOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let pointer = builder.value_id(self.get_operand_pointer(ctx));
        let object = builder.value_id(self.get_operand_object(ctx));
        let tensor_layout = builder.value_id(self.get_operand_tensor_layout(ctx));
        let memory_operand = self
            .get_attr_spirv_cooperative_matrix_store_tensor_nv_memory_operand(ctx)
            .map(|it| it.0)
            .unwrap();
        let memory_operand_align = self
            .get_attr_spirv_cooperative_matrix_store_tensor_nv_memory_operand_align(ctx)
            .map(|it| it.0.into());
        let tensor_addressing_operands = self
            .get_attr_spirv_cooperative_matrix_store_tensor_nv_tensor_addressing_operands(ctx)
            .unwrap()
            .0;
        let tensor_view = op
            .operands()
            .skip(3)
            .next()
            .map(|opd| Operand::IdRef(builder.value_id(opd)));
        builder
            .cooperative_matrix_store_tensor_nv(
                pointer,
                object,
                tensor_layout,
                memory_operand,
                memory_operand_align,
                tensor_addressing_operands,
                tensor_view,
            )
            .into_pliron_result()?;
        Ok(())
    }
}

#[pliron_op(
    name = "spirv.cooperative_matrix_per_element_op_nv",
    format,
    operands = (matrix),
    attributes = (
        spirv_cooperative_matrix_store_tensor_nv_func:IdentifierAttr,
    ),
    verifier = "succ"
)]
#[derive_op_interface_impl(NResultsInterface<1>, OneResultInterface)]
pub struct CooperativeMatrixPerElementOpNVOp;
impl CooperativeMatrixPerElementOpNVOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        result_ty: TypeHandle,
        matrix: Value,
        func: impl Into<IdentifierAttr>,
        extra_operands: Option<Value>,
    ) -> Self {
        let op = Self {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![result_ty],
                flat_vec![matrix, extra_operands],
                vec![],
                0,
            ),
        };
        op.set_attr_spirv_cooperative_matrix_store_tensor_nv_func(ctx, func.into());
        op
    }
}

#[op_interface_impl]
impl ToSpirvOp for CooperativeMatrixPerElementOpNVOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;
        let result = builder.value_id(self.get_result(ctx));
        let matrix = builder.value_id(self.get_operand_matrix(ctx));
        let tensor_view = op
            .operands()
            .skip(3)
            .next()
            .map(|opd| Operand::IdRef(builder.value_id(opd)));
        let func = builder.symbol_id(
            self.get_attr_spirv_cooperative_matrix_store_tensor_nv_func(ctx)
                .unwrap()
                .clone(),
        );
        let extra_operands = op
            .operands()
            .skip(1)
            .map(|opd| builder.value_id(opd))
            .collect::<Vec<_>>();

        builder
            .cooperative_matrix_per_element_op_nv(result_ty, Some(result), matrix, func, extra_operands)
            .into_pliron_result()?;
        Ok(())
    }
}
