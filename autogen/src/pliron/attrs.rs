use core::fmt::Display;

use heck::{ToShoutySnekCase, ToSnekCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    pliron::{PlironGenerator, ops::OpdKind},
    structs::{Category, OperandKind, Quantifier},
};

const SKIP_ATTRS: &[&str] = &["Decoration"];

impl PlironGenerator {
    pub fn generate_attributes(&mut self) -> TokenStream {
        self.register_builtin_attrs();

        let mut attributes = vec![];
        for operand_kind in self.grammar.operand_kinds.clone() {
            attributes.push(self.generate_attribute(&operand_kind));
        }

        quote! {
            use tracel_rspirv::spirv::*;
            use crate::prelude::*;

            #(#attributes)*
        }
    }

    pub fn generate_attribute(&mut self, operand: &OperandKind) -> TokenStream {
        if !matches!(operand.category, Category::ValueEnum | Category::BitEnum)
            || SKIP_ATTRS.contains(&operand.kind.as_str())
        {
            return TokenStream::new();
        }

        let inner_ty = format_ident!("{}", operand.kind);
        let attr_ty = format_ident!("{}Attr", operand.kind);
        self.operand_kinds.insert(operand.kind.clone(), attr_ty.clone());

        let attr_name = format!("spirv.{}", operand.kind.to_snek_case());
        let attr = quote! {
            #[pliron_attr(name = #attr_name, format = "$0", verifier = "succ")]
            #[derive(PartialEq, Clone, Debug, Hash)]
            pub struct #attr_ty(pub #inner_ty);
        };

        quote! {
            #attr

            impl #attr_ty {
                pub fn new(value: #inner_ty) -> Self {
                    Self(value)
                }
            }

            impl From<#inner_ty> for #attr_ty {
                fn from(value: #inner_ty) -> Self {
                    Self(value)
                }
            }

