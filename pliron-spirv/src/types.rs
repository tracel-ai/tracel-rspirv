use core::fmt::Write;

use ::pliron::{context::Context, r#type::TypedHandle};
use alloc::string::String;
use derive_new::new;
use pliron::{
    attribute::AttributeDict,
    builtin::{
        type_interfaces::FunctionTypeInterface,
        types::{FunctionType, IntegerType, Signedness, UnitType},
    },
    combine::{
        Parser,
        between,
        optional,
        parser::char::{char, string},
    },
    irfmt::parsers::{self, delimited_list_parser, spaced, type_parser},
    parsable::{IntoParseResult, Parsable, ParseResult, StateStream},
    printable::{self, Printable},
    r#type::type_cast,
};
use tracel_rspirv::{
    dr::Operand,
    spirv::{Decoration, FPEncoding, StorageClass, Word},
};

use crate::{
    ToSpirvType,
    decorations::{DecorationExt, DecorationInfo, decoration_for_key, decorations_parser, print_decorations},
    interfaces::VerCapExtTypeInterface,
    prelude::*,
};

/// Deduplicate int types since SPIR-V uses the same representation for unsigned and signless
pub(super) fn normalize_int_type(ctx: &Context, ty: TypeHandle) -> TypeHandle {
    if let Ok(int_ty) = TypedHandle::<IntegerType>::from_handle(ty, ctx)
        && int_ty.deref(ctx).is_unsigned()
    {
        IntegerType::get(ctx, int_ty.deref(ctx).width(), Signedness::Signless).to_handle()
    } else if let Ok(vector_ty) = TypedHandle::<VectorType>::from_handle(ty, ctx) {
        let inner = normalize_int_type(ctx, vector_ty.deref(ctx).element_type);
        VectorType::get(ctx, vector_ty.deref(ctx).count, inner).to_handle()
    } else if let Ok(ptr_ty) = TypedHandle::<PointerType>::from_handle(ty, ctx) {
        let inner = normalize_int_type(ctx, ptr_ty.deref(ctx).element_type);
        PointerType::get(ctx, inner, ptr_ty.deref(ctx).storage_class).to_handle()
    } else if let Ok(arr_ty) = TypedHandle::<ArrayType>::from_handle(ty, ctx) {
        let inner = normalize_int_type(ctx, arr_ty.deref(ctx).element_type);
        ArrayType::get(ctx, arr_ty.deref(ctx).count, inner, arr_ty.deref(ctx).stride).to_handle()
    } else if let Ok(arr_ty) = TypedHandle::<RuntimeArrayType>::from_handle(ty, ctx) {
        let inner = normalize_int_type(ctx, arr_ty.deref(ctx).element_type);
        RuntimeArrayType::get(ctx, inner, arr_ty.deref(ctx).stride).to_handle()
    } else {
        ty
    }
}

#[type_interface_impl]
impl ToSpirvType for IntegerType {
    fn to_spirv(&self, _ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        if self.width() == 1 {
            Ok(builder.type_bool())
        } else {
            let sign = match self.signedness() {
                Signedness::Signed => 1u32,
                Signedness::Unsigned | Signedness::Signless => 0u32,
            };
            Ok(builder.type_int(self.width(), sign))
        }
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for IntegerType {
    fn min_version(&self, _ctx: &Context) -> Option<(u8, u8)> {
        Some((1, 0))
    }
    fn required_extensions(&self, _ctx: &Context) -> Vec<Vec<&'static str>> {
        vec![]
    }
    fn required_capabilities(&self, _ctx: &Context) -> Vec<Vec<Capability>> {
        let mut out = vec![];
        match self.width() {
            64 => out.push(vec![Capability::Int64]),
            16 => out.push(vec![Capability::Int16]),
            8 => out.push(vec![Capability::Int8]),
            _ => {}
        }
        out
    }
}

#[type_interface_impl]
impl ToSpirvType for UnitType {
    fn to_spirv(&self, _ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        Ok(builder.type_void())
    }
}

#[type_interface_impl]
impl ToSpirvType for FunctionType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let res_types = self.res_types();
        let result_ty = spirv_type_id(ctx, builder, *res_types.first().expect("Should have return type"))?;
        let args = self
            .arg_types()
            .into_iter()
            .map(|ty| spirv_type_id(ctx, builder, ty))
            .collect::<Result<Vec<_>>>()?;
        Ok(builder.type_function(result_ty, args))
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for FunctionType {
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        let mut out = (1, 0);
        for ty in self.arg_types().iter().chain(self.res_types().iter()) {
            let ty = ty.deref(ctx);
            if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*ty) {
                out = out.max(ver_cap.min_version(ctx)?);
            }
        }
        Some(out)
    }
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        let mut out = vec![];
        for ty in self.arg_types().iter().chain(self.res_types().iter()) {
            let ty = ty.deref(ctx);
            if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*ty) {
                out.extend(ver_cap.required_extensions(ctx));
            }
        }
        out
    }
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        let mut out = vec![];
        for ty in self.arg_types().iter().chain(self.res_types().iter()) {
            let ty = ty.deref(ctx);
            if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*ty) {
                out.extend(ver_cap.required_capabilities(ctx));
            }
        }
        out
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
    fn to_spirv(&self, _ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let mut args = vec![self.width.into()];
        args.extend(self.encoding.map(Operand::FPEncoding));
        Ok(builder.type_float(self.width, self.encoding))
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for FloatType {
    fn min_version(&self, _ctx: &Context) -> Option<(u8, u8)> {
        match self.encoding {
            Some(encoding) => Operand::from(encoding).minimum_version(),
            None => Some((1, 0)),
        }
    }
    fn required_extensions(&self, _ctx: &Context) -> Vec<Vec<&'static str>> {
        let mut out = vec![];
        if let Some(encoding) = self.encoding.map(Operand::from) {
            out.extend(encoding.required_extensions());
        }
        out
    }
    fn required_capabilities(&self, _ctx: &Context) -> Vec<Vec<Capability>> {
        let mut out = vec![];
        match self.width {
            64 => out.push(vec![Capability::Float64]),
            16 => out.push(vec![Capability::Float16]),
            8 => out.push(vec![Capability::Float8EXT]),
            _ => {}
        }
        if let Some(encoding) = self.encoding.map(Operand::from) {
            out.extend(encoding.required_capabilities());
        }
        out
    }
}

