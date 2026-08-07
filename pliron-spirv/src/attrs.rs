use core::fmt;

use alloc::string::String;
use derive_more::{From, Into};
use derive_new::new;
use itertools::Itertools;
use pliron::{
    attribute::attr_cast,
    builtin::{
        attr_interfaces::TypedAttrInterface,
        attributes::IntegerAttr,
        types::{IntegerType, Signedness},
    },
    combine::{Parser, parser::char::char},
    derive::pliron_attr,
    identifier::Identifier,
    irfmt::{
        parsers::{delimited_list_parser, quoted_string_parse, spaced},
        printers::quoted,
    },
    parsable::{IntoParseResult, Parsable, ParseResult, StateStream},
    printable::{self, Printable},
    r#type::TypedHandle,
    verify_err_noloc,
};
use tracel_rspirv::spirv::{Capability, Word};

use crate::{PlironBuilder, ToSpirvAttr, prelude::*, types::FloatType};

pub use crate::autogen_attrs::*;

#[pliron_attr(name = "spirv.literal", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug, Hash, new, From, Into)]
pub struct LiteralIntegerAttr(pub u32);

#[pliron_attr(name = "spirv.string", verifier = "succ")]
#[derive(PartialEq, Clone, Debug, Hash, new, From, Into)]
pub struct LiteralStringAttr(pub String);

impl From<StringAttr> for LiteralStringAttr {
    fn from(value: StringAttr) -> Self {
        LiteralStringAttr::new(value.into())
    }
}

impl Printable for LiteralStringAttr {
    fn fmt(&self, ctx: &Context, state: &printable::State, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        quoted(&self.0).fmt(ctx, state, f)
    }
}

impl Parsable for LiteralStringAttr {
    type Arg = ();
    type Parsed = Self;

    fn parse<'a>(state_stream: &mut StateStream<'a>, _arg: Self::Arg) -> ParseResult<'a, Self::Parsed> {
        Ok(LiteralStringAttr::new(quoted_string_parse(state_stream, ())?.0)).into_parse_result()
    }
}

#[pliron_attr(name = "spirv.composite", format = "`[` $values ` : ` $ty `]`", verifier = "succ")]
#[derive(PartialEq, Clone, Debug, Hash, new, From, Into)]
pub struct CompositeAttr {
    pub values: VecAttr,
    pub ty: TypeHandle,
}

#[attr_interface_impl]
impl TypedAttrInterface for CompositeAttr {
    fn get_type(&self, _ctx: &Context) -> TypeHandle {
        self.ty
    }
}

#[attr_interface_impl]
impl ToSpirvAttr for CompositeAttr {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let mut values = vec![];
        for value in self.values.0.iter() {
            let Some(to_spirv) = attr_cast::<dyn ToSpirvAttr>(&**value) else {
                return verify_err_noloc!("Constants must implement `ToSpirvAttr`");
            };
            values.push(to_spirv.to_spirv(ctx, builder)?);
        }
        let result_ty = spirv_type_id(ctx, builder, self.ty)?;
        builder.constant_composite(result_ty, values).into_pliron_result()
    }
}

#[pliron_attr(name = "spirv.float", format = "$bits ` : ` $ty", verifier = "succ")]
#[derive(PartialEq, Clone, Debug, Hash, new, From, Into)]
pub struct FloatAttr {
    pub bits: u64,
    pub ty: TypedHandle<FloatType>,
}

#[attr_interface_impl]
impl TypedAttrInterface for FloatAttr {
    fn get_type(&self, _ctx: &Context) -> TypeHandle {
        self.ty.into()
    }
}

#[attr_interface_impl]
impl ToSpirvAttr for FloatAttr {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let width = self.ty.deref(ctx).width;
        if width <= 32 {
            builder.constant_bit32(ctx, self.ty.into(), self.bits as u32)
        } else {
            builder.constant_bit64(ctx, self.ty.into(), self.bits)
        }
    }
}

#[attr_interface_impl]
impl ToSpirvAttr for IntegerAttr {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let width = self.get_type().deref(ctx).width();
        if width == 1 {
            let bool = spirv_type_id(ctx, builder, self.get_type().to_handle())?;
            if self.value().is_zero() {
                builder.constant_false(bool).into_pliron_result()
            } else {
                builder.constant_true(bool).into_pliron_result()
            }
        } else if width <= 32 {
            builder.constant_bit32(ctx, self.get_type().into(), self.value().to_u32())
        } else {
            builder.constant_bit64(ctx, self.get_type().into(), self.value().to_u64())
        }
    }
}

