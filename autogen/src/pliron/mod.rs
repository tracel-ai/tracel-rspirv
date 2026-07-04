use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    structs::{ExtInstSetGrammar, Grammar},
    write,
};

mod attrs;
mod decorations;
mod extensions;
mod ops;

pub struct PlironGenerator {
    base_path: PathBuf,
    grammar: Grammar,
    extension_grammars: Vec<ExtensionGrammar>,
    operand_kinds: HashMap<String, Ident>,
}

pub struct ExtensionGrammar {
    prefix: &'static str,
    builder_prefix: &'static str,
    has_results: bool,
    grammar: ExtInstSetGrammar,
}

impl PlironGenerator {
    pub fn new(base_path: &Path, grammar: Grammar) -> Self {
        let extended_instruction_sets = [
            ("GLSL.std.450", "GL", "gl_", true),
            ("OpenCL.std.100", "CL", "cl_", true),
            ("NonSemantic.DebugPrintf", "DebugPrintfOp", "", false),
            // (
            //     "NonSemantic.Shader.DebugInfo.100",
            //     "DebugInfoOp",
            //     "shader_",
            //     true,
            // ),
        ];

        let extended_instruction_sets =
            extended_instruction_sets
                .iter()
                .map(|(ext, op_prefix, builder_prefix, with_result)| {
                    let grammar: ExtInstSetGrammar = serde_json::from_str(
                        &std::fs::read_to_string(base_path.join(format!(
                            "external/SPIRV-Headers/include/spirv/unified1/extinst.{}.grammar.json",
                            ext.to_lowercase()
                        )))
                        .unwrap(),
                    )
                    .unwrap();
                    ExtensionGrammar {
                        prefix: op_prefix,
                        builder_prefix,
                        has_results: *with_result,
                        grammar,
                    }
                });

        Self {
            base_path: base_path.to_path_buf(),
            grammar,
            extension_grammars: extended_instruction_sets.into_iter().collect(),
            operand_kinds: Default::default(),
        }
    }

    fn write_formatted<P: AsRef<Path> + ?Sized>(&self, file: &P, tokens: TokenStream) {
        let syn_file: syn::File = syn::parse2(tokens).unwrap();
        let path = self.base_path.join("../pliron-spirv/src").join(file);
        let formatted = prettyplease::unparse(&syn_file);
        write(&path, formatted);
    }
}

pub fn generate_pliron_dialect(base_path: &Path, grammar: Grammar) {
    let mut generator = PlironGenerator::new(base_path, grammar);
    let attrs = generator.generate_attributes();
    generator.write_formatted("autogen_attrs.rs", attrs);
    let decorations = generator.generate_decorations();
    generator.write_formatted("autogen_decorations.rs", decorations);
    let ops = generator.generate_ops();
    generator.write_formatted("autogen_ops.rs", ops);
    for ext in &generator.extension_grammars {
        let ops = generator.generate_ext_ops(ext);
        let file = format!("ext/autogen_{}.rs", ext.prefix.to_lowercase());
        generator.write_formatted(&file, ops);
    }
}