#[pliron_type(
    name = "spirv.vector",
    format = "`<` $count ` x ` $element_type `>`",
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
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let element_ty = spirv_type_id(ctx, builder, self.element_type)?;
        Ok(builder.type_vector(element_ty, self.count))
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for VectorType {
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        let inner = self.element_type.deref(ctx);
        if self.count > 4 {
            None
        } else if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            ver_cap.min_version(ctx)
        } else {
            Some((1, 0))
        }
    }
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        let mut out = vec![];
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            out.extend(ver_cap.required_extensions(ctx));
        }
        if self.count > 4 {
            out.push(vec!["SPV_EXT_long_vector"]);
        }
        out
    }
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        let mut out = vec![];
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            out.extend(ver_cap.required_capabilities(ctx));
        }
        if self.count > 4 {
            out.push(vec![Capability::LongVectorEXT]);
        }
        out
    }
}

/// Long vector from [`SPV_EXT_long_vector`](https://github.khronos.org/SPIRV-Registry/extensions/EXT/SPV_EXT_long_vector.html)
#[pliron_type(
    name = "spirv.vector_id",
    format = "`<` $count ` x ` $element_type `>`",
    generate_get = true,
    verifier = "succ"
)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VectorIdType {
    pub count: u32,
    pub element_type: TypeHandle,
}

