use std::collections::HashMap;

use heck::ToSnekCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    pliron::PlironGenerator,
    split_vendor_tag,
    structs::{Class, Instruction, Quantifier},
    utils::as_ident,
};

const SKIP_OP_CLASSES: &[Class] = &[
    Class::Type,
    Class::Branch, // Missing Abort
    Class::Constant,
    Class::Debug,
    Class::ExtensionDecl, // All handled
    Class::ModeSetting,
    Class::Annotation,
    Class::Graph,
    Class::Image,
    Class::Exclude,
];

const SKIP_OP_NAMES: &[&str] = &[
    // Function
    "OpFunction",
    "OpFunctionParameter",
    "OpFunctionEnd",
    "OpMemoryModel",
    "OpExecutionMode",
    "OpExecutionModeId",
    "OpCapability",
    "OpConditionalCapabilityINTEL",
    "OpTensorReadARM",
    "OpTensorWriteARM",
    "OpUntypedVariableKHR",
    "OpUntypedPrefetchKHR",
    "OpConstantStringAMDX",
    "OpSpecConstantStringAMDX",
    "OpReorderThreadWithHitObjectNV",
    "OpHitObjectReorderExecuteShaderEXT",
    "OpHitObjectTraceReorderExecuteEXT",
    "OpHitObjectTraceMotionReorderExecuteEXT",
    "OpReorderThreadWithHitObjectEXT",
    "OpCooperativeMatrixStoreTensorNV",
    "OpCooperativeMatrixLoadTensorNV",
    "OpSamplerImageAddressingModeNV",
];

const UNSKIP_OP_NAMES: &[&str] = &["OpEntryPoint"];

const SYMBOL_ARG_NAMES: &[&str] = &["Function", "Func", "Entry Point", "Interface"];

fn should_skip(op: &Instruction) -> bool {
    let skip_class = op.class.as_ref().is_some_and(|class| SKIP_OP_CLASSES.contains(class));
    let unskip_op = UNSKIP_OP_NAMES.contains(&op.opname.as_str());
    let skip_name = SKIP_OP_NAMES.contains(&op.opname.as_str());
    (skip_class && !unskip_op) || skip_name
}

pub enum OpdKind {
    ResultType,
    ResultValue,
    Value(Ident, Quantifier),
    Attribute(Ident, Ident, Quantifier),
    // Special-case
    MemoryAccess(Ident, Ident),
    /// Bitflags attribute that's passed by constant ID
    ValueAttr(Ident, Ident),
    /// Symbol reference
    SymbolRef(Ident, Quantifier),
}

impl PlironGenerator {
    pub fn generate_ops(&self) -> TokenStream {
        let mut root_ops = vec![];
        let mut vendor_ops: HashMap<&str, Vec<_>> = HashMap::new();
        for op in self.grammar.instructions.iter() {
            let (_, vendor) = split_vendor_tag(&op.opname);
            let op = self.generate_op(op);
            match vendor {
                Some(vendor) => vendor_ops.entry(vendor).or_default().push(op),
                None => root_ops.push(op),
            }
        }

        let vendor_modules = vendor_ops.into_iter().map(|(vendor, ops)| {
            let module = format_ident!("{}", vendor.to_lowercase());
            quote! {
                pub mod #module {
                    use super::*;
                    #(#ops)*
                }
            }
        });

