use alloc::boxed::Box;
use pliron::{
    attribute::{AttrObj, attr_cast},
    basic_block::BasicBlock,
    builtin::{
        attributes::{IdentifierAttr, IntegerAttr, TypeAttr},
        ops::ConstantOp,
        types::IntegerType,
    },
    context::Ptr,
    graph::ControlFlowGraph,
    identifier::Identifier,
    irbuild::{IRStatus, rewriter::Rewriter},
    linked_list::ContainsLinkedList,
    op::op_cast,
    opts::constants::BranchOpFoldInterface,
    region::Region,
    verify_err,
};
use tracel_rspirv::spirv::{AddressingModel, ExecutionMode, MemoryModel, Word};

use crate::{
    ToSpirvAttr,
    autogen_attrs::{BuiltInAttr, StorageClassAttr},
    decorations::all_decorations,
    parse::canonical_format,
    prelude::*,
    spirv_symbol_id,
    types::PointerType,
};

pub use crate::autogen_ops::*;

#[pliron_op(
    name = "spirv.global_variable",
    attributes = (
        spirv_global_variable_type:TypeAttr,
        spirv_global_variable_storage_class:StorageClassAttr,
        spirv_global_variable_initializer:IdentifierAttr,
        spirv_global_variable_location:LiteralIntegerAttr,
        spirv_global_variable_binding:LiteralIntegerAttr,
        spirv_global_variable_descriptor_set:LiteralIntegerAttr,
        spirv_global_variable_built_in:BuiltInAttr,
    ),
    verifier = "succ"
)]
#[derive_op_interface_impl(SymbolOpInterface, DecoratableOp)]
pub struct GlobalVariableOp;
canonical_format!(GlobalVariableOp);
impl GlobalVariableOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        r#type: TypeHandle,
        storage_class: impl Into<StorageClassAttr>,
        sym_name: Identifier,
        initializer: Option<Identifier>,
    ) -> Self {
        let op = Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![r#type], vec![], vec![], 0),
        };
        op.set_attr_spirv_global_variable_storage_class(ctx, storage_class.into());
        op.set_symbol_name(ctx, sym_name);
        if let Some(initializer) = initializer {
            op.set_attr_spirv_global_variable_initializer(ctx, IdentifierAttr::new(initializer));
        }
        op
    }

    pub fn set_location(&self, ctx: &Context, location: impl Into<LiteralIntegerAttr>) {
        self.set_attr_spirv_global_variable_location(ctx, location.into());
    }

    pub fn set_binding(&self, ctx: &Context, binding: impl Into<LiteralIntegerAttr>) {
        self.set_attr_spirv_global_variable_binding(ctx, binding.into());
    }

    pub fn set_descriptor_set(&self, ctx: &Context, descriptor_set: impl Into<LiteralIntegerAttr>) {
        self.set_attr_spirv_global_variable_descriptor_set(ctx, descriptor_set.into());
    }

    pub fn set_built_in(&self, ctx: &Context, built_in: impl Into<BuiltInAttr>) {
        self.set_attr_spirv_global_variable_built_in(ctx, built_in.into());
    }
}

#[op_interface_impl]
impl ToSpirvOp for GlobalVariableOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let element_type = self.get_attr_spirv_global_variable_type(ctx).unwrap();
        let storage_class = self.get_attr_spirv_global_variable_storage_class(ctx).unwrap().0;
        let sym_name = self.get_symbol_name(ctx);
        let initializer = self
            .get_attr_spirv_global_variable_initializer(ctx)
            .map(|sym| spirv_symbol_id(ctx, builder, sym.clone()))
            .transpose()?;

        let ty = PointerType::get(ctx, element_type.get_type(ctx), storage_class).to_handle();
        let result_ty = spirv_type_id(ctx, builder, ty)?;
        let result = builder.id();
        builder.symbols.insert(sym_name, result);

        apply_all_decorations(ctx, builder, self, result);

        builder
            .variable(result_ty, Some(result), storage_class, initializer)
            .into_pliron_result()?;
        Ok(())
    }
}

