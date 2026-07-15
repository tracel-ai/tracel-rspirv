use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::fmt;
use derive_more::From;
use tracel_rspirv::spirv::MemoryAccess;

use itertools::Itertools;
use pliron::{
    attribute::AttrObj,
    builtin::{
        attributes::{IdentifierAttr, VecAttr},
        op_interfaces::SymbolOpInterface,
    },
    combine::{
        Parser,
        optional,
        parser::{
            char::{char, string},
            combinator::no_partial,
        },
    },
    context::{Context, Ptr},
    identifier::Identifier,
    irfmt::parsers::{self, delimited_list_parser, process_parsed_ssa_defs, spaced, type_parser},
    location::Location,
    op::{Op, OpObj, op_cast, op_impls_static},
    operation::Operation,
    parsable::{IntoParseResult, Parsable, ParseResult, StateStream, parser_combinator},
    printable::Printable,
    std_deps::sync::LazyLock,
    value::Value,
};

use crate::{
    attrs::MemoryAccessAttr,
    decorations::{decorations_parser, print_decorations},
    opt_memory_access,
};

macro_rules! canonical_format {
    ($ty: ty; $($opds: expr),*) => {
        const _: () = {
            const OPERANDS: &[crate::format::FormatVar] = &[ $($opds),* ];

            impl ::pliron::printable::Printable for $ty {
                fn fmt(&self, ctx: &Context, _: &::pliron::printable::State, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    crate::format::canonical_syntax_print(self, OPERANDS, ctx, fmt)?;
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
                    crate::format::canonical_syntax_parse::<Self>(OPERANDS, state_stream, arg)
                }
            }
        };
    };
}
pub(crate) use canonical_format;

macro_rules! attr {
    ($key: expr, $ty: ty, $name: expr, $quant: expr $(,)*) => {
        crate::format::FormatVar::Attribute($key, crate::format::inner_attr_parse::<$ty>, $name, $quant)
    };
}
pub(crate) use attr;

pub enum Quantifier {
    One,
    ZeroOrOne,
    ZeroOrMore,
}

