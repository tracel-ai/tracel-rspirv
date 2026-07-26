use crate::{
    ops::{khr::PoisonOp, *},
    types::PointerType,
};
use alloc::{vec, vec::Vec};
use pliron::{
    arg_err,
    builtin::op_interfaces::OneResultInterface,
    context::Context,
    derive::{op_interface, op_interface_impl, type_interface},
    irbuild::{inserter::Inserter, rewriter::Rewriter},
    op::Op,
    opts::mem2reg::{AllocInfo, PromotableAllocationInterface, PromotableOpInterface, PromotableOpKind},
    result::Result,
    r#type::{Type, TypedHandle},
    value::Value,
};
use thiserror::Error;
use tracel_rspirv::spirv::{Capability, StorageClass};

/// Lists the required version, capabilities, and extensions required for this instance of this
/// operation. If the minimum version is `Some` and less than or equal to the target version, the
/// operation does not need any extensions. Otherwise, one of each set of extensions (see
/// [`required_extensions`](VerCapExtOpInterface::required_extensions)) is required.
#[op_interface]
pub trait VerCapExtOpInterface {
    /// Lists the minimum version from which an operation is *always available*. `None` means
    /// extensions are always required for the instruction or at least one operand.
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)>;
    /// Lists the set of extensions required for this instruction, if the minimum version is not met.
    /// The returned value is a nested vector whose element is the extension name. The outer
    /// vector's elements (which are vectors) should be interpreted as conjunction
    /// while the inner vector's elements (which are extension names)
    /// should be interpreted as disjunction. For example, given
    /// ```ignore
    /// [["Extension_A", "Extension_B"], ["Extension_C"], ["Extension_D", "Extension_E"]]
    /// ```
    /// The operation instance is available when (`"Extension_A"` OR `"Extension_B"`)
    /// AND (`"Extension_C"`) AND (`"Extension_D"` OR `"Extension_E"`) is enabled.
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>>;
    /// Lists the set of capabilities required for this instruction. Has the same semantics as
    /// [`required_extensions`](VerCapExtOpInterface::required_capabilities).
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>>;
    fn verify(_op: &dyn Op, _ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }
}

/// Lists the required version, capabilities, and extensions required for this instance of this
/// type. If the minimum version is `Some` and less than or equal to the target version, the
/// type does not need any extensions. Otherwise, one of each set of extensions (see
/// [`required_extensions`](VerCapExtTypeInterface::required_extensions)) is required.
#[type_interface]
pub trait VerCapExtTypeInterface {
    /// Lists the minimum version from which an operation is *always available*. `None` means
    /// extensions are always required for the instruction or at least one operand.
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)>;
    /// Lists the set of extensions required for this instruction, if the minimum version is not met.
    /// The returned value is a nested vector whose element is the extension name. The outer
    /// vector's elements (which are vectors) should be interpreted as conjunction
    /// while the inner vector's elements (which are extension names)
    /// should be interpreted as disjunction. For example, given
    /// ```ignore
    /// [["Extension_A", "Extension_B"], ["Extension_C"], ["Extension_D", "Extension_E"]]
    /// ```
    /// The operation instance is available when (`"Extension_A"` OR `"Extension_B"`)
    /// AND (`"Extension_C"`) AND (`"Extension_D"` OR `"Extension_E"`) is enabled.
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>>;
    /// Lists the set of capabilities required for this instruction. Has the same semantics as
    /// [`required_extensions`](VerCapExtTypeInterface::required_capabilities).
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>>;
    fn verify(_ty: &dyn Type, _ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }
}

#[macro_export]
macro_rules! NoSideEffects {
    ($ty: ty) => {
        #[::pliron::derive::op_interface_impl]
        impl pliron::opts::dce::SideEffects for $ty {
            fn has_side_effects(&self, _ctx: &Context) -> bool {
                false
            }
        }
    };
}

// Apply as needed, there are far too many to keep track and they're not marked in the spec
NoSideEffects!(InBoundsAccessChainOp);
NoSideEffects!(AccessChainOp);
NoSideEffects!(LoadOp);
NoSideEffects!(PoisonOp);
NoSideEffects!(AddressOfOp);
NoSideEffects!(CompositeExtractOp);