#[pliron_op(
    name = "spirv.ExecutionMode",
    attributes = (
        spirv_execution_mode_entry_point: IdentifierAttr,
        spirv_execution_mode_execution_mode: ExecutionModeAttr,
        spirv_execution_mode_arguments: VecAttr,
    ),
    verifier = "succ"
)]
pub struct ExecutionModeOp;
canonical_format!(ExecutionModeOp);
impl ExecutionModeOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, entry_point: Identifier, mode: impl Into<ExecutionModeAttr>, args: Vec<u32>) -> Self {
        let op = Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![], vec![], vec![], 0),
        };
        op.set_attr_spirv_execution_mode_entry_point(ctx, IdentifierAttr::new(entry_point));
        op.set_attr_spirv_execution_mode_execution_mode(ctx, mode.into());
        op.set_attr_spirv_execution_mode_arguments(
            ctx,
            VecAttr(
                args.into_iter()
                    .map(|lit| -> AttrObj { Box::new(LiteralIntegerAttr::new(lit)) })
                    .collect(),
            ),
        );
        op
    }

    pub fn entry_point(&self, ctx: &Context) -> Identifier {
        self.get_attr_spirv_execution_mode_entry_point(ctx)
            .unwrap()
            .clone()
            .into()
    }

    pub fn mode(&self, ctx: &Context) -> ExecutionMode {
        self.get_attr_spirv_execution_mode_execution_mode(ctx)
            .unwrap()
            .clone()
            .0
    }

    pub fn arguments(&self, ctx: &Context) -> Vec<u32> {
        self.get_attr_spirv_execution_mode_arguments(ctx)
            .unwrap()
            .0
            .iter()
            .map(|attr| attr.downcast_ref::<LiteralIntegerAttr>().unwrap().0)
            .collect()
    }
}

#[op_interface_impl]
impl ToSpirvOp for ExecutionModeOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let entry_point = spirv_symbol_id(ctx, builder, self.entry_point(ctx))?;
        let mode = self.mode(ctx);
        let args = self.arguments(ctx);
        builder.execution_mode(entry_point, mode, args);
        Ok(())
    }
}

#[pliron_op(
    name = "spirv.ExecutionModeId",
    attributes = (
        spirv_execution_mode_entry_point: IdentifierAttr,
        spirv_execution_mode_execution_mode: ExecutionModeAttr,
    ),
    verifier = "succ"
)]
pub struct ExecutionModeIdOp;
canonical_format!(ExecutionModeIdOp);
impl ExecutionModeIdOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        entry_point: Identifier,
        mode: impl Into<ExecutionModeAttr>,
        args: Vec<Value>,
    ) -> Self {
        let op = Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![], args, vec![], 0),
        };
        op.set_attr_spirv_execution_mode_entry_point(ctx, IdentifierAttr::new(entry_point));
        op.set_attr_spirv_execution_mode_execution_mode(ctx, mode.into());
        op
    }

    pub fn entry_point(&self, ctx: &Context) -> Identifier {
        self.get_attr_spirv_execution_mode_entry_point(ctx)
            .unwrap()
            .clone()
            .into()
    }

    pub fn mode(&self, ctx: &Context) -> ExecutionMode {
        self.get_attr_spirv_execution_mode_execution_mode(ctx)
            .unwrap()
            .clone()
            .0
    }

    pub fn arguments(&self, ctx: &Context) -> Vec<Value> {
        self.get_operation().deref(ctx).operands().collect()
    }
}

#[op_interface_impl]
impl ToSpirvOp for ExecutionModeIdOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let op = self.get_operation().deref(ctx);
        let entry_point = spirv_symbol_id(ctx, builder, self.entry_point(ctx))?;
        let mode = self.mode(ctx);
        let args = op.operands().map(|val| builder.value_id(val)).collect::<Vec<_>>();
        builder.execution_mode_id(entry_point, mode, args);
        Ok(())
    }
}

