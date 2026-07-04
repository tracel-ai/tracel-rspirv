use heck::ToSnekCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    pliron::{attrs::attr_const_name, PlironGenerator},
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
                let (ty, as_params) = if variant.parameters.is_empty() {
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
                (variant, ty, as_params)
            })
            .collect::<Vec<_>>();

        let attr_keys = decorations.iter().map(|(variant, _, _)| {
            let const_ident = attr_const_name(&variant.symbol);
            let name = variant.symbol.to_snek_case();
            quote! {
                pub static #const_ident: ::pliron::std_deps::sync::LazyLock<::pliron::identifier::Identifier> =
                    ::pliron::std_deps::sync::LazyLock::new(|| #name.try_into().unwrap());
            }
        });

        let getters = decorations.iter().map(|(variant, ty, _)| {
            let const_ident = attr_const_name(&variant.symbol);
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
        });

        let trait_getters = decorations.iter().map(|(variant, ty, _)| {
            let func_name = format_ident!("get_decoration_{}", variant.symbol.to_snek_case());
            quote! {
                #[allow(non_snake_case)]
                fn #func_name<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, #ty>> where Self: Sized {
                    #func_name(self, ctx)
                }
            }
        });

        let setters = decorations.iter().map(|(variant, ty, _)| {
            let const_ident = attr_const_name(&variant.symbol);
            let func_name = format_ident!("set_decoration_{}", variant.symbol.to_snek_case());
            quote! {
                #[allow(non_snake_case)]
                pub fn #func_name(op: &dyn DecoratableOp, ctx: &Context, value: #ty) {
                    op.get_operation().deref_mut(ctx).attributes.set(
                        #const_ident.clone(),
                        value,
                    );
                }
            }
        });

        let trait_setters = decorations.iter().map(|(variant, ty, _)| {
            let func_name = format_ident!("set_decoration_{}", variant.symbol.to_snek_case());
            quote! {
                #[allow(non_snake_case)]
                fn #func_name(&self, ctx: &Context, value: #ty) where Self: Sized {
                    #func_name(self, ctx, value);
                }
            }
        });

        let decoration_list = decorations.iter().map(|(variant, _, as_args)| {
            let name = format_ident!("{}", variant.symbol);
            let getter = format_ident!("get_decoration_{}", variant.symbol.to_snek_case());

            quote![#[allow(unused)]
            if let Some(attr) = #getter(op, ctx) {
                out.push((Decoration::#name, #as_args));
            }]
        });

        let as_args = decorations.iter().map(|(variant, ty, as_args)| {
            let name = format_ident!("{}", variant.symbol);
            quote![Decoration::#name => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<#ty>().unwrap();
                #as_args
            }]
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
                    all_decorations(self, ctx)
                }
            }
        };

        quote! {
            use crate::{prelude::*, autogen_attrs::*, decorations::DecorationInfo};
            use core::cell::Ref;

            #(#attr_keys)*
            #interface

            #(#getters)*
            #(#setters)*

            pub fn all_decorations(op: &dyn DecoratableOp, ctx: &Context) -> Vec<(Decoration, Vec<Operand>)> {
                let mut out = Vec::new();
                #(#decoration_list)*
                out
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
