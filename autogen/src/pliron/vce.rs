use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    dr::opt_version_tokens,
    pliron::{PlironGenerator, ops::OpdKind},
    structs::{Instruction, Quantifier},
    utils::as_ident,
};

type ExtraCap = (&'static str, fn() -> TokenStream);

const EXTRA_CAPABILITIES: &[ExtraCap] = &[
    ("OpSDot", integer_dot_product_extra_capabilities),
    ("OpUDot", integer_dot_product_extra_capabilities),
    ("OpSUDot", integer_dot_product_extra_capabilities),
    ("OpSDotAccSat", integer_dot_product_extra_capabilities),
    ("OpUDotAccSat", integer_dot_product_extra_capabilities),
    ("OpSUDotAccSat", integer_dot_product_extra_capabilities),
];

fn extra_capabilities(name: &str) -> TokenStream {
    if let Some((_, tokens)) = EXTRA_CAPABILITIES.iter().find(|(op, _)| *op == name) {
        tokens()
    } else {
        quote![]
    }
}

impl PlironGenerator {
    pub fn generate_vce_impl(&self, op: &Instruction, ty_name: &Ident, opds: &[OpdKind]) -> TokenStream {
        let version = opt_version_tokens(&op.version);
        let inst_caps = &op.capabilities;
        let extra_caps = extra_capabilities(&op.opname);
        let inst_extensions = &op.extensions;

        let mut min_version = quote! {
            #[allow(unused_mut)]
            let mut result: (u8, u8) = #version?;
        };
        let mut require_capabilities = quote! {
            #[allow(unused_mut)]
            let mut result = vec![];
        };
        let mut required_extensions = quote! {
            #[allow(unused_mut)]
            let mut result = vec![];
        };

        if !inst_caps.is_empty() {
            let caps = inst_caps.iter().map(|cap| {
                let ident = as_ident(cap);
                quote![Capability::#ident]
            });
            require_capabilities.extend(quote![result.push(vec![#(#caps),*]);]);
        }
        require_capabilities.extend(extra_caps);

        if !inst_extensions.is_empty() {
            required_extensions.extend(quote![result.push(vec![#(#inst_extensions),*]);]);
        }

        for opd in opds {
            match opd {
                OpdKind::Attribute(ident, _, Quantifier::One)
                | OpdKind::MemoryAccess(ident, _)
                | OpdKind::ValueAttr(ident, _) => {
                    let attr_get = format_ident!("get_attr_{}", ident);
                    min_version.extend(quote! {
                        result = result.max(Operand::from(self.#attr_get(ctx).clone().0).minimum_version()?);
                    });
                    require_capabilities.extend(quote! {
                        result.extend(Operand::from(self.#attr_get(ctx).clone().0).required_capabilities());
                    });
                    required_extensions.extend(quote! {
                        result.extend(Operand::from(self.#attr_get(ctx).clone().0).required_extensions());
                    });
                }
                OpdKind::Attribute(ident, _, Quantifier::ZeroOrOne) => {
                    let attr_get = format_ident!("get_attr_{}", ident);
                    min_version.extend(quote! {
                        if let Some(attr) = self.#attr_get(ctx) {
                            result = result.max(Operand::from(attr.clone().0).minimum_version()?);
                        }
                    });
                    require_capabilities.extend(quote! {
                        if let Some(attr) = self.#attr_get(ctx) {
                            result.extend(Operand::from(attr.clone().0).required_capabilities());
                        }
                    });
                    required_extensions.extend(quote! {
                        if let Some(attr) = self.#attr_get(ctx) {
                            result.extend(Operand::from(attr.clone().0).required_extensions());
                        }
                    });
                }
                OpdKind::Attribute(ident, ty, Quantifier::ZeroOrMore) => {
                    let attr_get = format_ident!("get_attr_{}", ident);
                    min_version.extend(quote! {
                        for attr in from_vec_attr::<#ty>(self.#attr_get(ctx)) {
                            result = result.max(Operand::from(attr.clone().0).minimum_version()?);
                        }
                    });
                    require_capabilities.extend(quote! {
                        for attr in from_vec_attr::<#ty>(self.#attr_get(ctx)) {
                            result.extend(Operand::from(attr.clone().0).required_capabilities());
                        }
                    });
                    required_extensions.extend(quote! {
                        for attr in from_vec_attr::<#ty>(self.#attr_get(ctx)) {
                            result.extend(Operand::from(attr.clone().0).required_extensions());
                        }
                    });
                }
                OpdKind::ResultType
                | OpdKind::ResultValue
                | OpdKind::Value(..)
                | OpdKind::SymbolRef(..)
                | OpdKind::StringRef(..) => {}
            }
        }

        quote! {
            #[op_interface_impl]
            impl VerCapExtOpInterface for #ty_name {
                #[allow(unused_variables)]
                fn min_version(&self, ctx: &Context) -> Option<(u8, u8)> {
                    #min_version
                    Some(result)
                }
                #[allow(unused_variables, clippy::vec_init_then_push)]
                fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
                    #required_extensions
                    result
                }
                #[allow(unused_variables, clippy::vec_init_then_push)]
                fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
                    #require_capabilities
                    result
                }
            }
        }
    }
}

fn integer_dot_product_extra_capabilities() -> TokenStream {
    quote! {
        if let Some(vector) = self
            .get_operand_vector_1(ctx)
            .get_type(ctx)
            .deref(ctx)
            .downcast_ref::<crate::types::VectorType>()
            && let Some(int) = vector
                .element_type
                .deref(ctx)
                .downcast_ref::<pliron::builtin::types::IntegerType>()
        {
            if int.width() == 8 {
                result.push(vec![Capability::DotProductInput4x8Bit]);
            } else {
                result.push(vec![Capability::DotProductInputAll]);
            }
        }
        if let Some(attr) = self.get_attr_packed_vector_format(ctx) {
            result.push(vec![Capability::DotProductInput4x8BitPacked]);
        }
    }
}