#[op_interface_impl]
impl DecoratableOp for ConstantOp {}

#[op_interface_impl]
impl ToSpirvOp for ConstantOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let attr = self.get_value(ctx);
        let Some(to_spirv) = attr_cast::<dyn ToSpirvAttr>(&*attr) else {
            return verify_err!(self.loc(ctx), "Constants must implement `ToSpirvAttr`");
        };
        let id = to_spirv.to_spirv(ctx, builder)?;
        apply_all_decorations(ctx, builder, self, id);
        Ok(())
    }
}

#[pliron_op(
    name = "spirv.pliron.addressof",
    attributes = (
        spirv_addressof_variable: IdentifierAttr,
    ),
    verifier = "succ"
)]
#[derive_op_interface_impl(NResultsInterface<1>, OneResultInterface)]
pub struct AddressOfOp;
canonical_format!(AddressOfOp);
impl AddressOfOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, r#type: TypeHandle, variable: Identifier) -> Self {
        let op = Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![r#type], vec![], vec![], 0),
        };
        op.set_attr_spirv_addressof_variable(ctx, IdentifierAttr::new(variable));
        op
    }
}

#[op_interface_impl]
impl ToSpirvOp for AddressOfOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let symbol = self.get_attr_spirv_addressof_variable(ctx).unwrap().clone();
        let id = spirv_symbol_id(ctx, builder, symbol)?;
        builder.values.insert(self.get_result(ctx), id);
        Ok(())
    }
}

#[pliron_op(name = "spirv.pliron.merge", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct MergeOp;
canonical_format!(MergeOp);
impl MergeOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, operands: Vec<Value>) -> Self {
        Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![], operands, vec![], 0),
        }
    }
}

#[op_interface_impl]
impl ToSpirvOp for MergeOp {
    fn to_spirv(&self, _ctx: &Context, _builder: &mut PlironBuilder) -> Result<()> {
        // Handled by the higher-level selection construct
        Ok(())
    }
}

#[pliron_op(name = "spirv.Branch", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface, NResultsInterface<0>, NSuccsInterface<1>, OneSuccInterface)]
pub struct BranchOp;
canonical_format!(BranchOp);
impl BranchOp {
    pub fn new(ctx: &mut Context, dest: Ptr<BasicBlock>, dest_opds: Vec<Value>) -> Self {
        BranchOp {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![], dest_opds, vec![dest], 0),
        }
    }
}

#[op_interface_impl]
impl BranchOpInterface for BranchOp {
    fn successor_operands(&self, ctx: &Context, _succ_idx: usize) -> Vec<Value> {
        self.get_operation().deref(ctx).operands().collect()
    }

    fn add_successor_operand(&self, ctx: &mut Context, _succ_idx: usize, operand: Value) -> usize {
        Operation::push_operand(self.get_operation(), ctx, operand)
    }

    fn remove_successor_operand(&self, ctx: &mut Context, _succ_idx: usize, opd_idx: usize) -> Value {
        Operation::remove_operand(self.get_operation(), ctx, opd_idx)
    }
}

#[op_interface_impl]
impl BranchOpFoldInterface for BranchOp {
    fn check_fold(&self, ctx: &Context, _operands: &[Option<AttrObj>]) -> Vec<Ptr<BasicBlock>> {
        self.get_operation().deref(ctx).successors().collect()
    }
    fn fold_in_place(&self, _ctx: &mut Context, _ops: &[Option<AttrObj>], _rw: &mut dyn Rewriter) -> IRStatus {
        IRStatus::Unchanged
    }
}

#[op_interface_impl]
impl ToSpirvOp for BranchOp {
    fn to_spirv(&self, _ctx: &Context, _builder: &mut PlironBuilder) -> Result<()> {
        todo!()
    }
}