pub enum FormatVar {
    Value(&'static str, Quantifier),
    Attribute(&'static LazyLock<Identifier>, AttrParseFn, &'static str, Quantifier),
    MemoryAccess(&'static LazyLock<Identifier>, &'static str),
    Symbol(&'static LazyLock<Identifier>, &'static str, Quantifier),
}

#[inline(never)]
pub fn canonical_syntax_print(
    dyn_op: &dyn Op,
    opds: &[FormatVar],
    ctx: &Context,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let opid = dyn_op.get_opid();
    let op = dyn_op.get_operation().deref(ctx);

    if op.get_num_results() > 0 {
        let mut results = op.results().map(|it| it.disp(ctx).to_string());
        write!(f, "{} = ", results.join(", "))?;
    }

    let mut opd_id = 0;
    write!(f, "{opid}")?;

    if let Some(has_symbol) = op_cast::<dyn SymbolOpInterface>(dyn_op) {
        let symbol = has_symbol.get_symbol_name(ctx);
        write!(f, " @{symbol}")?;
    }

    let mut opd_disp = Vec::new();

    for opd in opds {
        match opd {
            FormatVar::Value(_, Quantifier::One) => {
                opd_disp.push(op.get_operand(opd_id).disp(ctx).to_string());
                opd_id += 1;
            }
            FormatVar::Value(name, Quantifier::ZeroOrOne) => {
                if let Some(opd) = op.operands().nth(opd_id) {
                    opd_disp.push(format!("{name}: {}", opd.disp(ctx)));
                }
            }
            FormatVar::Value(name, Quantifier::ZeroOrMore) => {
                let opds = op
                    .operands()
                    .skip(opd_id)
                    .map(|opd| opd.disp(ctx).to_string())
                    .join(", ");
                if !opds.is_empty() {
                    opd_disp.push(format!("{name}: [{opds}]"));
                }
            }
            FormatVar::Attribute(identifier, _, _, Quantifier::One) => {
                let attr = op.attributes.0.get(&***identifier).unwrap();
                opd_disp.push(format!("{}", (**attr).disp(ctx)));
            }
            FormatVar::Attribute(identifier, _, name, Quantifier::ZeroOrOne) => {
                if let Some(attr) = op.attributes.0.get(&***identifier) {
                    opd_disp.push(format!("{name} = {}", (**attr).disp(ctx)));
                }
            }
            FormatVar::Attribute(identifier, _, name, Quantifier::ZeroOrMore) => {
                let attr = op.attributes.get::<VecAttr>(identifier).unwrap();
                let disp = attr.0.iter().map(|attr| (**attr).disp(ctx).to_string()).join(", ");
                if !disp.is_empty() {
                    opd_disp.push(format!("{name} = [{disp}]"));
                }
            }
            FormatVar::MemoryAccess(identifier, name) => {
                if let Some(attr) = opt_memory_access(op.attributes.get::<MemoryAccessAttr>(identifier).unwrap().0) {
                    opd_disp.push(format!("{name} = {}", MemoryAccessAttr::new(attr).disp(ctx)));
                }
            }
            FormatVar::Symbol(identifier, _, Quantifier::One) => {
                let attr = op.attributes.get::<IdentifierAttr>(identifier).unwrap();
                let ident: Identifier = attr.clone().into();
                opd_disp.push(format!("@{}", ident.disp(ctx)));
            }
            FormatVar::Symbol(identifier, name, Quantifier::ZeroOrOne) => {
                if let Some(attr) = op.attributes.get::<IdentifierAttr>(identifier) {
                    let ident: Identifier = attr.clone().into();
                    opd_disp.push(format!("{name} = @{}", ident.disp(ctx)));
                }
            }
            FormatVar::Symbol(identifier, name, Quantifier::ZeroOrMore) => {
                let attr = op.attributes.get::<VecAttr>(identifier).unwrap();
                if !attr.0.is_empty() {
                    let mut disp = attr.0.iter().map(|attr| {
                        let ident: Identifier = attr.downcast_ref::<IdentifierAttr>().unwrap().clone().into();
                        format!("@{}", ident.disp(ctx))
                    });
                    opd_disp.push(format!("{name} = [{}]", disp.join(", ")));
                }
            }
        }
    }

    if !opd_disp.is_empty() {
        write!(f, " {}", opd_disp.join(", "))?;
    }

    if op.get_num_results() > 0 {
        let mut result_types = op.result_types().map(|it| it.disp(ctx).to_string());
        write!(f, " : <{}>", result_types.join(", "))?;
    }

    let mut decorations = String::new();
    print_decorations(ctx, &op.attributes, &mut decorations)?;
    if !decorations.is_empty() {
        write!(f, " {decorations}")?;
    }

    Ok(())
}

type AttrParseFn = for<'a> fn(&mut StateStream<'a>, &'a ()) -> ParseResult<'a, AttrObj>;
type ConcreteOpInfo = (fn(Ptr<Operation>) -> OpObj, core::any::TypeId);

#[derive(From)]
enum ParsedOpd {
    Value(Value),
    Values(Vec<Value>),
    Attr(Identifier, AttrObj),
    Empty,
}

impl From<Option<Value>> for ParsedOpd {
    fn from(value: Option<Value>) -> Self {
        match value {
            Some(value) => ParsedOpd::Value(value),
            None => ParsedOpd::Empty,
        }
    }
}

impl From<Option<ParsedOpd>> for ParsedOpd {
    fn from(value: Option<ParsedOpd>) -> Self {
        match value {
            Some(inner) => inner,
            None => ParsedOpd::Empty,
        }
    }
}

pub fn canonical_syntax_parse<'a, T: Op>(
    opds: &[FormatVar],
    state_stream: &mut StateStream<'a>,
    results: Vec<(Identifier, Location)>,
) -> ParseResult<'a, OpObj> {
    canonical_syntax_parse_impl(
        opds,
        op_impls_static::<T, dyn SymbolOpInterface>,
        state_stream,
        results,
        T::get_concrete_op_info,
        T::wrap_operation,
    )
}

