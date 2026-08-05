#![no_std]

extern crate alloc;
extern crate std;

use core::{
    cell::Ref,
    ops::{Deref, DerefMut},
};

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use pliron::{
    attribute::{AttrObj, Attribute},
    basic_block::BasicBlock,
    builtin::attributes::VecAttr,
    context::{Context, Ptr},
    derive::{attr_interface, op_interface, type_interface},
    identifier::Identifier,
    input_err_noloc,
    op::{Op, op_cast},
    operation::Operation,
    printable::Printable,
    result::Result,
    r#type::{Type, TypeHandle, type_cast},
    utils::table::{HMap, IMap},
    value::Value,
    verify_err,
    verify_error_noloc,
};
use thiserror::Error;
use tracel_rspirv::{
    dr::{Builder, Module},
    spirv::{MemoryAccess, Word},
};

pub use tracel_rspirv::spirv;

use crate::types::normalize_int_type;

mod autogen_attrs;
mod autogen_decorations;
mod autogen_ops;

pub mod attrs;
pub mod decorations;
pub mod ext;
mod format;
pub mod interfaces;
pub mod ops;
pub mod tensor_addressing_nv;
pub mod types;
pub mod util;

#[derive(Error, Debug)]
pub enum PlironSpirvError {
    #[error("Unresolved symbol reference @{_0}")]
    UnresolvedSymbol(Identifier),
    #[error("Global symbol @{_0} failed to be registered")]
    SymbolNotRegistered(Identifier),
}

#[derive(Default)]
pub struct PlironBuilder {
    builder: Builder,
    types: IMap<TypeHandle, Word>,
    constants: HMap<(TypeHandle, u64), Word>,
    values: HMap<Value, Word>,
    symbols: HMap<Identifier, Word>,
    blocks: HMap<(Ptr<BasicBlock>, BlockPos), Word>,
    strings: HMap<String, Word>,
}

impl Deref for PlironBuilder {
    type Target = Builder;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl DerefMut for PlironBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum BlockPos {
    Start,
    End,
}

impl PlironBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub(crate) fn value_id(&mut self, value: Value) -> Word {
        if let Some(existing) = self.values.get(&value) {
            *existing
        } else {
            let id = self.id();
            self.values.insert(value, id);
            id
        }
    }

    pub(crate) fn merge_values(&mut self, value_1: Value, value_2: Value) {
        match (self.values.get(&value_1), self.values.get(&value_2)) {
            (Some(_), Some(_)) => panic!("Both values already have IDs"),
            (Some(id), None) => self.values.insert(value_2, *id),
            (None, Some(id)) => self.values.insert(value_1, *id),
            (None, None) => {
                let id = self.value_id(value_1);
                self.values.insert(value_2, id)
            }
        };
    }

    pub(crate) fn symbol_id(&mut self, symbol: impl Into<Identifier>) -> Word {
        let sym = symbol.into();
        if let Some(existing) = self.symbols.get(&sym) {
            *existing
        } else {
            let id = self.id();
            self.symbols.insert(sym, id);
            id
        }
    }

    pub(crate) fn label_id(&mut self, block: Ptr<BasicBlock>, pos: BlockPos) -> Word {
        if let Some(existing) = self.blocks.get(&(block, pos)) {
            *existing
        } else {
            let id = self.id();
            self.blocks.insert((block, pos), id);
            id
        }
    }

    pub(crate) fn string_ref(&mut self, string: impl Into<String>) -> Word {
        let string = string.into();
        if let Some(existing) = self.strings.get(&string) {
            *existing
        } else {
            let id = self.string(string.clone());
            self.strings.insert(string, id);
            id
        }
    }

    /// Appends an OpConstant instruction with the given 32-bit bit pattern `value`.
    pub fn constant_bit32(&mut self, ctx: &Context, result_type: TypeHandle, value: u32) -> Result<Word> {
        let result_type = normalize_int_type(ctx, result_type);
        if let Some(existing) = self.constants.get(&(result_type, value as u64)) {
            Ok(*existing)
        } else {
            let ty = spirv_type_id(ctx, self, result_type)?;
            let id = self.builder.constant_bit32(ty, value);
            self.constants.insert((result_type, value as u64), id);
            Ok(id)
        }
    }

    /// Appends an OpConstant instruction with the given 64-bit bit pattern `value`.
    pub fn constant_bit64(&mut self, ctx: &Context, result_type: TypeHandle, value: u64) -> Result<Word> {
        let result_type = normalize_int_type(ctx, result_type);
        if let Some(existing) = self.constants.get(&(result_type, value)) {
            Ok(*existing)
        } else {
            let ty = spirv_type_id(ctx, self, result_type)?;
            let id = self.builder.constant_bit64(ty, value);
            self.constants.insert((result_type, value), id);
            Ok(id)
        }
    }