#[pliron_op(
    name = "spirv.BranchConditional",
    operands = (condition: IntegerType, true_dest_opds, false_dest_opds),
    verifier = "succ"
)]
#[derive_op_interface_impl(IsTerminatorInterface, NResultsInterface<0>, NSuccsInterface<2>, OperandSegmentInterface)]
pub struct BranchConditionalOp;
canonical_format!(BranchConditionalOp);
impl BranchConditionalOp {
    /// Create a new [CondBrOp].
    pub fn new(
        ctx: &mut Context,
        condition: Value,
        true_dest: Ptr<BasicBlock>,
        true_dest_opds: Vec<Value>,
        false_dest: Ptr<BasicBlock>,
        false_dest_opds: Vec<Value>,
    ) -> Self {
        let (operands, segment_sizes) =
            Self::compute_segment_sizes(vec![vec![condition], true_dest_opds, false_dest_opds]);

        let op = BranchConditionalOp {
            op: Operation::new(
                ctx,
                Self::get_concrete_op_info(),
                vec![],
                operands,
                vec![true_dest, false_dest],
                0,
            ),
        };

        // Set the operand segment sizes attribute.
        op.set_operand_segment_sizes(ctx, segment_sizes);
        op
    }
}

#[op_interface_impl]
impl BranchOpInterface for BranchConditionalOp {
    fn successor_operands(&self, ctx: &Context, succ_idx: usize) -> Vec<Value> {
        // Skip the first segment, which is the condition.
        self.get_segment(ctx, succ_idx + 1)
    }

    fn add_successor_operand(&self, ctx: &mut Context, succ_idx: usize, operand: Value) -> usize {
        // The successor operands start at segment 1, since segment 0 is the condition operand.
        self.push_to_segment(ctx, succ_idx + 1, operand)
    }

    fn remove_successor_operand(&self, ctx: &mut Context, succ_idx: usize, opd_idx: usize) -> Value {
        // The successor operands start at segment 1, since segment 0 is the condition operand.
        self.remove_from_segment(ctx, succ_idx + 1, opd_idx)
    }
}

impl BranchConditionalOp {
    fn possible_successor_indices(&self, ctx: &Context, operands: &[Option<AttrObj>]) -> Vec<usize> {
        let Some(cond_attr) = operands.first().unwrap().as_ref() else {
            let num_successors = self.get_operation().deref(ctx).successors().count();
            return (0..num_successors).collect();
        };
        let cond = cond_attr
            .downcast_ref::<IntegerAttr>()
            .expect("CondBrOp condition operand must be an IntegerAttr");
        let taken = if cond.value().is_zero() { 1 } else { 0 };
        vec![taken]
    }
}

#[op_interface_impl]
impl BranchOpFoldInterface for BranchConditionalOp {
    fn check_fold(&self, ctx: &Context, operands: &[Option<AttrObj>]) -> Vec<Ptr<BasicBlock>> {
        let successors: Vec<Ptr<BasicBlock>> = self.get_operation().deref(ctx).successors().collect();

        self.possible_successor_indices(ctx, operands)
            .iter()
            .map(|ind| successors[*ind])
            .collect()
    }

    fn fold_in_place(&self, ctx: &mut Context, ops: &[Option<AttrObj>], rewriter: &mut dyn Rewriter) -> IRStatus {
        let possible_successor_indices = self.possible_successor_indices(ctx, ops);
        if possible_successor_indices.len() != 1 {
            return IRStatus::Unchanged;
        };
        let successor_ind = possible_successor_indices[0];
        let successors: Vec<Ptr<BasicBlock>> = self.get_operation().deref(ctx).successors().collect();
        let new_op = BranchOp::new(
            ctx,
            successors[successor_ind],
            self.successor_operands(ctx, successor_ind),
        )
        .get_operation();
        let old_op = self.get_operation();
        rewriter.insert_operation(ctx, new_op);
        rewriter.replace_operation(ctx, old_op, new_op);
        IRStatus::Changed
    }
}

