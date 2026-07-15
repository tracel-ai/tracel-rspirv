use alloc::{boxed::Box, string::String};
use pliron::{
    attribute::{AttrObj, attr_cast},
    basic_block::BasicBlock,
    builtin::{
        attributes::{IdentifierAttr, IntegerAttr, TypeAttr},
        ops::{ConstantOp, FuncOp},
        type_interfaces::FunctionTypeInterface,
        types::{FunctionType, IntegerType},
    },
    context::Ptr,
    dict_key,
    graph::ControlFlowGraph,
    identifier::Identifier,
    irbuild::{IRStatus, rewriter::Rewriter},
    linked_list::ContainsLinkedList,
    op::op_cast,
    opts::constants::BranchOpFoldInterface,
    region::Region,
    r#type::TypedHandle,
    verify_err,
};
use tracel_rspirv::spirv::{
    AddressingModel,
    Capability,
    ExecutionMode,
    FunctionControl,
    MemoryModel,
    SelectionControl,
    Word,
};

use crate::{
    ToSpirvAttr,
    autogen_attrs::StorageClassAttr,
    decorations::all_decorations_for_op,
    format::{FormatVar, Quantifier, attr, canonical_format},
    op_to_spirv,
    prelude::*,
    types::PointerType,
};

pub use crate::autogen_ops::*;

#[pliron_op(
    name = "spirv.GlobalVariable",
    attributes = (
        spirv_global_variable_type:TypeAttr,
        spirv_global_variable_storage_class:StorageClassAttr,
        spirv_global_variable_initializer:IdentifierAttr,
    ),
    verifier = "succ"
)]
#[derive_op_interface_impl(SymbolOpInterface, DecoratableOp)]
pub struct GlobalVariableOp;
impl GlobalVariableOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        r#type: TypeHandle,
        storage_class: impl Into<StorageClassAttr>,
        sym_name: Identifier,
        initializer: Option<Identifier>,
    ) -> Self {
        let storage_class = storage_class.into();
        let result_ty = PointerType::get(ctx, r#type, storage_class.0).to_handle();
        let op = Self {
            op: Operation::new(ctx, Self::get_concrete_op_info(), vec![result_ty], vec![], vec![], 0),
        };
        op.set_attr_spirv_global_variable_type(ctx, r#type.into());
        op.set_attr_spirv_global_variable_storage_class(ctx, storage_class);
        op.set_symbol_name(ctx, sym_name);
        if let Some(initializer) = initializer {
            op.set_attr_spirv_global_variable_initializer(ctx, IdentifierAttr::new(initializer));
        }
        op
    }
}

canonical_format!(GlobalVariableOp;
    attr!(
        &global_variable_op_attr_names::ATTR_KEY_SPIRV_GLOBAL_VARIABLE_TYPE,
        TypeAttr,
        "type",
        Quantifier::One,
    ),
    attr!(
        &global_variable_op_attr_names::ATTR_KEY_SPIRV_GLOBAL_VARIABLE_STORAGE_CLASS,
        StorageClassAttr,
        "storage_class",
        Quantifier::One,
    ),
    attr!(
        &global_variable_op_attr_names::ATTR_KEY_SPIRV_GLOBAL_VARIABLE_INITIALIZER,
        IdentifierAttr,
        "init",
        Quantifier::ZeroOrOne,
    )
);

#[op_interface_impl]
impl ToSpirvOp for GlobalVariableOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let element_type = self.get_attr_spirv_global_variable_type(ctx).unwrap();
        let storage_class = self.get_attr_spirv_global_variable_storage_class(ctx).unwrap().0;
        let initializer = self
            .get_attr_spirv_global_variable_initializer(ctx)
            .map(|sym| builder.symbol_id(sym.clone()));

        let ty = PointerType::get(ctx, element_type.get_type(ctx), storage_class).to_handle();
        let result_ty = spirv_type_id(ctx, builder, ty)?;
        let result = builder.symbol_id(self.get_symbol_name(ctx));

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

canonical_format!(ExecutionModeOp;
    FormatVar::Symbol(
        &execution_mode_op_attr_names::ATTR_KEY_SPIRV_EXECUTION_MODE_ENTRY_POINT,
        "entry_point",
        Quantifier::One,
    ),
    attr!(
        &execution_mode_op_attr_names::ATTR_KEY_SPIRV_EXECUTION_MODE_EXECUTION_MODE,
        ExecutionModeAttr,
        "execution_mode",
        Quantifier::One,
    ),
    attr!(
        &execution_mode_op_attr_names::ATTR_KEY_SPIRV_EXECUTION_MODE_ARGUMENTS,
        LiteralIntegerAttr,
        "arguments",
        Quantifier::ZeroOrMore,
    )
);

#[op_interface_impl]
impl ToSpirvOp for ExecutionModeOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let entry_point = builder.symbol_id(self.entry_point(ctx));
        let mode = self.mode(ctx);
        let args = self.arguments(ctx);
        builder.execution_mode(entry_point, mode, args);
        Ok(())
    }
}

