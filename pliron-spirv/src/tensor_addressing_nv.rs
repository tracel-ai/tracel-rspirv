use core::cell::Ref;

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
        spirv_cooperative_matrix_load_tensor_memory_operand:MemoryAccessAttr,
        spirv_cooperative_matrix_load_tensor_memory_operand_align:LiteralIntegerAttr,
        spirv_cooperative_matrix_load_tensor_tensor_addressing_operands:TensorAddressingOperandsAttr,
        spirv_cooperative_matrix_load_tensor_decode_func:IdentifierAttr,
    ),
    verifier = "succ"
)]
#[derive_op_interface_impl(NResultsInterface<1>, OneResultInterface)]
pub struct CooperativeMatrixLoadTensorOp;
impl CooperativeMatrixLoadTensorOp {
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
        decode_func: Option<Identifier>,
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
        op.set_attr_spirv_cooperative_matrix_load_tensor_memory_operand(ctx, memory_operand.into());
        if let Some(align) = memory_operand_align {
            op.set_attr_spirv_cooperative_matrix_load_tensor_memory_operand_align(ctx, align.into());
        }
        op.set_attr_spirv_cooperative_matrix_load_tensor_tensor_addressing_operands(
            ctx,
            tensor_addressing_operands.into(),
        );
        if let Some(decode_func) = decode_func {
            op.set_attr_spirv_cooperative_matrix_load_tensor_decode_func(ctx, IdentifierAttr::new(decode_func));
        }
        op
    }

    pub fn get_attr_memory_operand<'a>(&self, ctx: &'a Context) -> Ref<'a, MemoryAccessAttr> {
        self.get_attr_spirv_cooperative_matrix_load_tensor_memory_operand(ctx)
            .unwrap()
    }

    pub fn get_attr_tensor_addressing_operands<'a>(&self, ctx: &'a Context) -> Ref<'a, TensorAddressingOperandsAttr> {
        self.get_attr_spirv_cooperative_matrix_load_tensor_tensor_addressing_operands(ctx)
            .unwrap()
    }
}

#[op_interface_impl]
impl ToSpirvOp for CooperativeMatrixLoadTensorOp {
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
            .get_attr_spirv_cooperative_matrix_load_tensor_memory_operand(ctx)
            .map(|it| it.0)
            .unwrap();
        let memory_operand_align = self
            .get_attr_spirv_cooperative_matrix_load_tensor_memory_operand_align(ctx)
            .map(|it| it.0.into());
        let tensor_addressing_operands = self
            .get_attr_spirv_cooperative_matrix_load_tensor_tensor_addressing_operands(ctx)
            .unwrap()
            .0;
        let tensor_view = op
            .operands()
            .skip(3)
            .next()
            .map(|opd| Operand::IdRef(builder.value_id(opd)));
        let decode_func = self
            .get_attr_spirv_cooperative_matrix_load_tensor_decode_func(ctx)
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

#[op_interface_impl]
impl VerCapExtOpInterface for CooperativeMatrixLoadTensorOp {
    fn min_version(&self, _ctx: &Context) -> Option<(u8, u8)> {
        None
    }
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        let mut result = vec![];
        result.extend(Operand::from(self.get_attr_memory_operand(ctx).0).required_extensions());
        result.extend(Operand::from(self.get_attr_tensor_addressing_operands(ctx).0).required_extensions());
        result
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        #[allow(unused_mut)]
        let mut result = vec![vec![Capability::CooperativeMatrixTensorAddressingNV]];
        result.extend(Operand::from(self.get_attr_memory_operand(ctx).0).required_capabilities());
        result.extend(Operand::from(self.get_attr_tensor_addressing_operands(ctx).0).required_capabilities());
        result
    }
}

#[pliron_op(
    name = "spirv.cooperative_matrix_store_tensor_nv",
    format,
    operands = (pointer, object, tensor_layout),
    attributes = (
        spirv_cooperative_matrix_store_tensor_memory_operand:MemoryAccessAttr,
        spirv_cooperative_matrix_store_tensor_memory_operand_align:LiteralIntegerAttr,
        spirv_cooperative_matrix_store_tensor_tensor_addressing_operands:TensorAddressingOperandsAttr
    ),
    verifier = "succ"
)]
pub struct CooperativeMatrixStoreTensorOp;
impl CooperativeMatrixStoreTensorOp {
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
        op.set_attr_spirv_cooperative_matrix_store_tensor_memory_operand(ctx, memory_operand.into());
        if let Some(align) = memory_operand_align {
            op.set_attr_spirv_cooperative_matrix_store_tensor_memory_operand_align(ctx, align.into());
        }
        op.set_attr_spirv_cooperative_matrix_store_tensor_tensor_addressing_operands(
            ctx,
            tensor_addressing_operands.into(),
        );
        op
    }

    pub fn get_attr_memory_operand<'a>(&self, ctx: &'a Context) -> Ref<'a, MemoryAccessAttr> {
        self.get_attr_spirv_cooperative_matrix_store_tensor_memory_operand(ctx)
            .unwrap()
    }

    pub fn get_attr_tensor_addressing_operands<'a>(&self, ctx: &'a Context) -> Ref<'a, TensorAddressingOperandsAttr> {
        self.get_attr_spirv_cooperative_matrix_store_tensor_tensor_addressing_operands(ctx)
            .unwrap()
    }
}

#[op_interface_impl]
impl ToSpirvOp for CooperativeMatrixStoreTensorOp {
    #[allow(unused, clippy::all)]
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        #[allow(unused)]
        let op = self.get_operation().deref(ctx);
        let pointer = builder.value_id(self.get_operand_pointer(ctx));
        let object = builder.value_id(self.get_operand_object(ctx));
        let tensor_layout = builder.value_id(self.get_operand_tensor_layout(ctx));
        let memory_operand = self
            .get_attr_spirv_cooperative_matrix_store_tensor_memory_operand(ctx)
            .map(|it| it.0)
            .unwrap();
        let memory_operand_align = self
            .get_attr_spirv_cooperative_matrix_store_tensor_memory_operand_align(ctx)
            .map(|it| it.0.into());
        let tensor_addressing_operands = self
            .get_attr_spirv_cooperative_matrix_store_tensor_tensor_addressing_operands(ctx)
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

#[op_interface_impl]
impl VerCapExtOpInterface for CooperativeMatrixStoreTensorOp {
    fn min_version(&self, _ctx: &Context) -> Option<(u8, u8)> {
        None
    }
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        let mut result = vec![];
        result.extend(Operand::from(self.get_attr_memory_operand(ctx).0).required_extensions());
        result.extend(Operand::from(self.get_attr_tensor_addressing_operands(ctx).0).required_extensions());
        result
    }
    #[allow(unused_variables, clippy::vec_init_then_push)]
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        #[allow(unused_mut)]
        let mut result = vec![vec![Capability::CooperativeMatrixTensorAddressingNV]];
        result.extend(Operand::from(self.get_attr_memory_operand(ctx).0).required_capabilities());
        result.extend(Operand::from(self.get_attr_tensor_addressing_operands(ctx).0).required_capabilities());
        result
    }
}