#[type_interface_impl]
impl ToSpirvType for VectorIdType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let element_ty = spirv_type_id(ctx, builder, self.element_type)?;
        let u32 = IntegerType::get(ctx, 32, Signedness::Signless).to_handle();
        let count = builder.constant_bit32(ctx, u32, self.count)?;
        Ok(builder.type_vector_id_ext(element_ty, count))
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for VectorIdType {
    fn min_version(&self, _ctx: &Context) -> Option<(u8, u8)> {
        None
    }
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        let mut out = vec![vec!["SPV_EXT_long_vector"]];
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            out.extend(ver_cap.required_extensions(ctx));
        }
        out
    }
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        let mut out = vec![vec![Capability::LongVectorEXT]];
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            out.extend(ver_cap.required_capabilities(ctx));
        }
        out
    }
}

#[pliron_type(name = "spirv.array", generate_get = true, verifier = "succ")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub count: u32,
    pub element_type: TypeHandle,
    pub stride: Option<u32>,
}

#[type_interface_impl]
impl ToSpirvType for ArrayType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let id = builder.id();
        if let Some(stride) = self.stride {
            builder.decorate(id, Decoration::ArrayStride, [stride.into()]);
        }

        let element_ty = spirv_type_id(ctx, builder, self.element_type)?;
        let u32 = IntegerType::get(ctx, 32, Signedness::Signless).to_handle();
        let count = builder.constant_bit32(ctx, u32, self.count)?;
        Ok(builder.type_array_id(Some(id), element_ty, count))
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for ArrayType {
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            ver_cap.min_version(ctx)
        } else {
            Some((1, 0))
        }
    }
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            ver_cap.required_extensions(ctx)
        } else {
            vec![]
        }
    }
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            ver_cap.required_capabilities(ctx)
        } else {
            vec![]
        }
    }
}

impl Printable for ArrayType {
    fn fmt(&self, ctx: &Context, _state: &printable::State, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "<{} x {}", self.count, self.element_type.disp(ctx))?;
        if let Some(stride) = self.stride {
            write!(f, ", stride = {stride}")?;
        }
        f.write_char('>')
    }
}

impl Parsable for ArrayType {
    type Arg = ();
    type Parsed = TypedHandle<Self>;

    fn parse<'a>(state_stream: &mut StateStream<'a>, _arg: Self::Arg) -> ParseResult<'a, Self::Parsed> {
        let stride_parser =
            (spaced(char(',')), spaced(string("stride")), spaced(char('='))).with(spaced(parsers::int_parser()));
        let mut parser = between(
            char('<'),
            char('>'),
            (
                parsers::int_parser().skip(spaced(char('x'))),
                parsers::type_parser(),
                optional(stride_parser),
            ),
        );
        let (count, element_type, stride) = parser.parse_stream(state_stream).into_result()?.0;
        let ctx: &mut &'a mut Context = &mut state_stream.state.ctx;
        Ok(Self::get(ctx, count, element_type, stride)).into_parse_result()
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
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let element_ty = spirv_type_id(ctx, builder, self.element_type)?;
        Ok(builder.type_pointer(None, self.storage_class, element_ty))
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for PointerType {
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        let mut out = (1, 0);
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            out = out.max(ver_cap.min_version(ctx)?);
        }
        out = out.max(Operand::from(self.storage_class).minimum_version()?);
        Some(out)
    }
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        let mut out = vec![];
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            out.extend(ver_cap.required_extensions(ctx));
        }
        out.extend(Operand::from(self.storage_class).required_extensions());
        out
    }
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        let mut out = vec![];
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            out.extend(ver_cap.required_capabilities(ctx));
        }
        out.extend(Operand::from(self.storage_class).required_capabilities());
        out
    }
}

#[pliron_type(name = "spirv.runtime_array", generate_get = true, verifier = "succ")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RuntimeArrayType {
    pub element_type: TypeHandle,
    pub stride: Option<u32>,
}