#[op_interface_impl]
impl VerCapExtOpInterface for ExecutionModeOp {
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        Operand::from(self.mode(ctx)).minimum_version()
    }

    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        Operand::from(self.mode(ctx)).required_extensions()
    }

    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        Operand::from(self.mode(ctx)).required_capabilities()
    }
}

#[pliron_op(
    name = "spirv.ExecutionModeId",
    format,
    attributes = (
        spirv_execution_mode_id_entry_point: IdentifierAttr,
        spirv_execution_mode_id_execution_mode: ExecutionModeAttr,
    ),
    verifier = "succ"
)]
pub struct ExecutionModeIdOp;
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
        op.set_attr_spirv_execution_mode_id_entry_point(ctx, IdentifierAttr::new(entry_point));
        op.set_attr_spirv_execution_mode_id_execution_mode(ctx, mode.into());
        op
    }

    pub fn entry_point(&self, ctx: &Context) -> Identifier {
        self.get_attr_spirv_execution_mode_id_entry_point(ctx)
            .unwrap()
            .clone()
            .into()
    }

    pub fn mode(&self, ctx: &Context) -> ExecutionMode {
        self.get_attr_spirv_execution_mode_id_execution_mode(ctx)
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
        let entry_point = builder.symbol_id(self.entry_point(ctx));
        let mode = self.mode(ctx);
        let args = op.operands().map(|val| builder.value_id(val)).collect::<Vec<_>>();
        builder.execution_mode_id(entry_point, mode, args);
        Ok(())
    }
}

#[op_interface_impl]
impl VerCapExtOpInterface for ExecutionModeIdOp {
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        Operand::from(self.mode(ctx)).minimum_version()
    }

    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        Operand::from(self.mode(ctx)).required_extensions()
    }

    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        Operand::from(self.mode(ctx)).required_capabilities()
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
        builder.values.insert(self.get_result(ctx), id);
        apply_all_decorations(ctx, builder, self, id);
        Ok(())
    }
}

#[pliron_op(
    name = "spirv.pliron.addressof",
    format = "`@` attr($spirv_addressof_variable, $IdentifierAttr) ` : ` type($0)",
    attributes = (
        spirv_addressof_variable: IdentifierAttr,
    ),
    verifier = "succ"
)]
#[derive_op_interface_impl(NResultsInterface<1>, OneResultInterface)]
pub struct AddressOfOp;
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
        let id = builder.symbol_id(symbol);
        builder.values.insert(self.get_result(ctx), id);
        Ok(())
    }
}

#[pliron_op(name = "spirv.pliron.merge", format = "operands(CharSpace(`,`))", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct MergeOp;
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
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let op = self.get_operation();
        let parent_op = op.deref(ctx).get_parent_op(ctx).expect("Should have parent");

        let yield_values = op.deref(ctx).operands().collect::<Vec<_>>();
        let parent_results = parent_op.deref(ctx).results().collect::<Vec<_>>();

        if yield_values.len() != parent_results.len() {
            return verify_err!(self.loc(ctx), "Merge operands don't match parent results");
        }

        for (opd, parent_res) in yield_values.into_iter().zip(parent_results) {
            let id = builder.value_id(opd);
            builder.values.insert(parent_res, id);
        }

        Ok(())
    }
}

#[pliron_op(
    name = "spirv.Branch",
    format = "succ($0) `(` operands(CharSpace(`,`)) `)`",
    verifier = "succ"
)]
#[derive_op_interface_impl(IsTerminatorInterface, NResultsInterface<0>, NSuccsInterface<1>, OneSuccInterface)]
pub struct BranchOp;
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
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let label_id = builder.label_id(self.get_successor(ctx));
        builder.branch(label_id).into_pliron_result()
    }
}

#[pliron_op(
    name = "spirv.BranchConditional",
    format,
    operands = (condition: IntegerType, true_dest_opds, false_dest_opds),
    verifier = "succ"
)]
#[derive_op_interface_impl(IsTerminatorInterface, NResultsInterface<0>, NSuccsInterface<2>, OperandSegmentInterface)]
pub struct BranchConditionalOp;
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

    pub fn true_dest(&self, ctx: &Context) -> Ptr<BasicBlock> {
        self.get_operation().deref(ctx).get_successor(0)
    }

    pub fn false_dest(&self, ctx: &Context) -> Ptr<BasicBlock> {
        self.get_operation().deref(ctx).get_successor(1)
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

#[op_interface_impl]
impl ToSpirvOp for BranchConditionalOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let condition = builder.value_id(self.get_operand_condition(ctx));
        let true_label_id = builder.label_id(self.true_dest(ctx));
        let false_label_id = builder.label_id(self.false_dest(ctx));
        builder
            .branch_conditional(condition, true_label_id, false_label_id, [])
            .into_pliron_result()
    }
}

