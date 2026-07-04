use core::iter;

use pliron::builtin::{
    type_interfaces::FunctionTypeInterface,
    types::{FunctionType, IntegerType, Signedness, UnitType},
};
use tracel_rspirv::{
    dr::{Instruction, Operand},
    spirv::{self, Decoration, FPEncoding, StorageClass},
};

use crate::{ToSpirvType, prelude::*};

fn type_inst(op: spirv::Op, id: u32, args: impl IntoIterator<Item = Operand>) -> Instruction {
    Instruction::new(op, None, Some(id), args.into_iter().collect())
}

#[type_interface_impl]
impl ToSpirvType for IntegerType {
    fn to_spirv(&self, _ctx: &Context, builder: &mut PlironBuilder) -> Result<Instruction> {
        let id = builder.id();
        if self.width() == 1 {
            Ok(type_inst(spirv::Op::TypeBool, id, []))
        } else {
            let sign = match self.signedness() {
                Signedness::Signed => 1u32,
                Signedness::Unsigned | Signedness::Signless => 0u32,
            };
            let args = [self.width().into(), sign.into()];
            Ok(type_inst(spirv::Op::TypeInt, id, args))
        }
    }
}

#[type_interface_impl]
impl ToSpirvType for UnitType {
    fn to_spirv(&self, _ctx: &Context, builder: &mut PlironBuilder) -> Result<Instruction> {
        let id = builder.id();
        Ok(type_inst(spirv::Op::TypeVoid, id, []))
    }
}

#[type_interface_impl]
impl ToSpirvType for FunctionType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Instruction> {
        let id = builder.id();
        let res_types = self.res_types();
        let result_ty = res_types.first().expect("Should have return type");
        let args = iter::once(*result_ty)
            .chain(self.arg_types())
            .map(|ty| Ok(Operand::IdRef(spirv_type_id(ctx, builder, ty)?)))
            .collect::<Result<Vec<_>>>()?;
        Ok(type_inst(spirv::Op::TypeFunction, id, args))
    }
}

#[pliron_type(
    name = "spirv.float",
    format = "$width opt($encoding)",
    generate_get = true,
    verifier = "succ"
)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FloatType {
    pub width: u32,
    pub encoding: Option<FPEncoding>,
}

#[type_interface_impl]
impl ToSpirvType for FloatType {
    fn to_spirv(&self, _ctx: &Context, builder: &mut PlironBuilder) -> Result<Instruction> {
        let id = builder.id();
        let mut args = vec![self.width.into()];
        args.extend(self.encoding.map(Operand::FPEncoding));
        Ok(type_inst(spirv::Op::TypeFloat, id, args))
    }
}

#[pliron_type(
    name = "spirv.vector",
    format = "`<` $count `x` $element_type `>`",
    generate_get = true,
    verifier = "succ"
)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VectorType {
    pub count: u32,
    pub element_type: TypeHandle,
}

#[type_interface_impl]
impl ToSpirvType for VectorType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Instruction> {
        let id = builder.id();
        let element_ty = Operand::IdRef(spirv_type_id(ctx, builder, self.element_type)?);
        let args = [element_ty, self.count.into()];
        Ok(type_inst(spirv::Op::TypeVector, id, args))
    }
}

/// Long vector from LongVectorEXT
#[pliron_type(
    name = "spirv.long_vector",
    format = "`<` $count `x` $element_type `>`",
    generate_get = true,
    verifier = "succ"
)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LongVectorType {
    pub count: u32,
    pub element_type: TypeHandle,
}

#[type_interface_impl]
impl ToSpirvType for LongVectorType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Instruction> {
        let id = builder.id();
        let element_ty = Operand::IdRef(spirv_type_id(ctx, builder, self.element_type)?);
        let u32 = IntegerType::get(ctx, 32, Signedness::Signless).to_handle();
        let count = Operand::IdRef(builder.constant_bit32(ctx, u32, self.count)?);
        let args = [element_ty, count];
        Ok(type_inst(spirv::Op::TypeVectorIdEXT, id, args))
    }
}

#[pliron_type(
    name = "spirv.array",
    format = "`<` $count `x` $element_type opt($stride) `>`",
    generate_get = true,
    verifier = "succ"
)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub count: u32,
    pub element_type: TypeHandle,
    pub stride: Option<u32>,
}

#[type_interface_impl]
impl ToSpirvType for ArrayType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Instruction> {
        let id = builder.id();
        if let Some(stride) = self.stride {
            builder.decorate(id, Decoration::ArrayStride, [stride.into()]);
        }

        let element_ty = Operand::IdRef(spirv_type_id(ctx, builder, self.element_type)?);
        let u32 = IntegerType::get(ctx, 32, Signedness::Signless).to_handle();
        let count = Operand::IdRef(builder.constant_bit32(ctx, u32, self.count)?);
        let args = [element_ty, count];
        Ok(type_inst(spirv::Op::TypeArray, id, args))
    }
}

#[pliron_type(
    name = "spirv.ptr",
    format = "`<` $element_type `, ` $storage_class `>`",
    generate_get = true,
    verifier = "succ"
)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PointerType {
    pub element_type: TypeHandle,
    pub storage_class: StorageClass,
}

#[type_interface_impl]
impl ToSpirvType for PointerType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Instruction> {
        let id = builder.id();
        let element_ty = spirv_type_id(ctx, builder, self.element_type)?;
        let args = [Operand::StorageClass(self.storage_class), Operand::IdRef(element_ty)];
        Ok(type_inst(spirv::Op::TypePointer, id, args))
    }
}

#[pliron_type(
    name = "spirv.runtime_array",
    format = "`<` $element_type opt($stride) `>`",
    generate_get = true,
    verifier = "succ"
)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RuntimeArrayType {
    pub element_type: TypeHandle,
    pub stride: Option<u32>,
}

#[type_interface_impl]
impl ToSpirvType for RuntimeArrayType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Instruction> {
        let id = builder.id();
        if let Some(stride) = self.stride {
            builder.decorate(id, Decoration::ArrayStride, [stride.into()]);
        }

        let element_ty = spirv_type_id(ctx, builder, self.element_type)?;
        let args = [Operand::IdRef(element_ty)];
        Ok(type_inst(spirv::Op::TypeRuntimeArray, id, args))
    }
}

// pub struct MemberDecorationInfo {
//     index: u32,
//     decoration: Decoration,
//     value: Option<AttrObj>,
// }

// #[pliron_type(name = "spirv.struct", generate_get = true, verifier = "succ")]
// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct StructType {
//     field_types: Vec<TypeHandle>,
//     offsets: Vec<u32>,
// }

// #[type_interface_impl]
// impl ToSpirvType for StructType {
//     fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Instruction> {
//         let id = builder.id();

//         let field_types = self
//             .field_types
//             .iter()
//             .map(|ty| Ok(Operand::IdRef(spirv_type_id(ctx, builder, *ty)?)))
//             .collect::<Result<Vec<_>>>()?;

//         let args = field_types.into_iter().map(Operand::IdRef);
//         Ok(type_inst(spirv::Op::TypeRuntimeArray, id, args))
//     }
// }
