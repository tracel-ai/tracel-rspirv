use std::collections::HashMap;

use heck::ToSnekCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    pliron::{PlironGenerator, attrs::attr_const_name},
    structs::Quantifier,
};

const SKIP_DECORATIONS: &[&str] = &[
    "LinkageAttributes",
    "NodeSharesPayloadLimitsWithAMDX",
    "NodeMaxPayloadsAMDX",
    "PayloadNodeNameAMDX",
    "PayloadNodeBaseIndexAMDX",
    "PayloadNodeArraySizeAMDX",
    "ArrayStrideIdEXT",
    "OffsetIdEXT",
    "CounterBuffer",
    "FunctionRoundingModeINTEL",
    "FunctionDenormModeINTEL",
    "MergeALTERA",
    "BankBitsALTERA",
    "MathOpDSPModeALTERA",
    "AliasScopeINTEL",
    "NoAliasINTEL",
    "FunctionFloatingPointModeINTEL",
    "FPMaxErrorDecorationINTEL",
    "LatencyControlConstraintALTERA",
    "MMHostInterfaceReadWriteModeALTERA",
    "HostAccessINTEL",
    "InitModeALTERA",
    "ConditionalINTEL",
    "CacheControlLoadINTEL",
    "CacheControlStoreINTEL",
];