#[derive(Error, Debug)]
#[error("Register Promotion: Allocation info provided is not related to this operation")]
pub struct UnrelatedAllocInfo;

#[op_interface_impl]
impl PromotableOpInterface for LoadOp {
    fn promotion_kind(&self, ctx: &Context, alloc_info: &AllocInfo) -> PromotableOpKind {
        if self.get_operand_pointer(ctx) == alloc_info.ptr {
            PromotableOpKind::Load
        } else {
            PromotableOpKind::NonPromotableUse
        }
    }

    fn promote(
        &self,
        ctx: &mut Context,
        alloc_info_reaching_defs: &[(AllocInfo, Value)],
        rewriter: &mut dyn Rewriter,
    ) -> Result<()> {
        if alloc_info_reaching_defs.len() != 1 {
            return arg_err!(self.loc(ctx), UnrelatedAllocInfo);
        }
        let (alloc_info, reaching_def) = &alloc_info_reaching_defs[0];
        if self.get_operand_pointer(ctx) != alloc_info.ptr {
            return arg_err!(self.loc(ctx), UnrelatedAllocInfo);
        }
        rewriter.replace_operation_with_values(ctx, self.get_operation(), vec![*reaching_def]);
        Ok(())
    }
}

#[op_interface_impl]
impl PromotableOpInterface for StoreOp {
    fn promotion_kind(&self, ctx: &Context, alloc_info: &AllocInfo) -> PromotableOpKind {
        if self.get_operand_pointer(ctx) == alloc_info.ptr {
            PromotableOpKind::Store(self.get_operand_pointer(ctx))
        } else {
            PromotableOpKind::NonPromotableUse
        }
    }

    fn promote(
        &self,
        ctx: &mut Context,
        alloc_info_reaching_defs: &[(AllocInfo, Value)],
        rewriter: &mut dyn Rewriter,
    ) -> Result<()> {
        if alloc_info_reaching_defs.len() != 1 {
            return arg_err!(self.loc(ctx), UnrelatedAllocInfo);
        }
        let (alloc_info, _reaching_def) = &alloc_info_reaching_defs[0];
        if self.get_operand_pointer(ctx) != alloc_info.ptr {
            return arg_err!(self.loc(ctx), UnrelatedAllocInfo);
        }
        rewriter.erase_operation(ctx, self.get_operation());
        Ok(())
    }
}

#[op_interface_impl]
impl PromotableAllocationInterface for VariableOp {
    fn alloc_info(&self, ctx: &Context) -> Vec<AllocInfo> {
        if self.get_attr_storage_class(ctx).0 != StorageClass::Function {
            return vec![];
        }
        let ty = TypedHandle::<PointerType>::from_handle(self.result_type(ctx), ctx).expect("Should be pointer");
        vec![AllocInfo {
            ptr: self.get_result(ctx),
            ty: ty.deref(ctx).element_type,
        }]
    }

    fn default_value(&self, ctx: &mut Context, inserter: &mut dyn Inserter, alloc_info: &AllocInfo) -> Result<Value> {
        if alloc_info.ptr != self.get_result(ctx) {
            return arg_err!(self.loc(ctx), UnrelatedAllocInfo);
        }
        let initializer = self.get_operation().deref(ctx).operands().next();
        Ok(initializer.unwrap_or_else(|| {
            let poison = PoisonOp::new(ctx, alloc_info.ty);
            inserter.insert_op(ctx, &poison);
            poison.get_result(ctx)
        }))
    }

    fn promote(&self, ctx: &mut Context, rewriter: &mut dyn Rewriter, alloc_infos: &[AllocInfo]) -> Result<()> {
        if alloc_infos.len() != 1 || alloc_infos[0].ptr != self.get_result(ctx) {
            return arg_err!(self.loc(ctx), UnrelatedAllocInfo);
        }
        rewriter.erase_operation(ctx, self.get_operation());
        Ok(())
    }
}