    pub fn module(self) -> Module {
        self.builder.module()
    }
}

pub trait IntoPlironResult<T> {
    fn into_pliron_result(self) -> Result<T>;
}

impl<T> IntoPlironResult<T> for core::result::Result<T, tracel_rspirv::dr::Error> {
    fn into_pliron_result(self) -> Result<T> {
        self.map_err(|err| verify_error_noloc!(err))
    }
}

impl IntoPlironResult<()> for () {
    fn into_pliron_result(self) -> Result<()> {
        Ok(())
    }
}

impl IntoPlironResult<u32> for u32 {
    fn into_pliron_result(self) -> Result<u32> {
        Ok(self)
    }
}

#[derive(Error, Debug)]
pub enum ToSpirvError {
    #[error("{_0} doesn't support conversion to SPIR-V")]
    UnsupportedOp(String),
    #[error("Found unsupported op {_0} after resolving symbol")]
    UnsupportedSymbolOp(String),
    #[error("{_0} doesn't support conversion to SPIR-V")]
    UnsupportedType(String),
}

#[op_interface]
pub trait ToSpirvOp {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()>;

    fn verify(_op: &dyn Op, _ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }
}

#[type_interface]
pub trait ToSpirvType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word>;

    fn verify(_ty: &dyn Type, _ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }
}

#[attr_interface]
pub trait ToSpirvAttr {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word>;

    fn verify(_attr: &dyn Attribute, _ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }
}

pub(crate) fn spirv_type_id(ctx: &Context, builder: &mut PlironBuilder, handle: TypeHandle) -> Result<Word> {
    let handle = normalize_int_type(ctx, handle);
    if let Some(existing) = builder.types.get(&handle) {
        Ok(*existing)
    } else {
        let ty = handle.deref(ctx);
        let Some(ty) = type_cast::<dyn ToSpirvType>(&*ty) else {
            let err = ToSpirvError::UnsupportedType(handle.disp(ctx).to_string());
            return input_err_noloc!(err);
        };
        let id = ty.to_spirv(ctx, builder)?;
        builder.types.insert(handle, id);
        Ok(id)
    }
}

pub(crate) fn op_to_spirv(ctx: &Context, builder: &mut PlironBuilder, op: Ptr<Operation>) -> Result<()> {
    let dyn_op = Operation::get_op_dyn(op, ctx);
    let Some(to_spirv) = op_cast::<dyn ToSpirvOp>(&*dyn_op) else {
        let error = ToSpirvError::UnsupportedOp(op.disp(ctx).to_string());
        return verify_err!(dyn_op.loc(ctx), error);
    };
    to_spirv.to_spirv(ctx, builder)
}

pub(crate) fn opt_memory_access(mem_access: MemoryAccess) -> Option<MemoryAccess> {
    if mem_access == MemoryAccess::NONE {
        None
    } else {
        Some(mem_access)
    }
}

pub(crate) fn as_vec_attr<T: Attribute>(values: impl IntoIterator<Item = T>) -> VecAttr {
    VecAttr(values.into_iter().map(|it| -> AttrObj { Box::new(it) }).collect())
}

pub(crate) fn from_vec_attr<T: Attribute>(attr: Ref<'_, VecAttr>) -> Vec<T> {
    attr.0
        .clone()
        .into_iter()
        .map(|it| *it.downcast::<T>().unwrap())
        .collect()
}

pub(crate) mod prelude {
    pub(crate) use crate::{
        IntoPlironResult,
        PlironBuilder,
        ToSpirvOp,
        as_vec_attr,
        attrs::*,
        decorations::DecoratableOp,
        from_vec_attr,
        interfaces::VerCapExtOpInterface,
        opt_memory_access,
        spirv_type_id,
        util::flat_vec,
    };
    pub(crate) use alloc::{
        string::{String, ToString},
        vec,
        vec::Vec,
    };
    pub(crate) use pliron::{
        builtin::{
            attributes::{IdentifierAttr, StringAttr, UnitAttr, VecAttr},
            op_interfaces::*,
        },
        context::Context,
        derive::*,
        identifier::Identifier,
        op::Op,
        operation::Operation,
        result::Result,
        r#type::{TypeHandle, Typed},
        value::Value,
    };
    pub(crate) use tracel_rspirv::{
        dr::Operand,
        spirv::{Capability, Decoration},
    };
}