impl PlironGenerator {
    pub fn generate_decorations(&self) -> TokenStream {
        let decorations = self
            .grammar
            .operand_kinds
            .iter()
            .find(|it| it.kind == "Decoration")
            .unwrap();

        let decorations = decorations
            .enumerants
            .iter()
            .filter(|it| !SKIP_DECORATIONS.contains(&it.symbol.as_str()))
            .filter(|it| !it.symbol.ends_with("Id"))
            .map(|variant| {
                let has_params = !variant.parameters.is_empty();
                let (ty, as_params) = if !has_params {
                    (format_ident!("UnitAttr"), quote![vec![]])
                } else if variant.parameters.len() == 1 {
                    assert_eq!(
                        variant.parameters[0].quantifier,
                        Quantifier::One,
                        "decoration: {}",
                        variant.symbol
                    );
                    match variant.parameters[0].kind.as_str() {
                        "LiteralInteger" => (
                            format_ident!("LiteralIntegerAttr"),
                            quote![vec![Operand::LiteralBit32(attr.0)]],
                        ),
                        "BuiltIn" => (format_ident!("BuiltInAttr"), quote![vec![Operand::BuiltIn(attr.0)]]),
                        "IdScope" => (
                            format_ident!("ScopeAttr"),
                            quote![vec![Operand::IdRef(attr.spirv_id(ctx, builder)?)]],
                        ),
                        "FunctionParameterAttribute" => (format_ident!("VecAttr"), quote![todo!()]),
                        "FPRoundingMode" => (
                            format_ident!("FPRoundingModeAttr"),
                            quote![vec![Operand::FPRoundingMode(attr.0)]],
                        ),
                        "FPFastMathMode" => (
                            format_ident!("FPFastMathModeAttr"),
                            quote![vec![Operand::FPFastMathMode(attr.0)]],
                        ),
                        "LiteralString" => (
                            format_ident!("StringAttr"),
                            quote![vec![Operand::LiteralString(attr.as_str().to_string())]],
                        ),
                        other => {
                            panic!("Unhandled decoration param {} in {}", other, variant.symbol)
                        }
                    }
                } else {
                    panic!("Unhandled multi-parameter decoration {}", variant.symbol);
                };
                (variant, has_params, ty, as_params)
            })
            .collect::<Vec<_>>();

        fn decoration_key(symbol: &str) -> String {
            format!("spirv_decoration_{}", symbol.to_snek_case())
        }

        let attr_keys = decorations.iter().map(|(variant, ..)| {
            let const_ident = attr_const_name(&variant.symbol);
            let name = decoration_key(&variant.symbol);
            quote! {
                pub static #const_ident: ::pliron::std_deps::sync::LazyLock<::pliron::identifier::Identifier> =
                    ::pliron::std_deps::sync::LazyLock::new(|| #name.try_into().unwrap());
            }
        });

        let getters = decorations.iter().map(|(variant, has_params, ty, _)| {
            let const_ident = attr_const_name(&variant.symbol);
            if *has_params {
                let func_name = format_ident!("get_decoration_{}", variant.symbol.to_snek_case());
                quote! {
                    #[allow(non_snake_case)]
                    #[inline(never)]
                    pub fn #func_name<'a>(op: &dyn DecoratableOp, ctx: &'a Context) -> Option<Ref<'a, #ty>> {
                        Ref::filter_map(op.get_operation().deref(ctx), |op| {
                            op.attributes.get::<#ty>(&#const_ident)
                        })
                        .ok()
                    }
                }
            } else {
                let func_name = format_ident!("has_decoration_{}", variant.symbol.to_snek_case());
                quote! {
                    #[allow(non_snake_case)]
                    #[inline(never)]
                    pub fn #func_name(op: &dyn DecoratableOp, ctx: &Context) -> bool {
                        let op = op.get_operation().deref(ctx);
                        op.attributes.0.contains_key(&*#const_ident)
                    }
                }
            }
        });

        let trait_getters = decorations.iter().map(|(variant, has_params, ty, _)| {
            if *has_params {
                let func_name = format_ident!("get_decoration_{}", variant.symbol.to_snek_case());
                quote! {
                    #[allow(non_snake_case)]
                    fn #func_name<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, #ty>> where Self: Sized {
                        #func_name(self, ctx)
                    }
                }
            } else {
                let func_name = format_ident!("has_decoration_{}", variant.symbol.to_snek_case());
                quote! {
                    #[allow(non_snake_case)]
                    fn #func_name(&self, ctx: &Context) -> bool where Self: Sized {
                        #func_name(self, ctx)
                    }
                }
            }
        });

        let setters = decorations.iter().map(|(variant, has_params, ty, _)| {
            let const_ident = attr_const_name(&variant.symbol);
            let func_name = format_ident!("set_decoration_{}", variant.symbol.to_snek_case());
            if *has_params {
                quote! {
                    #[allow(non_snake_case)]
                    pub fn #func_name(op: &dyn DecoratableOp, ctx: &Context, value: #ty) {
                        op.get_operation().deref_mut(ctx).attributes.set(
                            #const_ident.clone(),
                            value,
                        );
                    }
                }
            } else {
                quote! {
                    #[allow(non_snake_case)]
                    pub fn #func_name(op: &dyn DecoratableOp, ctx: &Context) {
                        op.get_operation().deref_mut(ctx).attributes.set(
                            #const_ident.clone(),
                            UnitAttr::new(),
                        );
                    }
                }
            }
        });

        let trait_setters = decorations.iter().map(|(variant, has_params, ty, _)| {
            let func_name = format_ident!("set_decoration_{}", variant.symbol.to_snek_case());
            if *has_params {
                quote! {
                    #[allow(non_snake_case)]
                    fn #func_name(&self, ctx: &Context, value: #ty) where Self: Sized {
                        #func_name(self, ctx, value);
                    }
                }
            } else {
                quote! {
                    #[allow(non_snake_case)]
                    fn #func_name(&self, ctx: &Context) where Self: Sized {
                        #func_name(self, ctx);
                    }
                }
            }
        });

        let mut arg_kinds: HashMap<(Ident, String), (TokenStream, Vec<Ident>)> = HashMap::new();

        for (variant, _, ty, as_args) in decorations.iter() {
            arg_kinds
                .entry((ty.clone(), as_args.to_string()))
                .or_insert_with(|| (as_args.clone(), Default::default()))
                .1
                .push(format_ident!("{}", variant.symbol));
        }

        let as_args = arg_kinds.into_iter().map(|((ty, _), (as_args, variants))| {
            let first = &variants[0];
            let rest = variants.iter().skip(1).map(|variant| quote![| Decoration::#variant]);
            quote! {
                Decoration::#first #(#rest)* => {
                    #[allow(unused)]
                    let attr = self.value.downcast_ref::<#ty>().unwrap();
                    #as_args
                }
            }
        });

        let decorations_to_keys = decorations.iter().map(|(variant, ..)| {
            let name = format_ident!("{}", variant.symbol);
            let const_ident = attr_const_name(&variant.symbol);
            quote![Decoration::#name => &#const_ident]
        });

        let keys_to_decorations = decorations.iter().map(|(variant, ..)| {
            let key = decoration_key(&variant.symbol);
            let name = format_ident!("{}", variant.symbol);
            quote![#key => Some(Decoration::#name)]
        });

        let interface = quote! {
            #[op_interface]
            pub trait DecoratableOp {
                fn verify(_op: &dyn Op, _ctx: &Context) -> Result<()>
                where
                    Self: Sized,
                {
                    Ok(())
                }

                #(#trait_getters)*
                #(#trait_setters)*

                fn all_decorations(&self, ctx: &Context) -> Vec<(Decoration, Vec<Operand>)> where Self: Sized {
                    all_decorations_for_op(self, ctx)
                }
            }
        };

        quote! {
            use crate::{
                prelude::*,
                decorations::{DecorationInfo, DecorationExt, all_decorations_for_op}
            };
            use core::cell::Ref;

            #(#attr_keys)*
            #interface

            #(#getters)*
            #(#setters)*

            impl DecorationExt for Decoration {
                fn decoration_key(&self) -> &'static Identifier {
                    match self {
                        #(#decorations_to_keys,)*
                        _ => unimplemented!("Unsupported decoration"),
                    }
                }
            }

            pub fn decoration_for_key(identifier: &Identifier) -> Option<Decoration> {
                match identifier.as_str() {
                    #(#keys_to_decorations,)*
                    _ => None,
                }
            }

            impl DecorationInfo {
                pub fn as_operands(&self) -> Vec<Operand> {
                    match self.decoration {
                        #(#as_args),*
                        _ => unimplemented!("Unsupported decoration"),
                    }
                }
            }
        }
    }
}