#[pliron_op(name = "spirv.pliron.selection", verifier = "succ")]
#[derive_op_interface_impl(NRegionsInterface<1>, OneRegionInterface)]
pub struct SelectionOp;
canonical_format!(SelectionOp);
impl SelectionOp {
    pub fn new(ctx: &mut Context, result_types: Vec<TypeHandle>) -> Self {
        let op = Operation::new(ctx, Self::get_concrete_op_info(), result_types, vec![], vec![], 1);
        let region = op.deref(ctx).get_region(0);
        let entry_block = BasicBlock::new(ctx, None, vec![]);
        entry_block.insert_at_front(region, ctx);
        Self { op }
    }

    pub fn region(&self, ctx: &Context) -> Ptr<Region> {
        self.get_operation().deref(ctx).get_region(0)
    }

    pub fn entry_block(&self, ctx: &Context) -> Ptr<BasicBlock> {
        self.region(ctx).entry_node(ctx).unwrap()
    }
}

#[op_interface_impl]
impl ToSpirvOp for SelectionOp {
    fn to_spirv(&self, _ctx: &Context, _builder: &mut PlironBuilder) -> Result<()> {
        todo!();
    }
}

#[pliron_op(name = "spirv.pliron.loop", verifier = "succ")]
#[derive_op_interface_impl(NRegionsInterface<1>, OneRegionInterface)]
pub struct LoopOp;
canonical_format!(LoopOp);
impl LoopOp {
    pub fn new(ctx: &mut Context, result_types: Vec<TypeHandle>) -> Self {
        let op = Operation::new(ctx, Self::get_concrete_op_info(), result_types, vec![], vec![], 1);
        let region = op.deref(ctx).get_region(0);
        let entry_block = BasicBlock::new(ctx, None, vec![]);
        entry_block.insert_at_front(region, ctx);
        Self { op }
    }

    pub fn region(&self, ctx: &Context) -> Ptr<Region> {
        self.get_operation().deref(ctx).get_region(0)
    }

    pub fn entry_block(&self, ctx: &Context) -> Ptr<BasicBlock> {
        self.region(ctx).entry_node(ctx).unwrap()
    }
}

#[op_interface_impl]
impl ToSpirvOp for LoopOp {
    fn to_spirv(&self, _ctx: &Context, _builder: &mut PlironBuilder) -> Result<()> {
        todo!();
    }
}

#[pliron_op(name = "spirv.Kill", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct KillOp;
canonical_format!(KillOp);
impl KillOp {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![], vec![], vec![], 0),
        }
    }
}

#[op_interface_impl]
impl ToSpirvOp for KillOp {
    fn to_spirv(&self, _ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        builder.kill().into_pliron_result()?;
        Ok(())
    }
}

#[pliron_op(name = "spirv.Return", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct ReturnOp;
canonical_format!(ReturnOp);
impl ReturnOp {
    pub fn new(ctx: &mut Context, value: Option<Value>) -> Self {
        Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![], flat_vec![value], vec![], 0),
        }
    }
}

#[op_interface_impl]
impl ToSpirvOp for ReturnOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        if let Some(value) = self.get_operation().deref(ctx).operands().next() {
            let value = builder.value_id(value);
            builder.ret_value(value).into_pliron_result()?;
        } else {
            builder.ret().into_pliron_result()?;
        }
        Ok(())
    }
}

#[pliron_op(name = "spirv.Unreachable", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct UnreachableOp;
canonical_format!(UnreachableOp);
impl UnreachableOp {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![], vec![], vec![], 0),
        }
    }
}

#[op_interface_impl]
impl ToSpirvOp for UnreachableOp {
    fn to_spirv(&self, _ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        builder.unreachable().into_pliron_result()?;
        Ok(())
    }
}