#[pliron_attr(name = "spirv.vce", verifier = "succ")]
#[derive(Clone, Debug, PartialEq, Hash, new)]
pub struct VerCapExtAttr {
    pub version: (u8, u8),
    pub capabilities: Vec<Capability>,
    pub extensions: Vec<Identifier>,
}

impl Default for VerCapExtAttr {
    fn default() -> Self {
        Self {
            version: (1, 0),
            capabilities: Default::default(),
            extensions: Default::default(),
        }
    }
}

impl Printable for VerCapExtAttr {
    fn fmt(&self, ctx: &Context, _state: &pliron::printable::State, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let capabilities = self.capabilities.iter().map(|it| it.disp(ctx)).join(", ");
        let extensions = self.extensions.iter().map(|it| it.disp(ctx)).join(", ");
        write!(
            f,
            "<v{}.{}, [{capabilities}], [{extensions}]>",
            self.version.0, self.version.1
        )
    }
}

impl Parsable for VerCapExtAttr {
    type Arg = ();
    type Parsed = Self;

    fn parse<'a>(
        input: &mut pliron::parsable::StateStream<'a>,
        _arg: Self::Arg,
    ) -> pliron::parsable::ParseResult<'a, Self::Parsed> {
        spaced(char('<')).parse_stream(input).into_result()?;
        char('v').parse_stream(input).into_result()?;
        let v0 = u8::parser(()).parse_stream(input).into_result()?;
        char('.').parse_stream(input).into_result()?;
        let v1 = u8::parser(()).parse_stream(input).into_result()?;
        let version = (v0.0, v1.0);

        spaced(char(',')).parse_stream(input).into_result()?;

        let cap = Capability::parser(());
        let mut caps = delimited_list_parser('[', ']', ',', cap);
        let (capabilities, _) = caps.parse_stream(input).into_result()?;

        spaced(char(',')).parse_stream(input).into_result()?;

        let ext = Identifier::parser(());
        let mut exts = delimited_list_parser('[', ']', ',', ext);
        let (extensions, c) = exts.parse_stream(input).into_result()?;

        Ok((VerCapExtAttr::new(version, capabilities, extensions), c))
    }
}

impl ScopeAttr {
    pub fn spirv_id(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<u32> {
        let out_ty = IntegerType::get(ctx, 32, Signedness::Signless).to_handle();
        builder.constant_bit32(ctx, out_ty, self.0 as u32)
    }
}

impl MemorySemanticsAttr {
    pub fn spirv_id(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<u32> {
        let out_ty = IntegerType::get(ctx, 32, Signedness::Signless).to_handle();
        builder.constant_bit32(ctx, out_ty, self.0.bits())
    }
}

impl CooperativeMatrixLayoutAttr {
    pub fn spirv_id(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<u32> {
        let out_ty = IntegerType::get(ctx, 32, Signedness::Signless).to_handle();
        builder.constant_bit32(ctx, out_ty, self.0 as u32)
    }
}

#[cfg(test)]
mod parse_tests {
    use alloc::string::ToString;
    use pliron::{
        context::Context,
        location::Source,
        parsable::{self, Parsable, state_stream_from_iterator},
        printable::Printable,
    };
    use tracel_rspirv::spirv::Capability;

    use crate::attrs::VerCapExtAttr;

    #[test]
    fn parse_ver_cap() {
        let ctx = &mut Context::new();
        let input = "< v1.3, [Shader, GroupNonUniform], [SPV_KHR_8bit_storage]>";

        let state_stream = state_stream_from_iterator(input.chars(), parsable::State::new(ctx, Source::InMemory));

        let (parsed, _) = VerCapExtAttr::parser(()).parse(state_stream).expect("Should parse");

        assert_eq!(parsed.version, (1, 3));
        assert_eq!(parsed.capabilities, [Capability::Shader, Capability::GroupNonUniform]);
        assert_eq!(parsed.extensions, ["SPV_KHR_8bit_storage".try_into().unwrap()]);

        let display = parsed.disp(ctx).to_string();

        assert_eq!(display, "<v1.3, [Shader, GroupNonUniform], [SPV_KHR_8bit_storage]>");
    }
}