#[inline(never)]
pub fn canonical_syntax_parse_impl<'a>(
    opds: &[FormatVar],
    has_symbol: fn() -> bool,
    input: &mut StateStream<'a>,
    results: Vec<(Identifier, Location)>,
    concrete_op: fn() -> ConcreteOpInfo,
    wrap_op: fn(Ptr<Operation>) -> OpObj,
) -> ParseResult<'a, OpObj> {
    let symbol = if has_symbol() {
        Some(symbol_parse(input, &())?.0)
    } else {
        None
    };

    let parsers = opds.iter().map(|opd| match opd {
        FormatVar::Value(_, Quantifier::One) => ssa_opd_parser(),
        FormatVar::Value(name, Quantifier::ZeroOrOne) => opt_parser(labeled(name, ':', ssa_opd_parser())),
        FormatVar::Value(name, Quantifier::ZeroOrMore) => opt_parser(labeled(name, ':', ssa_opds_parser())),
        FormatVar::Attribute(key, parse, _, Quantifier::One) => attr_parser(*parse, key),
        FormatVar::Attribute(key, parse, name, Quantifier::ZeroOrOne) => {
            opt_parser(labeled(name, '=', attr_parser(*parse, key)))
        }
        FormatVar::Attribute(key, parse, name, Quantifier::ZeroOrMore) => {
            let parse = optional(labeled(name, '=', vec_attr_parser(*parse, key)));
            no_partial(parse.map(|attr| {
                attr.unwrap_or_else(|| ParsedOpd::Attr((**key).clone(), VecAttr(Default::default()).into()))
            }))
            .boxed()
        }
        FormatVar::MemoryAccess(key, name) => {
            let attr = (spaced(string(name)), spaced(char('='))).with(MemoryAccessAttr::parser(()));
            no_partial(optional(attr).map(|attr| {
                let access = attr.map(|it| it.0).unwrap_or(MemoryAccess::NONE);
                ParsedOpd::Attr((**key).clone(), MemoryAccessAttr::new(access).into())
            }))
            .boxed()
        }
        FormatVar::Symbol(key, _, Quantifier::One) => symbol_opd_parser(key),
        FormatVar::Symbol(key, name, Quantifier::ZeroOrOne) => opt_parser(labeled(name, '=', symbol_opd_parser(key))),
        FormatVar::Symbol(key, name, Quantifier::ZeroOrMore) => {
            let attr_parser = parser_combinator(symbol_parse, &());
            let parser = delimited_list_parser('[', ']', ',', attr_parser).map(|attrs| {
                let attrs = attrs.into_iter().map(|sym| IdentifierAttr::new(sym).into()).collect();
                ParsedOpd::Attr((**key).clone(), VecAttr(attrs).into())
            });
            no_partial(optional(labeled(name, '=', parser)).map(|attr| {
                attr.unwrap_or_else(|| ParsedOpd::Attr((**key).clone(), VecAttr(Default::default()).into()))
            }))
            .boxed()
        }
    });

    let mut opds_parsed = vec![];

    for mut parser in parsers {
        let value = parser.parse_stream(input).into_result()?.0;
        opds_parsed.push(value);
        optional(spaced(char(','))).parse_stream(input).into_result()?;
    }

    let mut ty_parse = optional(spaced(char(':')).with(delimited_list_parser('<', '>', ',', type_parser())));
    let ty = ty_parse.parse_stream(input).into_result()?.0.unwrap_or_default();

    let decorations = spaced(decorations_parser()).parse_stream(input).into_result()?.0;

    let opds = opds_parsed.iter().flat_map(|opd| match opd {
        ParsedOpd::Value(value) => vec![*value],
        ParsedOpd::Values(values) => values.clone(),
        ParsedOpd::Attr(..) | ParsedOpd::Empty => vec![],
    });

    let ctx = &mut input.state.ctx;
    let op = Operation::new(ctx, concrete_op(), ty, opds.collect(), vec![], 0);

    op.deref_mut(ctx).attributes = decorations;
    for opd in opds_parsed {
        match opd {
            ParsedOpd::Value(..) | ParsedOpd::Values(..) | ParsedOpd::Empty => {}
            ParsedOpd::Attr(identifier, attribute) => {
                op.deref_mut(ctx).attributes.0.insert(identifier, attribute);
            }
        }
    }

    if let Some(sym) = symbol {
        let op_dyn = Operation::get_op_dyn(op, ctx);
        op_cast::<dyn SymbolOpInterface>(&*op_dyn)
            .expect("Should implement `SymbolOpInterface`")
            .set_symbol_name(ctx, sym);
    }

    process_parsed_ssa_defs(input, &results, op)?;

    Ok(wrap_op(op)).into_parse_result()
}