#[pliron_op(name = "spirv.TerminateInvocation", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct TerminateInvocationOp;
canonical_format!(TerminateInvocationOp);
impl TerminateInvocationOp {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![], vec![], vec![], 0),
        }
    }
}

#[op_interface_impl]
impl ToSpirvOp for TerminateInvocationOp {
    fn to_spirv(&self, _ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        builder.terminate_invocation().into_pliron_result()?;
        Ok(())
    }
}

#[pliron_op(name = "spirv.DemoteToHelperInvocation", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct DemoteToHelperInvocationOp;
canonical_format!(DemoteToHelperInvocationOp);
impl DemoteToHelperInvocationOp {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![], vec![], vec![], 0),
        }
    }
}

#[op_interface_impl]
impl ToSpirvOp for DemoteToHelperInvocationOp {
    fn to_spirv(&self, _ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        builder.demote_to_helper_invocation().into_pliron_result()?;
        Ok(())
    }
}

#[pliron_op(
    name = "spirv.module",
    attributes = (
        spirv_module_addressing_model: AddressingModelAttr,
        spirv_module_memory_model: MemoryModelAttr,
        spirv_module_vce: VerCapExtAttr,
    ),
    verifier = "succ"
)]
#[derive_op_interface_impl(
    SingleBlockRegionInterface,
    NRegionsInterface<1>,
    OneRegionInterface,
    SymbolOpInterface,
    NoTerminatorInterface,
    SymbolTableInterface,
    IsolatedFromAboveInterface
)]
pub struct SpirvModuleOp;
canonical_format!(SpirvModuleOp);
impl SpirvModuleOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        sym_name: Identifier,
        addressing_model: AddressingModel,
        memory_model: MemoryModel,
    ) -> Self {
        let op = Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![], vec![], vec![], 1),
        };

        op.set_symbol_name(ctx, sym_name);
        op.set_attr_spirv_module_addressing_model(ctx, addressing_model.into());
        op.set_attr_spirv_module_memory_model(ctx, memory_model.into());

        let region = op.get_region_i(ctx, 0);
        let block = BasicBlock::new(ctx, None, vec![]);
        block.insert_at_front(region, ctx);

        op
    }

    pub fn set_vce(&self, ctx: &Context, vce: VerCapExtAttr) {
        self.set_attr_spirv_module_vce(ctx, vce);
    }
}

#[op_interface_impl]
impl ToSpirvOp for SpirvModuleOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let addressing = self.get_attr_spirv_module_addressing_model(ctx).unwrap().0;
        let mem_model = self.get_attr_spirv_module_memory_model(ctx).unwrap().0;
        builder.memory_model(addressing, mem_model);

        let vce = self.get_attr_spirv_module_vce(ctx).map(|it| it.clone());
        let vce = vce.unwrap_or_default();

        builder.set_version(vce.version.0, vce.version.1);

        for extension in vce.extensions {
            builder.extension(extension);
        }

        for capability in vce.capabilities {
            builder.capability(capability);
        }

        let block = self.get_body(ctx, 0);
        let ops = block.deref(ctx).iter(ctx).collect::<Vec<_>>();
        for op in ops {
            let dyn_op = Operation::get_op_dyn(op, ctx);
            let Some(to_spirv) = op_cast::<dyn ToSpirvOp>(&*dyn_op) else {
                return verify_err!(
                    self.loc(ctx),
                    "All operations inside a `spirv.module must implement `ToSpirvOp`"
                );
            };
            to_spirv.to_spirv(ctx, builder)?;
        }

        Ok(())
    }
}

pub(crate) fn apply_all_decorations(ctx: &Context, builder: &mut PlironBuilder, op: &dyn DecoratableOp, id: Word) {
    for (decoration, args) in all_decorations(op, ctx) {
        builder.decorate(id, decoration, args);
    }
}
