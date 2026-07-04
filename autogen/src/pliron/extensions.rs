use heck::{ToPascalCase, ToSnekCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    pliron::{ops::OpdKind, ExtensionGrammar, PlironGenerator},
    structs::Instruction,
    utils::as_ident,
};

impl PlironGenerator {
    pub fn generate_ext_ops(&self, extension: &ExtensionGrammar) -> TokenStream {
        let mut ops = vec![];
        for op in extension.grammar.instructions.iter() {
            ops.push(self.generate_ext_op(op, extension.prefix, extension.builder_prefix, extension.has_results));
        }

        quote! {
            #![allow(clippy::let_and_return, unused_imports)]
            use crate::prelude::*;
            use crate::attrs::*;

            #(#ops)*
        }
    }

    fn generate_ext_op(&self, op: &Instruction, prefix: &str, builder_prefix: &str, has_result: bool) -> TokenStream {
        let op_name = &op.opname;
        let ty_name = format_ident!("{}Op", op_name.to_pascal_case());
        let builder_name = match has_result {
            true => as_ident(&format!("{builder_prefix}{}_id", op_name.to_snek_case())),
            false => as_ident(&format!("{builder_prefix}{}", op_name.to_snek_case())),
        };
        let namespace = format!("spirv_{}_{}", prefix.to_lowercase(), op_name.to_snek_case());
        let op_name = format!("spirv.{prefix}.{}", op_name.to_snek_case());

        if matches!(op.operands.first(), Some(opd) if opd.kind == "IdResult") {
            panic!("Result with no type")
        }

        let mut operands = self.operands(op);
        self.validate_operands(&operands);

        if has_result {
            operands.insert(0, OpdKind::ResultValue);
            operands.insert(0, OpdKind::ResultType);
        }

        self.generate_op_impl(&op_name, &ty_name, &builder_name, &namespace, &operands, has_result)
    }
}