#[inline(never)]
fn symbol_parse<'a>(input: &mut StateStream<'a>, _: &()) -> ParseResult<'a, Identifier> {
    char('@').with(Identifier::parser(())).parse_stream(input).into_result()
}

#[inline(never)]
fn symbol_opd_parser<'a>(
    key: &'a Identifier,
) -> Box<dyn Parser<StateStream<'a>, Output = ParsedOpd, PartialState = ()> + 'a> {
    let parse =
        parser_combinator(symbol_parse, &()).map(|sym| ParsedOpd::Attr(key.clone(), IdentifierAttr::new(sym).into()));
    no_partial(parse).boxed()
}

#[inline(never)]
fn ssa_opd_parser<'a>() -> Box<dyn Parser<StateStream<'a>, Output = ParsedOpd, PartialState = ()> + 'a> {
    parsers::ssa_opd_parser().map(ParsedOpd::Value).boxed()
}

#[inline(never)]
fn attr_parser<'a>(
    inner: AttrParseFn,
    key: &'a Identifier,
) -> Box<dyn Parser<StateStream<'a>, Output = ParsedOpd, PartialState = ()> + 'a> {
    parser_combinator(inner, &())
        .map(|attr| ParsedOpd::Attr(key.clone(), attr))
        .boxed()
}

#[inline(never)]
fn vec_attr_parser<'a>(
    inner: AttrParseFn,
    key: &'a Identifier,
) -> Box<dyn Parser<StateStream<'a>, Output = ParsedOpd, PartialState = ()> + 'a> {
    let attr_parser = parser_combinator(inner, &());
    let parser = delimited_list_parser('[', ']', ',', attr_parser)
        .map(|attr| ParsedOpd::Attr(key.clone(), VecAttr::new(attr).into()));
    no_partial(parser).boxed()
}

fn ssa_opds_parser<'a>() -> impl Parser<StateStream<'a>, Output = Vec<Value>, PartialState = ()> {
    no_partial(delimited_list_parser('[', ']', ',', parsers::ssa_opd_parser()))
}

#[inline(never)]
fn labeled<'a, O: Into<ParsedOpd> + 'a>(
    label: &'static str,
    sep: char,
    inner: impl Parser<StateStream<'a>, Output = O> + 'a,
) -> Box<dyn Parser<StateStream<'a>, Output = ParsedOpd, PartialState = ()> + 'a> {
    let parse = (spaced(string(label)), spaced(char(sep))).with(inner);
    no_partial(parse.map(Into::into)).boxed()
}

fn opt_parser<'a>(
    inner: impl Parser<StateStream<'a>, Output = ParsedOpd, PartialState = ()> + 'a,
) -> Box<dyn Parser<StateStream<'a>, Output = ParsedOpd, PartialState = ()> + 'a> {
    no_partial(optional(inner).map(Into::into)).boxed()
}

pub fn inner_attr_parse<'a, A: Sized + Parsable<Arg = (), Parsed = A> + Into<AttrObj>>(
    parsable_state: &mut StateStream<'a>,
    _: &'a (),
) -> ParseResult<'a, AttrObj> {
    A::parse(parsable_state, ()).map(|(attr, r)| -> (AttrObj, _) { (attr.into(), r) })
}