#[type_interface_impl]
impl ToSpirvType for RuntimeArrayType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let id = builder.id();
        if let Some(stride) = self.stride {
            builder.decorate(id, Decoration::ArrayStride, [stride.into()]);
        }

        let element_ty = spirv_type_id(ctx, builder, self.element_type)?;
        Ok(builder.type_runtime_array_id(Some(id), element_ty))
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for RuntimeArrayType {
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            ver_cap.min_version(ctx)
        } else {
            Some((1, 0))
        }
    }
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            ver_cap.required_extensions(ctx)
        } else {
            vec![]
        }
    }
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        let inner = self.element_type.deref(ctx);
        if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*inner) {
            ver_cap.required_capabilities(ctx)
        } else {
            vec![]
        }
    }
}

impl Printable for RuntimeArrayType {
    fn fmt(&self, ctx: &Context, _state: &printable::State, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "<{}", self.element_type.disp(ctx))?;
        if let Some(stride) = self.stride {
            write!(f, ", stride = {stride}")?;
        }
        f.write_char('>')
    }
}

impl Parsable for RuntimeArrayType {
    type Arg = ();
    type Parsed = TypedHandle<Self>;

    fn parse<'a>(state_stream: &mut StateStream<'a>, _arg: Self::Arg) -> ParseResult<'a, Self::Parsed> {
        let stride_parser =
            (spaced(char(',')), spaced(string("stride")), spaced(char('='))).with(spaced(parsers::int_parser()));
        let mut parser = between(char('<'), char('>'), (parsers::type_parser(), optional(stride_parser)));
        let (element_type, stride) = parser.parse_stream(state_stream).into_result()?.0;
        let ctx = &mut state_stream.state.ctx;
        Ok(Self::get(ctx, element_type, stride)).into_parse_result()
    }
}

#[format]
#[derive(Debug, PartialEq, Eq, Hash, new)]
pub struct MemberDecorationInfo {
    pub index: u32,
    pub decoration: DecorationInfo,
}

#[pliron_type(name = "spirv.struct", generate_get = true, verifier = "succ")]
#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct StructType {
    pub field_types: Vec<TypeHandle>,
    pub offsets: Vec<u32>,
    pub member_decorations: Vec<MemberDecorationInfo>,
    pub type_decorations: Vec<DecorationInfo>,
}

impl StructType {
    pub fn decorate_type(&mut self, decoration: DecorationInfo) {
        self.type_decorations.push(decoration);
    }

    pub fn decorate_member(&mut self, idx: usize, decoration: DecorationInfo) {
        self.member_decorations
            .push(MemberDecorationInfo::new(idx as u32, decoration));
    }
}

#[type_interface_impl]
impl ToSpirvType for StructType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let id = builder.id();

        let field_types = self
            .field_types
            .iter()
            .map(|ty| spirv_type_id(ctx, builder, *ty))
            .collect::<Result<Vec<_>>>()?;

        for (i, &offset) in self.offsets.iter().enumerate() {
            builder.member_decorate(id, i as u32, Decoration::Offset, [offset.into()]);
        }

        for MemberDecorationInfo { index, decoration } in &self.member_decorations {
            builder.member_decorate(id, *index, decoration.decoration, decoration.as_operands());
        }

        for decoration in &self.type_decorations {
            builder.decorate(id, decoration.decoration, decoration.as_operands());
        }

        Ok(builder.type_struct_id(Some(id), field_types))
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for StructType {
    fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
        let mut out = (1, 0);
        for field in self.field_types.iter() {
            let ty = field.deref(ctx);
            if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*ty) {
                out = out.max(ver_cap.min_version(ctx)?);
            }
        }
        for decoration in self.member_decorations.iter() {
            out = out.max(Operand::from(decoration.decoration.decoration).minimum_version()?);
        }
        for decoration in self.type_decorations.iter() {
            out = out.max(Operand::from(decoration.decoration).minimum_version()?);
        }
        Some(out)
    }
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        let mut out = vec![];
        for field in self.field_types.iter() {
            let ty = field.deref(ctx);
            if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*ty) {
                out.extend(ver_cap.required_extensions(ctx));
            }
        }
        for decoration in self.member_decorations.iter() {
            out.extend(Operand::from(decoration.decoration.decoration).required_extensions());
        }
        for decoration in self.type_decorations.iter() {
            out.extend(Operand::from(decoration.decoration).required_extensions());
        }
        out
    }
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        let mut out = vec![];
        for field in self.field_types.iter() {
            let ty = field.deref(ctx);
            if let Some(ver_cap) = type_cast::<dyn VerCapExtTypeInterface>(&*ty) {
                out.extend(ver_cap.required_capabilities(ctx));
            }
        }
        for decoration in self.member_decorations.iter() {
            out.extend(Operand::from(decoration.decoration.decoration).required_capabilities());
        }
        for decoration in self.type_decorations.iter() {
            out.extend(Operand::from(decoration.decoration).required_capabilities());
        }
        out
    }
}

