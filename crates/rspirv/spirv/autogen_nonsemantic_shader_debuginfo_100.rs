// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[allow(unused)]
use bitflags::bitflags;
bitflags! { # [doc = "SPIR-V operand kind: [DebugInfoFlags](https://github.khronos.org/SPIRV-Registry/nonsemantic/NonSemantic.Shader.DebugInfo.100.html#_a_id_debug_info_flags_a_debug_info_flags)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct DebugInfoFlags : u32 { const NONE = 0u32 ; const FLAG_IS_PROTECTED = 1u32 ; const FLAG_IS_PRIVATE = 2u32 ; const FLAG_IS_PUBLIC = 3u32 ; const FLAG_IS_LOCAL = 4u32 ; const FLAG_IS_DEFINITION = 8u32 ; const FLAG_FWD_DECL = 16u32 ; const FLAG_ARTIFICIAL = 32u32 ; const FLAG_EXPLICIT = 64u32 ; const FLAG_PROTOTYPED = 128u32 ; const FLAG_OBJECT_POINTER = 256u32 ; const FLAG_STATIC_MEMBER = 512u32 ; const FLAG_INDIRECT_VARIABLE = 1024u32 ; const FLAG_L_VALUE_REFERENCE = 2048u32 ; const FLAG_R_VALUE_REFERENCE = 4096u32 ; const FLAG_IS_OPTIMIZED = 8192u32 ; const FLAG_IS_ENUM_CLASS = 16384u32 ; const FLAG_TYPE_PASS_BY_VALUE = 32768u32 ; const FLAG_TYPE_PASS_BY_REFERENCE = 65536u32 ; const FLAG_UNKNOWN_PHYSICAL_LAYOUT = 131072u32 ; } }
bitflags! { # [doc = "SPIR-V operand kind: [BuildIdentifierFlags](https://github.khronos.org/SPIRV-Registry/nonsemantic/NonSemantic.Shader.DebugInfo.100.html#_a_id_build_identifier_flags_a_build_identifier_flags)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct BuildIdentifierFlags : u32 { const IDENTIFIER_POSSIBLE_DUPLICATES = 1u32 ; } }
#[doc = "SPIR-V operand kind: [DebugBaseTypeAttributeEncoding](https://github.khronos.org/SPIRV-Registry/nonsemantic/NonSemantic.Shader.DebugInfo.100.html#_a_id_debug_base_type_attribute_encoding_a_debug_base_type_attribute_encoding)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum DebugBaseTypeAttributeEncoding {
    Unspecified = 0u32,
    Address = 1u32,
    Boolean = 2u32,
    Float = 3u32,
    Signed = 4u32,
    SignedChar = 5u32,
    Unsigned = 6u32,
    UnsignedChar = 7u32,
}
impl DebugBaseTypeAttributeEncoding {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=7u32 => unsafe {
                core::mem::transmute::<u32, DebugBaseTypeAttributeEncoding>(n)
            },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl DebugBaseTypeAttributeEncoding {}
impl core::str::FromStr for DebugBaseTypeAttributeEncoding {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Unspecified" => Self::Unspecified,
            "Address" => Self::Address,
            "Boolean" => Self::Boolean,
            "Float" => Self::Float,
            "Signed" => Self::Signed,
            "SignedChar" => Self::SignedChar,
            "Unsigned" => Self::Unsigned,
            "UnsignedChar" => Self::UnsignedChar,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [DebugCompositeType](https://github.khronos.org/SPIRV-Registry/nonsemantic/NonSemantic.Shader.DebugInfo.100.html#_a_id_debug_composite_type_a_debug_composite_type)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum DebugCompositeType {
    Class = 0u32,
    Structure = 1u32,
    Union = 2u32,
}
impl DebugCompositeType {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=2u32 => unsafe { core::mem::transmute::<u32, DebugCompositeType>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl DebugCompositeType {}
impl core::str::FromStr for DebugCompositeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Class" => Self::Class,
            "Structure" => Self::Structure,
            "Union" => Self::Union,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [DebugTypeQualifier](https://github.khronos.org/SPIRV-Registry/nonsemantic/NonSemantic.Shader.DebugInfo.100.html#_a_id_debug_type_qualifier_a_debug_type_qualifier)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum DebugTypeQualifier {
    ConstType = 0u32,
    VolatileType = 1u32,
    RestrictType = 2u32,
    AtomicType = 3u32,
}
impl DebugTypeQualifier {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=3u32 => unsafe { core::mem::transmute::<u32, DebugTypeQualifier>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl DebugTypeQualifier {}
impl core::str::FromStr for DebugTypeQualifier {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ConstType" => Self::ConstType,
            "VolatileType" => Self::VolatileType,
            "RestrictType" => Self::RestrictType,
            "AtomicType" => Self::AtomicType,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [DebugOperation](https://github.khronos.org/SPIRV-Registry/nonsemantic/NonSemantic.Shader.DebugInfo.100.html#_a_id_debug_operation_a_debug_operation)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum DebugOperation {
    Deref = 0u32,
    Plus = 1u32,
    Minus = 2u32,
    PlusUconst = 3u32,
    BitPiece = 4u32,
    Swap = 5u32,
    Xderef = 6u32,
    StackValue = 7u32,
    Constu = 8u32,
    Fragment = 9u32,
}
impl DebugOperation {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=9u32 => unsafe { core::mem::transmute::<u32, DebugOperation>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl DebugOperation {}
impl core::str::FromStr for DebugOperation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Deref" => Self::Deref,
            "Plus" => Self::Plus,
            "Minus" => Self::Minus,
            "PlusUconst" => Self::PlusUconst,
            "BitPiece" => Self::BitPiece,
            "Swap" => Self::Swap,
            "Xderef" => Self::Xderef,
            "StackValue" => Self::StackValue,
            "Constu" => Self::Constu,
            "Fragment" => Self::Fragment,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [DebugImportedEntity](https://github.khronos.org/SPIRV-Registry/nonsemantic/NonSemantic.Shader.DebugInfo.100.html#_a_id_debug_imported_entity_a_debug_imported_entity)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum DebugImportedEntity {
    ImportedModule = 0u32,
    ImportedDeclaration = 1u32,
}
impl DebugImportedEntity {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=1u32 => unsafe { core::mem::transmute::<u32, DebugImportedEntity>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl DebugImportedEntity {}
impl core::str::FromStr for DebugImportedEntity {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ImportedModule" => Self::ImportedModule,
            "ImportedDeclaration" => Self::ImportedDeclaration,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V extension [instructions](https://github.khronos.org/SPIRV-Registry/nonsemantic/NonSemantic.Shader.DebugInfo.100.html#_a_id_instructions_a_instructions) opcodes"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum DebugInfoOp {
    DebugInfoNone = 0u32,
    DebugCompilationUnit = 1u32,
    DebugTypeBasic = 2u32,
    DebugTypePointer = 3u32,
    DebugTypeQualifier = 4u32,
    DebugTypeArray = 5u32,
    DebugTypeVector = 6u32,
    DebugTypedef = 7u32,
    DebugTypeFunction = 8u32,
    DebugTypeEnum = 9u32,
    DebugTypeComposite = 10u32,
    DebugTypeMember = 11u32,
    DebugTypeInheritance = 12u32,
    DebugTypePtrToMember = 13u32,
    DebugTypeTemplate = 14u32,
    DebugTypeTemplateParameter = 15u32,
    DebugTypeTemplateTemplateParameter = 16u32,
    DebugTypeTemplateParameterPack = 17u32,
    DebugGlobalVariable = 18u32,
    DebugFunctionDeclaration = 19u32,
    DebugFunction = 20u32,
    DebugLexicalBlock = 21u32,
    DebugLexicalBlockDiscriminator = 22u32,
    DebugScope = 23u32,
    DebugNoScope = 24u32,
    DebugInlinedAt = 25u32,
    DebugLocalVariable = 26u32,
    DebugInlinedVariable = 27u32,
    DebugDeclare = 28u32,
    DebugValue = 29u32,
    DebugOperation = 30u32,
    DebugExpression = 31u32,
    DebugMacroDef = 32u32,
    DebugMacroUndef = 33u32,
    DebugImportedEntity = 34u32,
    DebugSource = 35u32,
    DebugFunctionDefinition = 101u32,
    DebugSourceContinued = 102u32,
    DebugLine = 103u32,
    DebugNoLine = 104u32,
    DebugBuildIdentifier = 105u32,
    DebugStoragePath = 106u32,
    DebugEntryPoint = 107u32,
    DebugTypeMatrix = 108u32,
}
impl DebugInfoOp {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=35u32 => unsafe { core::mem::transmute::<u32, DebugInfoOp>(n) },
            101u32..=108u32 => unsafe { core::mem::transmute::<u32, DebugInfoOp>(n) },
            _ => return None,
        })
    }
}
#[allow(clippy::upper_case_acronyms)]
#[allow(non_upper_case_globals)]
impl DebugInfoOp {
    pub const VERSION: u8 = 100u8;
    pub const REVISION: u8 = 6u8;
}
