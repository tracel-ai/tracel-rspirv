#![no_std]

extern crate alloc;

use core::{
    cell::Ref,
    ops::{Deref, DerefMut},
};

use alloc::{boxed::Box, vec::Vec};
use pliron::{
    attribute::{AttrObj, Attribute},
    builtin::{attributes::VecAttr, op_interfaces::SymbolTableInterface},
    context::Context,
    derive::{attr_interface, op_interface, type_interface},
    identifier::Identifier,
    input_err_noloc,
    op::{Op, op_cast},
    operation::Operation,
    printable::Printable,
    result::Result,
    std_deps::hash::FxHashMap,
    r#type::{Type, TypeHandle, type_cast},
    value::Value,
    verify_error_noloc,
};
use thiserror::Error;
use tracel_rspirv::{
    dr::{Builder, Instruction},
    spirv::Word,
};

use crate::ops::SpirvModuleOp;

#[allow(unused_parens, unused_imports, unused_variables, clippy::all)]
mod autogen_attrs;
#[allow(unused_parens, unused_imports, unused_variables, clippy::all)]
mod autogen_decorations;
#[allow(unused_parens, unused_imports, unused_variables, clippy::all)]
mod autogen_ops;

pub mod attrs;
pub mod decorations;
pub mod ext;
pub mod ops;
pub mod tensor_addressing_nv;
pub mod types;
pub mod util;

pub(crate) mod parse;

#[derive(Error, Debug)]
pub enum PlironSpirvError {
    #[error("Unresolved symbol reference @{_0}")]
    UnresolvedSymbol(Identifier),
    #[error("Global symbol @{_0} failed to be registered")]
    SymbolNotRegistered(Identifier),
}

pub struct PlironBuilder {
    builder: Builder,
    module: SpirvModuleOp,
    types: FxHashMap<TypeHandle, Instruction>,
    constants: FxHashMap<(TypeHandle, u64), Word>,
    values: FxHashMap<Value, Word>,
    symbols: FxHashMap<Identifier, Word>,
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

impl PlironBuilder {
    pub(crate) fn value_id(&mut self, value: Value) -> Word {
        if let Some(existing) = self.values.get(&value) {
            *existing
        } else {
            let id = self.id();
            self.values.insert(value, id);
            id
        }
    }

    /// Appends an OpConstant instruction with the given 32-bit bit pattern `value`.
    pub fn constant_bit32(&mut self, ctx: &Context, result_type: TypeHandle, value: u32) -> Result<Word> {
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
        if let Some(existing) = self.constants.get(&(result_type, value)) {
            Ok(*existing)
        } else {
            let ty = spirv_type_id(ctx, self, result_type)?;
            let id = self.builder.constant_bit64(ty, value);
            self.constants.insert((result_type, value), id);
            Ok(id)
        }
    }

    pub fn finalize(mut self) -> Builder {
        for (_, ty) in self.types {
            self.builder.module_mut().types_global_values.push(ty);
        }

        self.builder
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
    fn get(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let handle = self.get_self_handle(ctx);
        if let Some(existing) = builder.types.get(&handle) {
            Ok(existing.result_id.expect("Should have result"))
        } else {
            let inst = self.to_spirv(ctx, builder)?;
            let id = inst.result_id.expect("Should have result");
            builder.types.insert(handle, inst);
            Ok(id)
        }
    }
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Instruction>;

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
    let ty = handle.deref(ctx);
    let Some(ty) = type_cast::<dyn ToSpirvType>(&*ty) else {
        return input_err_noloc!("Found unsupported type {} in SPIR-V op", handle.disp(ctx));
    };
    ty.get(ctx, builder)
}

pub(crate) fn spirv_symbol_id(ctx: &Context, builder: &mut PlironBuilder, sym: impl Into<Identifier>) -> Result<Word> {
    let sym = sym.into();
    if let Some(existing) = builder.symbols.get(&sym) {
        Ok(*existing)
    } else {
        let op = builder
            .module
            .lookup(ctx, &sym)
            .ok_or_else(|| verify_error_noloc!(PlironSpirvError::UnresolvedSymbol(sym.clone())))?;
        let dyn_op = Operation::get_op_dyn(op, ctx);
        let Some(ty) = op_cast::<dyn ToSpirvOp>(&*dyn_op) else {
            return input_err_noloc!("Found unsupported op {} after resolving symbol", op.disp(ctx));
        };
        ty.to_spirv(ctx, builder)?; // Global identifiers should register themselves in the transform
        let id = *builder
            .symbols
            .get(&sym)
            .ok_or_else(|| verify_error_noloc!(PlironSpirvError::SymbolNotRegistered(sym)))?;
        Ok(id)
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
        spirv_symbol_id,
        spirv_type_id,
        util::flat_vec,
    };
    pub(crate) use alloc::{string::ToString, vec, vec::Vec};
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
    pub(crate) use tracel_rspirv::{dr::Operand, spirv::Decoration};
}