impl Printable for StructType {
    fn fmt(&self, ctx: &Context, state: &printable::State, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_char('<')?;
        for (i, field) in self.field_types.iter().enumerate() {
            field.fmt(ctx, state, f)?;
            let mut segments = vec![];
            if let Some(offset) = self.offsets.get(i) {
                segments.push(offset.to_string())
            }
            let mut attrs = AttributeDict::default();
            for decoration in self.member_decorations.iter().filter(|it| it.index == i as u32) {
                let DecorationInfo { decoration, value } = &decoration.decoration;
                attrs.0.insert(decoration.decoration_key().clone(), value.clone());
            }
            if !attrs.0.is_empty() {
                let mut attrs_str = String::new();
                print_decorations(ctx, &attrs, &mut attrs_str)?;
                segments.push(attrs_str);
            }
            if !segments.is_empty() {
                write!(f, " [{}]", segments.join(", "))?;
            }
            if i < self.field_types.len() - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str(">")?;
        let mut attrs = AttributeDict::default();
        for DecorationInfo { decoration, value } in self.type_decorations.iter() {
            attrs.0.insert(decoration.decoration_key().clone(), value.clone());
        }
        if !attrs.0.is_empty() {
            f.write_str(" ")?;
            print_decorations(ctx, &attrs, f)?;
        }
        Ok(())
    }
}

impl Parsable for StructType {
    type Arg = ();
    type Parsed = TypedHandle<Self>;

    fn parse<'a>(state_stream: &mut StateStream<'a>, _arg: Self::Arg) -> ParseResult<'a, Self::Parsed> {
        let offset_and_decorations_parse = between(
            char('['),
            char(']'),
            (
                optional(parsers::int_parser::<u32>()),
                optional(optional(spaced(char(','))).with(decorations_parser())),
            ),
        );
        let field_parser = (type_parser(), spaced(offset_and_decorations_parse));
        let mut fields_parser = delimited_list_parser('<', '>', ',', field_parser);
        let fields = fields_parser.parse_stream(state_stream).into_result()?.0;
        let decorations = spaced(decorations_parser()).parse_stream(state_stream).into_result()?.0;

        let mut field_types = vec![];
        let mut offsets = vec![];
        let mut member_dec = vec![];
        let mut type_dec = vec![];

        for (i, (ty, (offset, attrs))) in fields.into_iter().enumerate() {
            field_types.push(ty);
            offsets.extend(offset);
            let attrs = attrs.unwrap_or_default();
            for (key, value) in attrs.0 {
                let decoration = decoration_for_key(&key).expect("Only decorations allowed here");
                let info = DecorationInfo::new(decoration, value);
                member_dec.push(MemberDecorationInfo::new(i as u32, info));
            }
        }

        for (key, value) in decorations.0 {
            let decoration = decoration_for_key(&key).expect("Only decorations allowed here");
            type_dec.push(DecorationInfo::new(decoration, value));
        }

        let ctx = &mut state_stream.state.ctx;
        Ok(Self::get(ctx, field_types, offsets, member_dec, type_dec)).into_parse_result()
    }
}