            impl From<#attr_ty> for #inner_ty {
                fn from(value: #attr_ty) -> Self {
                    value.0
                }
            }
        }
    }

    fn register_builtin_attrs(&mut self) {
        self.operand_kinds
            .insert("LiteralInteger".into(), format_ident!("LiteralIntegerAttr"));
        self.operand_kinds
            .insert("LiteralString".into(), format_ident!("LiteralStringAttr"));
    }

    pub fn attr_idents(&self, namespace: &str, opds: &[OpdKind]) -> TokenStream {
        let module_name = format_ident!("{}", namespace);
        let idents = opds.iter().filter_map(|opd| match opd {
            OpdKind::Attribute(name, ..)
            | OpdKind::ValueAttr(name, ..)
            | OpdKind::SymbolRef(name, ..)
            | OpdKind::StringRef(name, ..) => {
                let const_name = attr_const_name(name);
                let qualified_name = attr_qualified_name(namespace, name).to_string();
                Some(quote! {
                    pub static #const_name: ::pliron::std_deps::sync::LazyLock<::pliron::identifier::Identifier> =
                        ::pliron::std_deps::sync::LazyLock::new(|| #qualified_name.try_into().unwrap());
                })
            }
            OpdKind::MemoryAccess(name, name_align) => {
                let const_name = attr_const_name(name);
                let const_name_align = attr_const_name(name_align);
                let qualified_name = attr_qualified_name(namespace, name).to_string();
                let qualified_name_align = attr_qualified_name(namespace, name_align).to_string();
                Some(quote! {
                    pub static #const_name: ::pliron::std_deps::sync::LazyLock<::pliron::identifier::Identifier> =
                        ::pliron::std_deps::sync::LazyLock::new(|| #qualified_name.try_into().unwrap());
                    pub static #const_name_align: ::pliron::std_deps::sync::LazyLock<::pliron::identifier::Identifier> =
                        ::pliron::std_deps::sync::LazyLock::new(|| #qualified_name_align.try_into().unwrap());
                })
            }
            OpdKind::ResultType | OpdKind::ResultValue | OpdKind::Value(..) => None,
        });
        quote! {
            mod #module_name {
                #(#idents)*
            }
        }
    }

    pub fn generate_attr_get_set(&self, namespace: &str, opds: &[OpdKind]) -> Vec<TokenStream> {
        let specs = opds.iter().flat_map(|opd| match opd {
            OpdKind::Attribute(name, ty, Quantifier::One) => {
                vec![(name, ty.clone(), true)]
            }
            OpdKind::Attribute(name, ty, Quantifier::ZeroOrOne) => {
                vec![(name, ty.clone(), false)]
            }
            OpdKind::Attribute(name, _, Quantifier::ZeroOrMore) => {
                vec![(name, format_ident!("VecAttr"), true)]
            }
            OpdKind::ValueAttr(name, ty) => {
                vec![(name, ty.clone(), true)]
            }
            OpdKind::StringRef(name, Quantifier::One) => {
                vec![(name, format_ident!("LiteralStringAttr"), true)]
            }
            OpdKind::StringRef(name, Quantifier::ZeroOrOne) => {
                vec![(name, format_ident!("LiteralStringAttr"), false)]
            }
            OpdKind::StringRef(name, Quantifier::ZeroOrMore) => {
                vec![(name, format_ident!("VecAttr"), true)]
            }
            OpdKind::MemoryAccess(name, name_align) => {
                vec![
                    (name, format_ident!("MemoryAccessAttr"), true),
                    (name_align, format_ident!("LiteralIntegerAttr"), false),
                ]
            }
            OpdKind::SymbolRef(name, Quantifier::One) => {
                vec![(name, format_ident!("IdentifierAttr"), true)]
            }
            OpdKind::SymbolRef(name, Quantifier::ZeroOrOne) => {
                vec![(name, format_ident!("IdentifierAttr"), false)]
            }
            OpdKind::SymbolRef(name, Quantifier::ZeroOrMore) => {
                vec![(name, format_ident!("VecAttr"), true)]
            }
            OpdKind::ResultType | OpdKind::ResultValue | OpdKind::Value(..) => vec![],
        });
        let get_set = specs.map(|(name, ty, required)| {
            let const_name = qualified_attr_const_name(namespace, name);
            let fn_name_get = format_ident!("get_attr_{}", name);
            let fn_name_set = format_ident!("set_attr_{}", name);
            let fn_comment_get = format!("Get a [Ref](core::cell::Ref) to the value of the attribute named `{name}`.");
            let fn_comment_set = format!("Set the value of the attribute named `{name}`.");
            let get = match required {
                true => quote! {
                    #[doc = #fn_comment_get]
                    pub fn #fn_name_get<'a>(&self, ctx: &'a ::pliron::context::Context)
                        -> ::core::cell::Ref<'a, #ty>
                    {
                        ::core::cell::Ref::map(self.op.deref(ctx), |op|
                            op.attributes.get::<#ty>(&#const_name).unwrap())
                    }
                },
                false => quote! {
                    #[doc = #fn_comment_get]
                    pub fn #fn_name_get<'a>(&self, ctx: &'a ::pliron::context::Context)
                        -> Option<::core::cell::Ref<'a, #ty>>
                    {
                        ::core::cell::Ref::filter_map(self.op.deref(ctx), |op|
                            op.attributes.get::<#ty>(&#const_name)).ok()
                    }
                },
            };
            let remove = match required {
                true => quote![],
                false => {
                    let fn_name_remove = format_ident!("remove_attr_{}", name);
                    let fn_comment_remove = format!("Remove the attribute named `{name}`.");
                    quote! {
                        #[doc = #fn_comment_remove]
                        pub fn #fn_name_remove(&self, ctx: &::pliron::context::Context)
                        {
                            self.op.deref_mut(ctx).attributes.0.remove(&*#const_name);
                        }
                    }
                }
            };

            quote! {
                #get

                #[doc = #fn_comment_set]
                pub fn #fn_name_set(&self, ctx: &::pliron::context::Context, value: #ty) {
                    self.op.deref_mut(ctx).attributes.set(#const_name.clone(), value);
                }

                #remove
            }
        });
        get_set.collect()
    }
}

pub fn attr_const_name(ident: &impl Display) -> Ident {
    format_ident!("ATTR_{}", ident.to_string().TO_SHOUTY_SNEK_CASE())
}

pub fn qualified_attr_const_name(namespace: &str, ident: &impl Display) -> TokenStream {
    let module_name = format_ident!("{}", namespace);
    let const_name = attr_const_name(ident);
    quote![#module_name::#const_name]
}

fn attr_qualified_name(namespace: &str, ident: &Ident) -> Ident {
    format_ident!("{namespace}_{}", ident)
}
