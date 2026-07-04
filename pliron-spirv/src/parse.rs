macro_rules! canonical_format {
    ($ty: ty) => {
        impl ::pliron::printable::Printable for $ty {
            fn fmt(
                &self,
                ctx: &::pliron::context::Context,
                state: &::pliron::printable::State,
                fmt: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                ::pliron::op::canonical_syntax_print(::pliron::op::OpObj::new(*self), ctx, state, fmt)?;
                Ok(())
            }
        }

        impl ::pliron::parsable::Parsable for $ty {
            type Arg = ::pliron::alloc::vec::Vec<(::pliron::identifier::Identifier, ::pliron::location::Location)>;
            type Parsed = ::pliron::op::OpObj;
            fn parse<'__pliron_parse>(
                state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
                arg: Self::Arg,
            ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
                ::pliron::op::canonical_syntax_parse::<Self>(state_stream, arg)
            }

            fn parser<'a>(
                _arg: Self::Arg,
            ) -> alloc::boxed::Box<
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
}
pub(crate) use canonical_format;