#[pliron_op(name = "spirv.pliron.selection", format = "region($0)", verifier = "succ")]
#[derive_op_interface_impl(NRegionsInterface<1>, OneRegionInterface)]
pub struct SelectionOp;
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
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let region = self.region(ctx);
        let entry_block = self.entry_block(ctx);
        let merge_block = region.deref(ctx).get_tail().unwrap();
        let merge_label = builder.label_id(merge_block);
        let blocks = region.deref(ctx).iter(ctx).skip(1).collect::<Vec<_>>();

        builder
            .selection_merge(merge_label, SelectionControl::NONE)
            .into_pliron_result()?;
        for op in entry_block.deref(ctx).iter(ctx) {
            op_to_spirv(ctx, builder, op)?;
        }
        for block in blocks {
            block_to_spirv(ctx, builder, block, false, block == merge_block)?;
        }
        Ok(())
    }
}

#[pliron_op(name = "spirv.pliron.loop", format = "region($0)", verifier = "succ")]
#[derive_op_interface_impl(NRegionsInterface<1>, OneRegionInterface)]
pub struct LoopOp;
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

#[pliron_op(name = "spirv.Kill", format = "", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct KillOp;
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

#[pliron_op(name = "spirv.Return", format = "operands(CharSpace(`,`))", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct ReturnOp;
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

#[pliron_op(name = "spirv.Unreachable", format = "", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct UnreachableOp;
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

#[pliron_op(name = "spirv.TerminateInvocation", format = "", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct TerminateInvocationOp;
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

#[pliron_op(name = "spirv.DemoteToHelperInvocation", format = "", verifier = "succ")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct DemoteToHelperInvocationOp;
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
    format = "
    attr($spirv_module_addressing_model, $AddressingModelAttr) ` `
    attr($spirv_module_memory_model, $MemoryModelAttr) ` `
    opt_attr($spirv_module_vce, $VerCapExtAttr, label($requires)) ` `
    region($0)",
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

        let region = op.get_region(ctx);
        let block = BasicBlock::new(ctx, None, vec![]);
        block.insert_at_front(region, ctx);

        op
    }

    pub fn addressing_model(&self, ctx: &Context) -> AddressingModel {
        self.get_attr_spirv_module_addressing_model(ctx).unwrap().0
    }

    pub fn memory_model(&self, ctx: &Context) -> MemoryModel {
        self.get_attr_spirv_module_memory_model(ctx).unwrap().0
    }

    pub fn get_vce<'a>(&self, ctx: &'a Context) -> VerCapExtAttr {
        self.get_attr_spirv_module_vce(ctx)
            .map(|it| it.clone())
            .unwrap_or_default()
    }

    pub fn set_vce(&self, ctx: &Context, vce: VerCapExtAttr) {
        self.set_attr_spirv_module_vce(ctx, vce);
    }

    pub fn has_capability(&self, ctx: &Context, cap: &Capability) -> bool {
        self.get_attr_spirv_module_vce(ctx)
            .is_some_and(|vce| vce.capabilities.contains(cap))
    }

    pub fn has_extension(&self, ctx: &Context, ext: impl Into<String>) -> bool {
        let ident = Identifier::try_from(ext.into()).expect("Should be valid ident");
        self.get_attr_spirv_module_vce(ctx)
            .is_some_and(|vce| vce.extensions.contains(&ident))
    }

    pub fn insert_capability(&self, ctx: &Context, cap: Capability) {
        let mut vce = self.get_vce(ctx);
        if !vce.capabilities.contains(&cap) {
            vce.capabilities.push(cap);
            self.set_vce(ctx, vce);
        }
    }

    pub fn insert_extension(&self, ctx: &Context, ext: impl Into<String>) {
        let ident = Identifier::try_from(ext.into()).expect("Should be valid ident");
        let mut vce = self.get_vce(ctx);
        if !vce.extensions.contains(&ident) {
            vce.extensions.push(ident);
            self.set_vce(ctx, vce);
        }
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
            op_to_spirv(ctx, builder, op)?;
        }

        Ok(())
    }
}

