// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[allow(unused)]
use bitflags::bitflags;
#[doc = "SPIR-V extension [instructions](https://github.com/KhronosGroup/Vulkan-ValidationLayers/blob/master/docs/debug_printf.md#_a_id_instructions_a_instructions) opcodes"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum DebugPrintfOp {
    DebugPrintf = 1u32,
}
impl DebugPrintfOp {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            1u32 => unsafe { core::mem::transmute::<u32, DebugPrintfOp>(1u32) },
            _ => return None,
        })
    }
}
#[allow(clippy::upper_case_acronyms)]
#[allow(non_upper_case_globals)]
impl DebugPrintfOp {
    pub const REVISION: u8 = 1u8;
}
