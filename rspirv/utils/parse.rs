use core::{
    error::Error,
    fmt::{Debug, Display},
};

use pliron::{
    combine::{parser::char::spaces, sep_by, token, Parser},
    identifier::Identifier,
    input_err,
    input_error,
    location::Located,
    parsable::{IntoParseResult, Parsable, ParseResult, StateStream},
};

#[derive(Debug)]
struct ParseError(bitflags::parser::ParseError);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for ParseError {}

macro_rules! parsable_flags {
    ($ty: ty) => {
        const _: () = {
            fn parse_flag(name: &str) -> Result<u32, bitflags::parser::ParseError> {
                unsafe { core::mem::transmute(bitflags::parser::from_str::<$ty>(name)) }
            }

            impl ::pliron::printable::Printable for $ty {
                fn fmt(
                    &self,
                    _ctx: &::pliron::context::Context,
                    _state: &::pliron::printable::State,
                    fmt: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    bitflags::parser::to_writer(self, fmt)
                }
            }

            impl ::pliron::parsable::Parsable for $ty {
                type Arg = ();
                type Parsed = $ty;

                fn parse<'a>(
                    state_stream: &mut ::pliron::parsable::StateStream<'a>,
                    _arg: Self::Arg,
                ) -> ::pliron::parsable::ParseResult<'a, Self::Parsed> {
                    let flags = $crate::utils::parse::flags_parse(parse_flag, state_stream)?;
                    Ok((Self::from_bits_retain(flags.0), flags.1))
                }

                fn parser<'a>(
                    _arg: Self::Arg,
                ) -> Box<
                    dyn ::pliron::combine::Parser<
                            ::pliron::parsable::StateStream<'a>,
                            Output = Self::Parsed,
                            PartialState = (),
                        > + 'a,
                > {
                    todo!()
                }
            }
        };
    };
}
pub(crate) use parsable_flags;

macro_rules! parsable_enum {
    ($ty: ty; $($variant: ident = $disc: literal,)*) => {
        const _: () = {
            fn parse_variant(name: &str) -> Option<u32> {
                match name {
                    $(stringify!($variant) => Some($disc),)*
                    _ => None
                }
            }

            impl ::pliron::printable::Printable for $ty {
                fn fmt(
                    &self,
                    _ctx: &::pliron::context::Context,
                    _state: &::pliron::printable::State,
                    fmt: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(fmt, "{self:?}")
                }
            }

            impl ::pliron::parsable::Parsable for $ty {
                type Arg = ();
                type Parsed = $ty;

                fn parse<'a>(state_stream: &mut ::pliron::parsable::StateStream<'a>, _arg: Self::Arg) -> ::pliron::parsable::ParseResult<'a, Self::Parsed> {
                    let variant = $crate::utils::parse::enum_parse(parse_variant, state_stream)?;
                    Ok((unsafe { core::mem::transmute::<u32, Self>(variant.0) }, variant.1))
                }

                fn parser<'a>(_arg: Self::Arg) -> Box<dyn ::pliron::combine::Parser<::pliron::parsable::StateStream<'a>, Output = Self::Parsed, PartialState = ()> + 'a> {
                    // use ::pliron::combine::Parser;
                    // $crate::utils::parse::enum_parser(parse_variant).map(|variant| unsafe {
                    //     core::mem::transmute::<u32, Self>(variant)
                    // }).boxed()
                    todo!()
                }
            }
        };
    };
}
pub(crate) use parsable_enum;

#[inline(never)]
pub(crate) fn flags_parse<'a>(
    parse: fn(&str) -> Result<u32, bitflags::parser::ParseError>,
    state_stream: &mut StateStream<'a>,
) -> ParseResult<'a, u32> {
    let cur_loc = state_stream.loc();
    sep_by::<Vec<_>, _, _, _>(Identifier::parser(()).skip(spaces()), token('|').skip(spaces()))
        .flat_map(move |flags: Vec<Identifier>| -> core::result::Result<u32, _> {
            let flags = flags
                .into_iter()
                .map(|it| parse(it.as_str()))
                .collect::<core::result::Result<Vec<_>, _>>()
                .map_err(|err| input_error!(cur_loc.clone(), ParseError(err)))?;
            Ok(flags.into_iter().reduce(|a, b| a | b).unwrap_or(0))
        })
        .parse_stream(state_stream)
        .into_result()
}

#[inline(never)]
pub(crate) fn enum_parse<'a>(
    parse: fn(&str) -> Option<u32>,
    state_stream: &mut StateStream<'a>,
) -> ParseResult<'a, u32> {
    let cur_loc = state_stream.loc();
    let variant_name_parsed = ::pliron::identifier::Identifier::parser(())
        .parse_stream(state_stream)
        .into_result()?
        .0
        .to_string();
    let final_ret_value = match parse(&variant_name_parsed) {
        Some(value) => Ok(value),
        None => return input_err!(cur_loc.clone(), "Invalid variant name: {}", variant_name_parsed)?,
    };
    final_ret_value.into_parse_result()
}