#[op_interface_impl]
impl VerCapExtOpInterface for SpirvModuleOp {
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        let result = Operand::from(self.addressing_model(ctx)).minimum_version()?;
        Some(result.max(Operand::from(self.memory_model(ctx)).minimum_version()?))
    }

    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        let mut result = Operand::from(self.addressing_model(ctx)).required_extensions();
        result.extend(Operand::from(self.memory_model(ctx)).required_extensions());
        result
    }

    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        let mut result = Operand::from(self.addressing_model(ctx)).required_capabilities();
        result.extend(Operand::from(self.memory_model(ctx)).required_capabilities());
        result
    }
}

dict_key!(ATTR_FUNCTION_CONTROL, "spirv_function_control");

pub trait FunctionControlInterface {
    fn get_function_control(&self, ctx: &Context) -> FunctionControl;
    fn set_function_control(&self, ctx: &Context, control: impl Into<FunctionControlAttr>);
}

impl FunctionControlInterface for FuncOp {
    fn get_function_control(&self, ctx: &Context) -> FunctionControl {
        let op = self.get_operation().deref(ctx);
        let func_control = op.attributes.get::<FunctionControlAttr>(&ATTR_FUNCTION_CONTROL);
        func_control.map(|it| it.0).unwrap_or(FunctionControl::NONE)
    }

    fn set_function_control(&self, ctx: &Context, control: impl Into<FunctionControlAttr>) {
        let mut op = self.get_operation().deref_mut(ctx);
        op.attributes.set(ATTR_FUNCTION_CONTROL.clone(), control.into());
    }
}

#[op_interface_impl]
impl ToSpirvOp for FuncOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
        let entry = self.get_entry_block(ctx);
        let args = entry.deref(ctx).arguments().collect::<Vec<_>>();
        let func_ty =
            TypedHandle::<FunctionType>::from_handle(self.get_type(ctx), ctx).expect("Should be `FunctionType`");

        let func_ty_id = spirv_type_id(ctx, builder, self.get_type(ctx))?;
        let return_ty = spirv_type_id(ctx, builder, func_ty.deref(ctx).res_types()[0])?;
        let func_control = self.get_function_control(ctx);

        let func_id = builder.symbol_id(self.get_symbol_name(ctx));
        builder
            .begin_function(return_ty, Some(func_id), func_control, func_ty_id)
            .into_pliron_result()?;

        for &arg in args.iter() {
            let ty = spirv_type_id(ctx, builder, arg.get_type(ctx))?;
            builder.function_parameter(ty).into_pliron_result()?;
        }

        let blocks = self.get_region(ctx).deref(ctx).iter(ctx).collect::<Vec<_>>();
        for block in blocks {
            block_to_spirv(ctx, builder, block, block == entry, false)?;
        }

        builder.end_function().into_pliron_result()?;

        Ok(())
    }
}

#[inline(never)]
pub(crate) fn apply_all_decorations(ctx: &Context, builder: &mut PlironBuilder, op: &dyn DecoratableOp, id: Word) {
    for (decoration, args) in all_decorations_for_op(op, ctx) {
        builder.decorate(id, decoration, args);
    }
}

pub(crate) fn block_to_spirv(
    ctx: &Context,
    builder: &mut PlironBuilder,
    block: Ptr<BasicBlock>,
    is_entry: bool,
    skip_end: bool,
) -> Result<()> {
    let label_id = builder.label_id(block);
    builder.begin_block(Some(label_id)).into_pliron_result()?;

    let args = block.deref(ctx).arguments().collect::<Vec<_>>();
    if !is_entry && !args.is_empty() {
        let predecessors = block.preds(ctx);
        let mut pred_args = vec![Vec::new(); args.len()];
        for pred in predecessors {
            let branch = pred.deref(ctx).get_terminator(ctx).expect("Should have terminator");
            let idx = branch.deref(ctx).successors().position(|it| it == block).unwrap();
            let branch = Operation::get_op_dyn(branch, ctx);
            let branch = op_cast::<dyn BranchOpInterface>(&*branch).expect("Should be branch op");
            let arg_operands = branch.successor_operands(ctx, idx);
            for (i, opd) in arg_operands.into_iter().enumerate() {
                pred_args[i].push((builder.value_id(opd), builder.label_id(block)));
            }
        }

        for (arg, sources) in args.into_iter().zip(pred_args) {
            let result_id = builder.value_id(arg);
            let result_type = spirv_type_id(ctx, builder, arg.get_type(ctx))?;
            builder
                .phi(result_type, Some(result_id), sources)
                .into_pliron_result()?;
        }
    }

    let ops = block.deref(ctx).iter(ctx).collect::<Vec<_>>();
    for op in ops {
        op_to_spirv(ctx, builder, op)?;
    }

    if !skip_end {
        builder.select_block(None).into_pliron_result()?;
    }
    Ok(())
}
