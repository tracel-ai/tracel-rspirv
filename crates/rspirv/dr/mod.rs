//! Data representation of various SPIR-V language constructs.
//!
//! By language constructs, I mean general language concepts like module,
//! function, block, instruction, and operands. This is different
//! from the "control flow constructs" mentioned in the SPIR-V
//! [specification](https://goo.gl/YQRcZT).
//!
//! This data representation is designed to be lightweight; there are
//! no excessive sanity check or cross referrences within each language
//! construct. It is intended to be used as a plain data vehicle of
//! SPIR-V language constructs in the memory.
//!
//! Required components of a language construct may still be wrapped around
//! using `Option`; it makes the data representation more flexible since
//! we don't always require valid language constructs.
//!
//! Apart from definitions of various language constructs, this module also
//! provides a [loader](struct.Loader.html) for loading SPIR-V binaries
//! (together with the [parser](../binary/struct.Parser.html)) and a
//! [builder](struct.Builder.html) for building a SPIR-V data representation
//! interactively.

use crate::spirv::Word;

pub use self::build::{Builder, InsertPoint};
pub use self::constructs::{Block, Function, Instruction};
pub use self::constructs::{Module, ModuleHeader, Operand};
pub use self::loader::{load_bytes, load_words, Error, Loader};

mod build;
mod constructs;
mod loader;

pub mod autogen_glsl_std_450;
pub mod autogen_nonsemantic_debugprintf;
pub mod autogen_nonsemantic_shader_debuginfo_100;
pub mod autogen_opencl_std_100;

/// Appends an OpExtInstImport instruction and returns the result id.
pub(crate) fn ext_inst_import(b: &mut Builder, extended_inst_set: impl Into<String>) -> Word {
    let extended_inst_set: String = extended_inst_set.into();
    let operands = vec![Operand::LiteralString(extended_inst_set.clone())];
    let existing = b
        .module_ref()
        .ext_inst_imports
        .iter()
        .find(|inst| inst.operands == operands);
    match existing {
        Some(inst) => inst.result_id.unwrap(),
        None => b.ext_inst_import(extended_inst_set),
    }
}