        quote! {
            #![allow(clippy::let_and_return)]
            use crate::prelude::*;
            use crate::attrs::*;

            #(#root_ops)*
            #(#vendor_modules)*
        }
    }

    fn generate_op(&self, op: &Instruction) -> TokenStream {
        if should_skip(op) {
            return TokenStream::new();
        }

        let op_name = op.opname.strip_prefix("Op").unwrap();
        let builder_name = as_ident(&op_name.to_snek_case());
        let (op_name, vendor) = split_vendor_tag(op_name);
        let op_prefix = vendor.map(|it| format!("{it}.")).unwrap_or_default();
        let ty_name = format_ident!("{op_name}Op");
        let namespace = format!("spirv_{builder_name}");
        let op_name = format!("spirv.{op_prefix}{op_name}");

        let has_result = match op.operands.first() {
            Some(opd) if opd.kind == "IdResultType" => true,
            Some(opd) if opd.kind == "IdResult" => panic!(),
            _ => false,
        };

        if matches!(op.operands.first(), Some(opd) if opd.kind == "IdResult") {
            panic!("Result with no type")
        }

        let operands = self.operands(op);
        self.validate_operands(&operands);

        self.generate_op_impl(&op_name, &ty_name, &builder_name, &namespace, &operands, has_result)
    }

    pub fn generate_op_impl(
        &self,
        op_name: &str,
        ty_name: &Ident,
        builder_name: &Ident,
        namespace: &str,
        operands: &[OpdKind],
        has_result: bool,
    ) -> TokenStream {
        let values = self.values(operands);
        let to_spirv = self.to_spirv(ty_name, builder_name, operands, has_result);
        let attr_idents = self.attr_idents(namespace, operands);

        let interfaces = match has_result {
            true => quote!([NResultsInterface<1>, OneResultInterface, DecoratableOp]),
            false => quote!([DecoratableOp]),
        };

        let pliron_op = quote! {
            #[pliron_op(
                name = #op_name,
                format,
                operands = (#(#values),*),
                interfaces = #interfaces,
                verifier = "succ"
            )]
            pub struct #ty_name;
        };

        let constructor = self.constructor(ty_name, namespace, operands, has_result);
        let _format = self.op_format(ty_name);

        quote! {
            #pliron_op

            #attr_idents
            #constructor
            #to_spirv
        }
    }

    pub fn operands(&self, op: &Instruction) -> Vec<OpdKind> {
        op.operands
            .iter()
            .enumerate()
            .map(|(i, opd)| match opd.kind.as_str() {
                "IdResultType" => {
                    assert_eq!(opd.quantifier, Quantifier::One);
                    OpdKind::ResultType
                }
                "IdResult" => {
                    assert_eq!(opd.quantifier, Quantifier::One);
                    OpdKind::ResultValue
                }
                "IdRef" if SYMBOL_ARG_NAMES.contains(&opd.name.as_str()) => {
                    let name = as_ident(&opd.name.to_snek_case());
                    OpdKind::SymbolRef(name, opd.quantifier)
                }
                "IdRef" => {
                    let name = if opd.name.is_empty() {
                        as_ident(&format!("opd_{i}"))
                    } else {
                        as_ident(&opd.name.to_snek_case())
                    };
                    OpdKind::Value(name, opd.quantifier)
                }
                "MemoryAccess" => {
                    assert_ne!(opd.quantifier, Quantifier::ZeroOrMore);
                    let name = if opd.name.is_empty() {
                        as_ident(&format!("{}_{i}", opd.kind.to_snek_case()))
                    } else {
                        as_ident(&opd.name.to_snek_case())
                    };
                    let align = format_ident!("{}_align", name);
                    OpdKind::MemoryAccess(name, align)
                }
                "IdScope" => {
                    assert_eq!(opd.quantifier, Quantifier::One);
                    let name = if opd.name.is_empty() {
                        as_ident(&format!("{}_{i}", opd.kind.to_snek_case()))
                    } else {
                        as_ident(&opd.name.to_snek_case())
                    };
                    OpdKind::ValueAttr(name, format_ident!("ScopeAttr"))
                }
                "IdMemorySemantics" => {
                    assert_eq!(opd.quantifier, Quantifier::One);
                    let name = if opd.name.is_empty() {
                        as_ident(&format!("{}_{i}", opd.kind.to_snek_case()))
                    } else {
                        as_ident(&opd.name.to_snek_case())
                    };
                    OpdKind::ValueAttr(name, format_ident!("MemorySemanticsAttr"))
                }
                other if let Some(attr) = self.operand_kinds.get(other) => {
                    let name = if opd.name.is_empty() {
                        as_ident(&format!("{}_{i}", opd.kind.to_snek_case()))
                    } else {
                        as_ident(&opd.name.to_snek_case())
                    };
                    OpdKind::Attribute(name, attr.clone(), opd.quantifier)
                }
                other => panic!("Unsupported operand kind {} in op {}", other, op.opname),
            })
            .collect()
    }

    pub fn validate_operands(&self, opds: &[OpdKind]) {
        let value_quants = opds.iter().filter_map(|opd| match opd {
            OpdKind::Value(_, quant) => Some(quant),
            _ => None,
        });
        if value_quants.rev().skip(1).any(|quant| *quant != Quantifier::One) {
            panic!("Optional quantifier isn't last")
        }
    }

    pub fn values(&self, operands: &[OpdKind]) -> Vec<Ident> {
        let values = operands.iter().filter_map(|opd| match opd {
            OpdKind::Value(ident, _) => Some(ident.clone()),
            _ => None,
        });
        values.collect()
    }

    pub fn attr_setters(&self, opds: &[OpdKind]) -> Vec<TokenStream> {
        let res = opds.iter().filter_map(|opd| match opd {
            OpdKind::Attribute(name, _, Quantifier::One) => {
                let setter = format_ident!("set_attr_{}", name);
                Some(quote![op.#setter(ctx, #name.into());])
            }
            OpdKind::Attribute(name, _, Quantifier::ZeroOrMore) => {
                let setter = format_ident!("set_attr_{}", name);
                Some(quote![op.#setter(ctx, as_vec_attr(#name));])
            }
            OpdKind::ValueAttr(name, _) => {
                let setter = format_ident!("set_attr_{}", name);
                Some(quote![op.#setter(ctx, #name.into());])
            }
            OpdKind::MemoryAccess(name, name_align) => {
                let setter = format_ident!("set_attr_{}", name);
                let setter_align = format_ident!("set_attr_{}", name_align);
                Some(quote! {
                    op.#setter(ctx, #name.into());
                    if let Some(align) = #name_align {
                        op.#setter_align(ctx, align.into());
                    }
                })
            }
            OpdKind::SymbolRef(name, Quantifier::One) => {
                let setter = format_ident!("set_attr_{}", name);
                Some(quote! {
                    op.#setter(ctx, IdentifierAttr::new(#name));
                })
            }
            OpdKind::SymbolRef(name, Quantifier::ZeroOrMore) => {
                let setter = format_ident!("set_attr_{}", name);
                Some(quote![op.#setter(ctx,
                    as_vec_attr(#name.into_iter().map(|ident| IdentifierAttr::new(ident))));])
            }
            _ => None,
        });
        res.collect()
    }

    pub fn attr_getters(&self, operands: &[OpdKind]) -> Vec<TokenStream> {
        let values = self.values(operands);
        let getters = operands.iter().map(|opd| -> TokenStream {
            match opd {
                OpdKind::ResultType => {
                    quote![let result_ty = spirv_type_id(ctx, builder, self.get_result(ctx).get_type(ctx))?;]
                }
                OpdKind::ResultValue => quote![let result = builder.value_id(self.get_result(ctx));],
                OpdKind::Value(name, quant) => {
                    let getter = format_ident!("get_operand_{}", name);
                    let num_required_opds = values.len() - 1;
                    match quant {
                        Quantifier::One => {
                            quote![let #name = builder.value_id(self.#getter(ctx));]
                        }
                        Quantifier::ZeroOrOne => {
                            quote![let #name = op.operands().skip(#num_required_opds).next()
                        .map(|opd| builder.value_id(opd));]
                        }
                        Quantifier::ZeroOrMore => {
                            quote![let #name = op.operands().skip(#num_required_opds)
                        .map(|opd| builder.value_id(opd)).collect::<Vec<_>>();]
                        }
                    }
                }
                OpdKind::Attribute(name, ty, quant) => {
                    let getter = format_ident!("get_attr_{}", name);
                    match quant {
                        Quantifier::One => {
                            quote![let #name = self.#getter(ctx).clone().0;]
                        }
                        Quantifier::ZeroOrOne => quote![let #name = self.#getter(ctx).map(|it| it.clone().0);],
                        Quantifier::ZeroOrMore => quote![
                            let #name = from_vec_attr::<#ty>(self.#getter(ctx)).into_iter().map(|it| it.clone().0);
                        ],
                    }
                }
                OpdKind::ValueAttr(name, _) => {
                    let getter = format_ident!("get_attr_{}", name);
                    quote![let #name = self.#getter(ctx).spirv_id(ctx, builder)?;]
                }
                OpdKind::MemoryAccess(name, name_align) => {
                    let getter = format_ident!("get_attr_{}", name);
                    let getter_align = format_ident!("get_attr_{}", name_align);
                    quote! {
                        let #name = Some(self.#getter(ctx).0);
                        let #name_align = self.#getter_align(ctx).map(|it| it.0.into());
                    }
                }
                OpdKind::SymbolRef(name, quant) => {
                    let getter = format_ident!("get_attr_{}", name);
                    match quant {
                        Quantifier::One => {
                            quote![let #name = spirv_symbol_id(ctx, builder, self.#getter(ctx).clone())?;]
                        }
                        Quantifier::ZeroOrOne => quote! {
                            let #name = self.#getter(ctx)
                                .map(|it| spirv_symbol_id(ctx, builder, it.clone())).transpose()?;
                        },
                        Quantifier::ZeroOrMore => quote! {
                            let #name = from_vec_attr::<IdentifierAttr>(self.#getter(ctx)).into_iter()
                                .map(|it| spirv_symbol_id(ctx, builder, it.clone()))
                                .collect::<Result<Vec<_>>>()?;
                        },
                    }
                }
            }
        });
        getters.collect()
    }

    pub fn builder_args(&self, operands: &[OpdKind]) -> Vec<TokenStream> {
        let builder_args = operands.iter().map(|opd| match opd {
            OpdKind::ResultType => quote![result_ty],
            OpdKind::ResultValue => quote![Some(result)],
            OpdKind::Value(name, _) | OpdKind::Attribute(name, _, _) | OpdKind::ValueAttr(name, _) => {
                quote![#name]
            }
            OpdKind::MemoryAccess(name, name_align) => {
                quote![#name, #name_align]
            }
            OpdKind::SymbolRef(name, _) => {
                quote![#name]
            }
        });
        builder_args.collect()
    }

    pub fn constructor_args(&self, operands: &[OpdKind]) -> Vec<TokenStream> {
        let constructor_args = operands.iter().flat_map(|opd| match opd {
            OpdKind::ResultType => vec![quote![result_ty: TypeHandle]],
            OpdKind::ResultValue => vec![],
            OpdKind::Value(name, quant) => vec![match quant {
                Quantifier::One => quote![#name: Value],
                Quantifier::ZeroOrOne => quote![#name: Option<Value>],
                Quantifier::ZeroOrMore => quote![#name: Vec<Value>],
            }],
            OpdKind::Attribute(name, ty, Quantifier::One) => vec![quote![#name: impl Into<#ty>]],
            OpdKind::Attribute(_, _, Quantifier::ZeroOrOne) => vec![],
            OpdKind::Attribute(name, ty, Quantifier::ZeroOrMore) => vec![quote![#name: Vec<#ty>]],
            OpdKind::ValueAttr(name, ty) => vec![quote![#name: impl Into<#ty>]],
            OpdKind::MemoryAccess(name, name_align) => vec![
                quote![#name: impl Into<MemoryAccessAttr>],
                quote![#name_align: Option<u32>],
            ],
            OpdKind::SymbolRef(name, Quantifier::One) => vec![quote![#name: Identifier]],
            OpdKind::SymbolRef(_, Quantifier::ZeroOrOne) => vec![],
            OpdKind::SymbolRef(name, Quantifier::ZeroOrMore) => {
                vec![quote![#name: Vec<Identifier>]]
            }
        });
        constructor_args.collect()
    }

    pub fn constructor(&self, ty_name: &Ident, namespace: &str, operands: &[OpdKind], has_result: bool) -> TokenStream {
        let constructor_args = self.constructor_args(operands);
        let values = self.values(operands);
        let attr_setters = self.attr_setters(operands);
        let attr_get_set = self.generate_attr_get_set(namespace, operands);
        let result_ty = match has_result {
            true => quote![vec![result_ty]],
            false => quote![vec![]],
        };

        quote! {
            impl #ty_name {
                #[allow(clippy::too_many_arguments)]
                pub fn new(ctx: &mut Context, #(#constructor_args),*) -> Self {
                    let op = Self {
                        op: Operation::new(
                            ctx,
                            Self::get_concrete_op_info(),
                            #result_ty,
                            flat_vec![#(#values),*],
                            vec![],
                            0
                        ),
                    };
                    #(#attr_setters)*
                    op
                }

                #(#attr_get_set)*
            }
        }
    }

    pub fn to_spirv(
        &self,
        ty_name: &Ident,
        builder_name: &Ident,
        operands: &[OpdKind],
        has_result: bool,
    ) -> TokenStream {
        let getters = self.attr_getters(operands);
        let builder_args = self.builder_args(operands);

        let apply_decorations = match has_result {
            true => quote![crate::ops::apply_all_decorations(
                ctx, builder, self, result
            );],
            false => quote![],
        };

        quote! {
            #[op_interface_impl]
            impl ToSpirvOp for #ty_name {
                #[allow(unused, clippy::all)]
                fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<()> {
                    #[allow(unused)]
                    let op = self.get_operation().deref(ctx);
                    #(#getters)*
                    builder.#builder_name(#(#builder_args),*).into_pliron_result()?;
                    #apply_decorations
                    Ok(())
                }
            }
        }
    }

    pub fn op_format(&self, op_ty: &Ident) -> TokenStream {
        quote! {
            impl ::pliron::printable::Printable for #op_ty {
                fn fmt(
                    &self,
                    ctx: &::pliron::context::Context,
                    state: &::pliron::printable::State,
                    fmt: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    ::pliron::op::canonical_syntax_print(
                        ::pliron::op::OpObj::new(*self),
                        ctx,
                        state,
                        fmt,
                    )?;
                    Ok(())
                }
            }

            impl ::pliron::parsable::Parsable for #op_ty {
                type Arg = ::pliron::alloc::vec::Vec<
                    (::pliron::identifier::Identifier, ::pliron::location::Location),
                >;
                type Parsed = ::pliron::op::OpObj;
                fn parse<'__pliron_parse>(
                    state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
                    arg: Self::Arg,
                ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
                    use ::pliron::parsable::IntoParseResult;
                    use ::pliron::combine::Parser;
                    use ::pliron::input_err;
                    use ::pliron::location::Located;
                    let cur_loc = state_stream.loc();
                    use ::pliron::op::Op;
                    use ::pliron::operation::Operation;
                    use ::pliron::irfmt::parsers::{
                        process_parsed_ssa_defs, ssa_opd_parser, block_opd_parser, attr_parser,
                    };
                    ::pliron::op::canonical_syntax_parse::<Self>(state_stream, arg)
                }

                fn parser<'a>(_arg: Self::Arg) -> alloc::boxed::Box<dyn ::pliron::combine::Parser<::pliron::parsable::StateStream<'a>, Output = Self::Parsed, PartialState = ()> + 'a> {
                    todo!()
                }
            }
        }
    }
}
