// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[doc = "Data representation of a SPIR-V operand."]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum Operand {
    ImageOperands(spirv::ImageOperands),
    FPFastMathMode(spirv::FPFastMathMode),
    SelectionControl(spirv::SelectionControl),
    LoopControl(spirv::LoopControl),
    FunctionControl(spirv::FunctionControl),
    MemorySemantics(spirv::MemorySemantics),
    MemoryAccess(spirv::MemoryAccess),
    KernelProfilingInfo(spirv::KernelProfilingInfo),
    RayFlags(spirv::RayFlags),
    FragmentShadingRate(spirv::FragmentShadingRate),
    RawAccessChainOperands(spirv::RawAccessChainOperands),
    SourceLanguage(spirv::SourceLanguage),
    ExecutionModel(spirv::ExecutionModel),
    AddressingModel(spirv::AddressingModel),
    MemoryModel(spirv::MemoryModel),
    ExecutionMode(spirv::ExecutionMode),
    StorageClass(spirv::StorageClass),
    Dim(spirv::Dim),
    SamplerAddressingMode(spirv::SamplerAddressingMode),
    SamplerFilterMode(spirv::SamplerFilterMode),
    ImageFormat(spirv::ImageFormat),
    ImageChannelOrder(spirv::ImageChannelOrder),
    ImageChannelDataType(spirv::ImageChannelDataType),
    FPRoundingMode(spirv::FPRoundingMode),
    FPDenormMode(spirv::FPDenormMode),
    QuantizationModes(spirv::QuantizationModes),
    FPOperationMode(spirv::FPOperationMode),
    OverflowModes(spirv::OverflowModes),
    LinkageType(spirv::LinkageType),
    AccessQualifier(spirv::AccessQualifier),
    HostAccessQualifier(spirv::HostAccessQualifier),
    FunctionParameterAttribute(spirv::FunctionParameterAttribute),
    Decoration(spirv::Decoration),
    BuiltIn(spirv::BuiltIn),
    Scope(spirv::Scope),
    GroupOperation(spirv::GroupOperation),
    KernelEnqueueFlags(spirv::KernelEnqueueFlags),
    Capability(spirv::Capability),
    RayQueryIntersection(spirv::RayQueryIntersection),
    RayQueryCommittedIntersectionType(spirv::RayQueryCommittedIntersectionType),
    RayQueryCandidateIntersectionType(spirv::RayQueryCandidateIntersectionType),
    PackedVectorFormat(spirv::PackedVectorFormat),
    CooperativeMatrixOperands(spirv::CooperativeMatrixOperands),
    CooperativeMatrixLayout(spirv::CooperativeMatrixLayout),
    CooperativeMatrixUse(spirv::CooperativeMatrixUse),
    CooperativeMatrixReduce(spirv::CooperativeMatrixReduce),
    TensorClampMode(spirv::TensorClampMode),
    TensorAddressingOperands(spirv::TensorAddressingOperands),
    InitializationModeQualifier(spirv::InitializationModeQualifier),
    LoadCacheControl(spirv::LoadCacheControl),
    StoreCacheControl(spirv::StoreCacheControl),
    NamedMaximumNumberOfRegisters(spirv::NamedMaximumNumberOfRegisters),
    MatrixMultiplyAccumulateOperands(spirv::MatrixMultiplyAccumulateOperands),
    FPEncoding(spirv::FPEncoding),
    CooperativeVectorMatrixLayout(spirv::CooperativeVectorMatrixLayout),
    ComponentType(spirv::ComponentType),
    TensorOperands(spirv::TensorOperands),
    IdMemorySemantics(spirv::Word),
    IdScope(spirv::Word),
    IdRef(spirv::Word),
    LiteralBit32(u32),
    LiteralBit64(u64),
    LiteralExtInstInteger(u32),
    LiteralSpecConstantOpInteger(spirv::Op),
    LiteralString(String),
}
impl From<spirv::ImageOperands> for Operand {
    fn from(o: spirv::ImageOperands) -> Self {
        Self::ImageOperands(o)
    }
}
impl From<spirv::FPFastMathMode> for Operand {
    fn from(o: spirv::FPFastMathMode) -> Self {
        Self::FPFastMathMode(o)
    }
}
impl From<spirv::SelectionControl> for Operand {
    fn from(o: spirv::SelectionControl) -> Self {
        Self::SelectionControl(o)
    }
}
impl From<spirv::LoopControl> for Operand {
    fn from(o: spirv::LoopControl) -> Self {
        Self::LoopControl(o)
    }
}
impl From<spirv::FunctionControl> for Operand {
    fn from(o: spirv::FunctionControl) -> Self {
        Self::FunctionControl(o)
    }
}
impl From<spirv::MemorySemantics> for Operand {
    fn from(o: spirv::MemorySemantics) -> Self {
        Self::MemorySemantics(o)
    }
}
impl From<spirv::MemoryAccess> for Operand {
    fn from(o: spirv::MemoryAccess) -> Self {
        Self::MemoryAccess(o)
    }
}
impl From<spirv::KernelProfilingInfo> for Operand {
    fn from(o: spirv::KernelProfilingInfo) -> Self {
        Self::KernelProfilingInfo(o)
    }
}
impl From<spirv::RayFlags> for Operand {
    fn from(o: spirv::RayFlags) -> Self {
        Self::RayFlags(o)
    }
}
impl From<spirv::FragmentShadingRate> for Operand {
    fn from(o: spirv::FragmentShadingRate) -> Self {
        Self::FragmentShadingRate(o)
    }
}
impl From<spirv::RawAccessChainOperands> for Operand {
    fn from(o: spirv::RawAccessChainOperands) -> Self {
        Self::RawAccessChainOperands(o)
    }
}
impl From<spirv::SourceLanguage> for Operand {
    fn from(o: spirv::SourceLanguage) -> Self {
        Self::SourceLanguage(o)
    }
}
impl From<spirv::ExecutionModel> for Operand {
    fn from(o: spirv::ExecutionModel) -> Self {
        Self::ExecutionModel(o)
    }
}
impl From<spirv::AddressingModel> for Operand {
    fn from(o: spirv::AddressingModel) -> Self {
        Self::AddressingModel(o)
    }
}
impl From<spirv::MemoryModel> for Operand {
    fn from(o: spirv::MemoryModel) -> Self {
        Self::MemoryModel(o)
    }
}
impl From<spirv::ExecutionMode> for Operand {
    fn from(o: spirv::ExecutionMode) -> Self {
        Self::ExecutionMode(o)
    }
}
impl From<spirv::StorageClass> for Operand {
    fn from(o: spirv::StorageClass) -> Self {
        Self::StorageClass(o)
    }
}
impl From<spirv::Dim> for Operand {
    fn from(o: spirv::Dim) -> Self {
        Self::Dim(o)
    }
}
impl From<spirv::SamplerAddressingMode> for Operand {
    fn from(o: spirv::SamplerAddressingMode) -> Self {
        Self::SamplerAddressingMode(o)
    }
}
impl From<spirv::SamplerFilterMode> for Operand {
    fn from(o: spirv::SamplerFilterMode) -> Self {
        Self::SamplerFilterMode(o)
    }
}
impl From<spirv::ImageFormat> for Operand {
    fn from(o: spirv::ImageFormat) -> Self {
        Self::ImageFormat(o)
    }
}
impl From<spirv::ImageChannelOrder> for Operand {
    fn from(o: spirv::ImageChannelOrder) -> Self {
        Self::ImageChannelOrder(o)
    }
}
impl From<spirv::ImageChannelDataType> for Operand {
    fn from(o: spirv::ImageChannelDataType) -> Self {
        Self::ImageChannelDataType(o)
    }
}
impl From<spirv::FPRoundingMode> for Operand {
    fn from(o: spirv::FPRoundingMode) -> Self {
        Self::FPRoundingMode(o)
    }
}
impl From<spirv::FPDenormMode> for Operand {
    fn from(o: spirv::FPDenormMode) -> Self {
        Self::FPDenormMode(o)
    }
}
impl From<spirv::QuantizationModes> for Operand {
    fn from(o: spirv::QuantizationModes) -> Self {
        Self::QuantizationModes(o)
    }
}
impl From<spirv::FPOperationMode> for Operand {
    fn from(o: spirv::FPOperationMode) -> Self {
        Self::FPOperationMode(o)
    }
}
impl From<spirv::OverflowModes> for Operand {
    fn from(o: spirv::OverflowModes) -> Self {
        Self::OverflowModes(o)
    }
}
impl From<spirv::LinkageType> for Operand {
    fn from(o: spirv::LinkageType) -> Self {
        Self::LinkageType(o)
    }
}
impl From<spirv::AccessQualifier> for Operand {
    fn from(o: spirv::AccessQualifier) -> Self {
        Self::AccessQualifier(o)
    }
}
impl From<spirv::HostAccessQualifier> for Operand {
    fn from(o: spirv::HostAccessQualifier) -> Self {
        Self::HostAccessQualifier(o)
    }
}
impl From<spirv::FunctionParameterAttribute> for Operand {
    fn from(o: spirv::FunctionParameterAttribute) -> Self {
        Self::FunctionParameterAttribute(o)
    }
}
impl From<spirv::Decoration> for Operand {
    fn from(o: spirv::Decoration) -> Self {
        Self::Decoration(o)
    }
}
impl From<spirv::BuiltIn> for Operand {
    fn from(o: spirv::BuiltIn) -> Self {
        Self::BuiltIn(o)
    }
}
impl From<spirv::Scope> for Operand {
    fn from(o: spirv::Scope) -> Self {
        Self::Scope(o)
    }
}
impl From<spirv::GroupOperation> for Operand {
    fn from(o: spirv::GroupOperation) -> Self {
        Self::GroupOperation(o)
    }
}
impl From<spirv::KernelEnqueueFlags> for Operand {
    fn from(o: spirv::KernelEnqueueFlags) -> Self {
        Self::KernelEnqueueFlags(o)
    }
}
impl From<spirv::Capability> for Operand {
    fn from(o: spirv::Capability) -> Self {
        Self::Capability(o)
    }
}
impl From<spirv::RayQueryIntersection> for Operand {
    fn from(o: spirv::RayQueryIntersection) -> Self {
        Self::RayQueryIntersection(o)
    }
}
impl From<spirv::RayQueryCommittedIntersectionType> for Operand {
    fn from(o: spirv::RayQueryCommittedIntersectionType) -> Self {
        Self::RayQueryCommittedIntersectionType(o)
    }
}
impl From<spirv::RayQueryCandidateIntersectionType> for Operand {
    fn from(o: spirv::RayQueryCandidateIntersectionType) -> Self {
        Self::RayQueryCandidateIntersectionType(o)
    }
}
impl From<spirv::PackedVectorFormat> for Operand {
    fn from(o: spirv::PackedVectorFormat) -> Self {
        Self::PackedVectorFormat(o)
    }
}
impl From<spirv::CooperativeMatrixOperands> for Operand {
    fn from(o: spirv::CooperativeMatrixOperands) -> Self {
        Self::CooperativeMatrixOperands(o)
    }
}
impl From<spirv::CooperativeMatrixLayout> for Operand {
    fn from(o: spirv::CooperativeMatrixLayout) -> Self {
        Self::CooperativeMatrixLayout(o)
    }
}
impl From<spirv::CooperativeMatrixUse> for Operand {
    fn from(o: spirv::CooperativeMatrixUse) -> Self {
        Self::CooperativeMatrixUse(o)
    }
}
impl From<spirv::CooperativeMatrixReduce> for Operand {
    fn from(o: spirv::CooperativeMatrixReduce) -> Self {
        Self::CooperativeMatrixReduce(o)
    }
}
impl From<spirv::TensorClampMode> for Operand {
    fn from(o: spirv::TensorClampMode) -> Self {
        Self::TensorClampMode(o)
    }
}
impl From<spirv::TensorAddressingOperands> for Operand {
    fn from(o: spirv::TensorAddressingOperands) -> Self {
        Self::TensorAddressingOperands(o)
    }
}
impl From<spirv::InitializationModeQualifier> for Operand {
    fn from(o: spirv::InitializationModeQualifier) -> Self {
        Self::InitializationModeQualifier(o)
    }
}
impl From<spirv::LoadCacheControl> for Operand {
    fn from(o: spirv::LoadCacheControl) -> Self {
        Self::LoadCacheControl(o)
    }
}
impl From<spirv::StoreCacheControl> for Operand {
    fn from(o: spirv::StoreCacheControl) -> Self {
        Self::StoreCacheControl(o)
    }
}
impl From<spirv::NamedMaximumNumberOfRegisters> for Operand {
    fn from(o: spirv::NamedMaximumNumberOfRegisters) -> Self {
        Self::NamedMaximumNumberOfRegisters(o)
    }
}
impl From<spirv::MatrixMultiplyAccumulateOperands> for Operand {
    fn from(o: spirv::MatrixMultiplyAccumulateOperands) -> Self {
        Self::MatrixMultiplyAccumulateOperands(o)
    }
}
impl From<spirv::FPEncoding> for Operand {
    fn from(o: spirv::FPEncoding) -> Self {
        Self::FPEncoding(o)
    }
}
impl From<spirv::CooperativeVectorMatrixLayout> for Operand {
    fn from(o: spirv::CooperativeVectorMatrixLayout) -> Self {
        Self::CooperativeVectorMatrixLayout(o)
    }
}
impl From<spirv::ComponentType> for Operand {
    fn from(o: spirv::ComponentType) -> Self {
        Self::ComponentType(o)
    }
}
impl From<spirv::TensorOperands> for Operand {
    fn from(o: spirv::TensorOperands) -> Self {
        Self::TensorOperands(o)
    }
}
impl From<u32> for Operand {
    fn from(o: u32) -> Self {
        Self::LiteralBit32(o)
    }
}
impl From<u64> for Operand {
    fn from(o: u64) -> Self {
        Self::LiteralBit64(o)
    }
}
impl From<spirv::Op> for Operand {
    fn from(o: spirv::Op) -> Self {
        Self::LiteralSpecConstantOpInteger(o)
    }
}
impl From<String> for Operand {
    fn from(o: String) -> Self {
        Self::LiteralString(o)
    }
}
impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::ImageOperands(ref v) => write!(f, "{:?}", v),
            Operand::FPFastMathMode(ref v) => write!(f, "{:?}", v),
            Operand::SelectionControl(ref v) => write!(f, "{:?}", v),
            Operand::LoopControl(ref v) => write!(f, "{:?}", v),
            Operand::FunctionControl(ref v) => write!(f, "{:?}", v),
            Operand::MemorySemantics(ref v) => write!(f, "{:?}", v),
            Operand::MemoryAccess(ref v) => write!(f, "{:?}", v),
            Operand::KernelProfilingInfo(ref v) => write!(f, "{:?}", v),
            Operand::RayFlags(ref v) => write!(f, "{:?}", v),
            Operand::FragmentShadingRate(ref v) => write!(f, "{:?}", v),
            Operand::RawAccessChainOperands(ref v) => write!(f, "{:?}", v),
            Operand::SourceLanguage(ref v) => write!(f, "{:?}", v),
            Operand::ExecutionModel(ref v) => write!(f, "{:?}", v),
            Operand::AddressingModel(ref v) => write!(f, "{:?}", v),
            Operand::MemoryModel(ref v) => write!(f, "{:?}", v),
            Operand::ExecutionMode(ref v) => write!(f, "{:?}", v),
            Operand::StorageClass(ref v) => write!(f, "{:?}", v),
            Operand::Dim(ref v) => write!(f, "{}", &format!("{:?}", v)[3..]),
            Operand::SamplerAddressingMode(ref v) => write!(f, "{:?}", v),
            Operand::SamplerFilterMode(ref v) => write!(f, "{:?}", v),
            Operand::ImageFormat(ref v) => write!(f, "{:?}", v),
            Operand::ImageChannelOrder(ref v) => write!(f, "{:?}", v),
            Operand::ImageChannelDataType(ref v) => write!(f, "{:?}", v),
            Operand::FPRoundingMode(ref v) => write!(f, "{:?}", v),
            Operand::FPDenormMode(ref v) => write!(f, "{:?}", v),
            Operand::QuantizationModes(ref v) => write!(f, "{:?}", v),
            Operand::FPOperationMode(ref v) => write!(f, "{:?}", v),
            Operand::OverflowModes(ref v) => write!(f, "{:?}", v),
            Operand::LinkageType(ref v) => write!(f, "{:?}", v),
            Operand::AccessQualifier(ref v) => write!(f, "{:?}", v),
            Operand::HostAccessQualifier(ref v) => write!(f, "{:?}", v),
            Operand::FunctionParameterAttribute(ref v) => write!(f, "{:?}", v),
            Operand::Decoration(ref v) => write!(f, "{:?}", v),
            Operand::BuiltIn(ref v) => write!(f, "{:?}", v),
            Operand::Scope(ref v) => write!(f, "{:?}", v),
            Operand::GroupOperation(ref v) => write!(f, "{:?}", v),
            Operand::KernelEnqueueFlags(ref v) => write!(f, "{:?}", v),
            Operand::Capability(ref v) => write!(f, "{:?}", v),
            Operand::RayQueryIntersection(ref v) => write!(f, "{:?}", v),
            Operand::RayQueryCommittedIntersectionType(ref v) => write!(f, "{:?}", v),
            Operand::RayQueryCandidateIntersectionType(ref v) => write!(f, "{:?}", v),
            Operand::PackedVectorFormat(ref v) => write!(f, "{:?}", v),
            Operand::CooperativeMatrixOperands(ref v) => write!(f, "{:?}", v),
            Operand::CooperativeMatrixLayout(ref v) => write!(f, "{:?}", v),
            Operand::CooperativeMatrixUse(ref v) => write!(f, "{:?}", v),
            Operand::CooperativeMatrixReduce(ref v) => write!(f, "{:?}", v),
            Operand::TensorClampMode(ref v) => write!(f, "{:?}", v),
            Operand::TensorAddressingOperands(ref v) => write!(f, "{:?}", v),
            Operand::InitializationModeQualifier(ref v) => write!(f, "{:?}", v),
            Operand::LoadCacheControl(ref v) => write!(f, "{:?}", v),
            Operand::StoreCacheControl(ref v) => write!(f, "{:?}", v),
            Operand::NamedMaximumNumberOfRegisters(ref v) => write!(f, "{:?}", v),
            Operand::MatrixMultiplyAccumulateOperands(ref v) => write!(f, "{:?}", v),
            Operand::FPEncoding(ref v) => write!(f, "{:?}", v),
            Operand::CooperativeVectorMatrixLayout(ref v) => write!(f, "{:?}", v),
            Operand::ComponentType(ref v) => write!(f, "{:?}", v),
            Operand::IdMemorySemantics(ref v) => write!(f, "%{}", v),
            Operand::IdScope(ref v) => write!(f, "%{}", v),
            Operand::IdRef(ref v) => write!(f, "%{}", v),
            Operand::LiteralString(ref v) => write!(f, "{:?}", v),
            Operand::LiteralExtInstInteger(ref v) => write!(f, "{:?}", v),
            Operand::LiteralSpecConstantOpInteger(ref v) => write!(f, "{:?}", v),
            Operand::TensorOperands(ref v) => write!(f, "{:?}", v),
            Operand::LiteralBit32(ref v) => write!(f, "{:?}", v),
            Operand::LiteralBit64(ref v) => write!(f, "{:?}", v),
        }
    }
}
impl Operand {
    pub fn unwrap_image_operands(&self) -> spirv::ImageOperands {
        match *self {
            Self::ImageOperands(v) => v,
            ref other => panic!("Expected Operand::ImageOperands, got {} instead", other),
        }
    }
    pub fn unwrap_fp_fast_math_mode(&self) -> spirv::FPFastMathMode {
        match *self {
            Self::FPFastMathMode(v) => v,
            ref other => panic!("Expected Operand::FPFastMathMode, got {} instead", other),
        }
    }
    pub fn unwrap_selection_control(&self) -> spirv::SelectionControl {
        match *self {
            Self::SelectionControl(v) => v,
            ref other => panic!("Expected Operand::SelectionControl, got {} instead", other),
        }
    }
    pub fn unwrap_loop_control(&self) -> spirv::LoopControl {
        match *self {
            Self::LoopControl(v) => v,
            ref other => panic!("Expected Operand::LoopControl, got {} instead", other),
        }
    }
    pub fn unwrap_function_control(&self) -> spirv::FunctionControl {
        match *self {
            Self::FunctionControl(v) => v,
            ref other => panic!("Expected Operand::FunctionControl, got {} instead", other),
        }
    }
    pub fn unwrap_memory_semantics(&self) -> spirv::MemorySemantics {
        match *self {
            Self::MemorySemantics(v) => v,
            ref other => panic!("Expected Operand::MemorySemantics, got {} instead", other),
        }
    }
    pub fn unwrap_memory_access(&self) -> spirv::MemoryAccess {
        match *self {
            Self::MemoryAccess(v) => v,
            ref other => panic!("Expected Operand::MemoryAccess, got {} instead", other),
        }
    }
    pub fn unwrap_kernel_profiling_info(&self) -> spirv::KernelProfilingInfo {
        match *self {
            Self::KernelProfilingInfo(v) => v,
            ref other => panic!("Expected Operand::KernelProfilingInfo, got {} instead", other),
        }
    }
    pub fn unwrap_ray_flags(&self) -> spirv::RayFlags {
        match *self {
            Self::RayFlags(v) => v,
            ref other => panic!("Expected Operand::RayFlags, got {} instead", other),
        }
    }
    pub fn unwrap_fragment_shading_rate(&self) -> spirv::FragmentShadingRate {
        match *self {
            Self::FragmentShadingRate(v) => v,
            ref other => panic!("Expected Operand::FragmentShadingRate, got {} instead", other),
        }
    }
    pub fn unwrap_raw_access_chain_operands(&self) -> spirv::RawAccessChainOperands {
        match *self {
            Self::RawAccessChainOperands(v) => v,
            ref other => panic!("Expected Operand::RawAccessChainOperands, got {} instead", other),
        }
    }
    pub fn unwrap_source_language(&self) -> spirv::SourceLanguage {
        match *self {
            Self::SourceLanguage(v) => v,
            ref other => panic!("Expected Operand::SourceLanguage, got {} instead", other),
        }
    }
    pub fn unwrap_execution_model(&self) -> spirv::ExecutionModel {
        match *self {
            Self::ExecutionModel(v) => v,
            ref other => panic!("Expected Operand::ExecutionModel, got {} instead", other),
        }
    }
    pub fn unwrap_addressing_model(&self) -> spirv::AddressingModel {
        match *self {
            Self::AddressingModel(v) => v,
            ref other => panic!("Expected Operand::AddressingModel, got {} instead", other),
        }
    }
    pub fn unwrap_memory_model(&self) -> spirv::MemoryModel {
        match *self {
            Self::MemoryModel(v) => v,
            ref other => panic!("Expected Operand::MemoryModel, got {} instead", other),
        }
    }
    pub fn unwrap_execution_mode(&self) -> spirv::ExecutionMode {
        match *self {
            Self::ExecutionMode(v) => v,
            ref other => panic!("Expected Operand::ExecutionMode, got {} instead", other),
        }
    }
    pub fn unwrap_storage_class(&self) -> spirv::StorageClass {
        match *self {
            Self::StorageClass(v) => v,
            ref other => panic!("Expected Operand::StorageClass, got {} instead", other),
        }
    }
    pub fn unwrap_dim(&self) -> spirv::Dim {
        match *self {
            Self::Dim(v) => v,
            ref other => panic!("Expected Operand::Dim, got {} instead", other),
        }
    }
    pub fn unwrap_sampler_addressing_mode(&self) -> spirv::SamplerAddressingMode {
        match *self {
            Self::SamplerAddressingMode(v) => v,
            ref other => panic!("Expected Operand::SamplerAddressingMode, got {} instead", other),
        }
    }
    pub fn unwrap_sampler_filter_mode(&self) -> spirv::SamplerFilterMode {
        match *self {
            Self::SamplerFilterMode(v) => v,
            ref other => panic!("Expected Operand::SamplerFilterMode, got {} instead", other),
        }
    }
    pub fn unwrap_image_format(&self) -> spirv::ImageFormat {
        match *self {
            Self::ImageFormat(v) => v,
            ref other => panic!("Expected Operand::ImageFormat, got {} instead", other),
        }
    }
    pub fn unwrap_image_channel_order(&self) -> spirv::ImageChannelOrder {
        match *self {
            Self::ImageChannelOrder(v) => v,
            ref other => panic!("Expected Operand::ImageChannelOrder, got {} instead", other),
        }
    }
    pub fn unwrap_image_channel_data_type(&self) -> spirv::ImageChannelDataType {
        match *self {
            Self::ImageChannelDataType(v) => v,
            ref other => panic!("Expected Operand::ImageChannelDataType, got {} instead", other),
        }
    }
    pub fn unwrap_fp_rounding_mode(&self) -> spirv::FPRoundingMode {
        match *self {
            Self::FPRoundingMode(v) => v,
            ref other => panic!("Expected Operand::FPRoundingMode, got {} instead", other),
        }
    }
    pub fn unwrap_fp_denorm_mode(&self) -> spirv::FPDenormMode {
        match *self {
            Self::FPDenormMode(v) => v,
            ref other => panic!("Expected Operand::FPDenormMode, got {} instead", other),
        }
    }
    pub fn unwrap_quantization_modes(&self) -> spirv::QuantizationModes {
        match *self {
            Self::QuantizationModes(v) => v,
            ref other => panic!("Expected Operand::QuantizationModes, got {} instead", other),
        }
    }
    pub fn unwrap_fp_operation_mode(&self) -> spirv::FPOperationMode {
        match *self {
            Self::FPOperationMode(v) => v,
            ref other => panic!("Expected Operand::FPOperationMode, got {} instead", other),
        }
    }
    pub fn unwrap_overflow_modes(&self) -> spirv::OverflowModes {
        match *self {
            Self::OverflowModes(v) => v,
            ref other => panic!("Expected Operand::OverflowModes, got {} instead", other),
        }
    }
    pub fn unwrap_linkage_type(&self) -> spirv::LinkageType {
        match *self {
            Self::LinkageType(v) => v,
            ref other => panic!("Expected Operand::LinkageType, got {} instead", other),
        }
    }
    pub fn unwrap_access_qualifier(&self) -> spirv::AccessQualifier {
        match *self {
            Self::AccessQualifier(v) => v,
            ref other => panic!("Expected Operand::AccessQualifier, got {} instead", other),
        }
    }
    pub fn unwrap_host_access_qualifier(&self) -> spirv::HostAccessQualifier {
        match *self {
            Self::HostAccessQualifier(v) => v,
            ref other => panic!("Expected Operand::HostAccessQualifier, got {} instead", other),
        }
    }
    pub fn unwrap_function_parameter_attribute(&self) -> spirv::FunctionParameterAttribute {
        match *self {
            Self::FunctionParameterAttribute(v) => v,
            ref other => panic!("Expected Operand::FunctionParameterAttribute, got {} instead", other),
        }
    }
    pub fn unwrap_decoration(&self) -> spirv::Decoration {
        match *self {
            Self::Decoration(v) => v,
            ref other => panic!("Expected Operand::Decoration, got {} instead", other),
        }
    }
    pub fn unwrap_built_in(&self) -> spirv::BuiltIn {
        match *self {
            Self::BuiltIn(v) => v,
            ref other => panic!("Expected Operand::BuiltIn, got {} instead", other),
        }
    }
    pub fn unwrap_scope(&self) -> spirv::Scope {
        match *self {
            Self::Scope(v) => v,
            ref other => panic!("Expected Operand::Scope, got {} instead", other),
        }
    }
    pub fn unwrap_group_operation(&self) -> spirv::GroupOperation {
        match *self {
            Self::GroupOperation(v) => v,
            ref other => panic!("Expected Operand::GroupOperation, got {} instead", other),
        }
    }
    pub fn unwrap_kernel_enqueue_flags(&self) -> spirv::KernelEnqueueFlags {
        match *self {
            Self::KernelEnqueueFlags(v) => v,
            ref other => panic!("Expected Operand::KernelEnqueueFlags, got {} instead", other),
        }
    }
    pub fn unwrap_capability(&self) -> spirv::Capability {
        match *self {
            Self::Capability(v) => v,
            ref other => panic!("Expected Operand::Capability, got {} instead", other),
        }
    }
    pub fn unwrap_ray_query_intersection(&self) -> spirv::RayQueryIntersection {
        match *self {
            Self::RayQueryIntersection(v) => v,
            ref other => panic!("Expected Operand::RayQueryIntersection, got {} instead", other),
        }
    }
    pub fn unwrap_ray_query_committed_intersection_type(&self) -> spirv::RayQueryCommittedIntersectionType {
        match *self {
            Self::RayQueryCommittedIntersectionType(v) => v,
            ref other => panic!(
                "Expected Operand::RayQueryCommittedIntersectionType, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_ray_query_candidate_intersection_type(&self) -> spirv::RayQueryCandidateIntersectionType {
        match *self {
            Self::RayQueryCandidateIntersectionType(v) => v,
            ref other => panic!(
                "Expected Operand::RayQueryCandidateIntersectionType, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_packed_vector_format(&self) -> spirv::PackedVectorFormat {
        match *self {
            Self::PackedVectorFormat(v) => v,
            ref other => panic!("Expected Operand::PackedVectorFormat, got {} instead", other),
        }
    }
    pub fn unwrap_cooperative_matrix_operands(&self) -> spirv::CooperativeMatrixOperands {
        match *self {
            Self::CooperativeMatrixOperands(v) => v,
            ref other => panic!("Expected Operand::CooperativeMatrixOperands, got {} instead", other),
        }
    }
    pub fn unwrap_cooperative_matrix_layout(&self) -> spirv::CooperativeMatrixLayout {
        match *self {
            Self::CooperativeMatrixLayout(v) => v,
            ref other => panic!("Expected Operand::CooperativeMatrixLayout, got {} instead", other),
        }
    }
    pub fn unwrap_cooperative_matrix_use(&self) -> spirv::CooperativeMatrixUse {
        match *self {
            Self::CooperativeMatrixUse(v) => v,
            ref other => panic!("Expected Operand::CooperativeMatrixUse, got {} instead", other),
        }
    }
    pub fn unwrap_cooperative_matrix_reduce(&self) -> spirv::CooperativeMatrixReduce {
        match *self {
            Self::CooperativeMatrixReduce(v) => v,
            ref other => panic!("Expected Operand::CooperativeMatrixReduce, got {} instead", other),
        }
    }
    pub fn unwrap_tensor_clamp_mode(&self) -> spirv::TensorClampMode {
        match *self {
            Self::TensorClampMode(v) => v,
            ref other => panic!("Expected Operand::TensorClampMode, got {} instead", other),
        }
    }
    pub fn unwrap_tensor_addressing_operands(&self) -> spirv::TensorAddressingOperands {
        match *self {
            Self::TensorAddressingOperands(v) => v,
            ref other => panic!("Expected Operand::TensorAddressingOperands, got {} instead", other),
        }
    }
    pub fn unwrap_initialization_mode_qualifier(&self) -> spirv::InitializationModeQualifier {
        match *self {
            Self::InitializationModeQualifier(v) => v,
            ref other => panic!("Expected Operand::InitializationModeQualifier, got {} instead", other),
        }
    }
    pub fn unwrap_load_cache_control(&self) -> spirv::LoadCacheControl {
        match *self {
            Self::LoadCacheControl(v) => v,
            ref other => panic!("Expected Operand::LoadCacheControl, got {} instead", other),
        }
    }
    pub fn unwrap_store_cache_control(&self) -> spirv::StoreCacheControl {
        match *self {
            Self::StoreCacheControl(v) => v,
            ref other => panic!("Expected Operand::StoreCacheControl, got {} instead", other),
        }
    }
    pub fn unwrap_named_maximum_number_of_registers(&self) -> spirv::NamedMaximumNumberOfRegisters {
        match *self {
            Self::NamedMaximumNumberOfRegisters(v) => v,
            ref other => panic!("Expected Operand::NamedMaximumNumberOfRegisters, got {} instead", other),
        }
    }
    pub fn unwrap_matrix_multiply_accumulate_operands(&self) -> spirv::MatrixMultiplyAccumulateOperands {
        match *self {
            Self::MatrixMultiplyAccumulateOperands(v) => v,
            ref other => panic!(
                "Expected Operand::MatrixMultiplyAccumulateOperands, got {} instead",
                other
            ),
        }
    }
    pub fn unwrap_fp_encoding(&self) -> spirv::FPEncoding {
        match *self {
            Self::FPEncoding(v) => v,
            ref other => panic!("Expected Operand::FPEncoding, got {} instead", other),
        }
    }
    pub fn unwrap_cooperative_vector_matrix_layout(&self) -> spirv::CooperativeVectorMatrixLayout {
        match *self {
            Self::CooperativeVectorMatrixLayout(v) => v,
            ref other => panic!("Expected Operand::CooperativeVectorMatrixLayout, got {} instead", other),
        }
    }
    pub fn unwrap_component_type(&self) -> spirv::ComponentType {
        match *self {
            Self::ComponentType(v) => v,
            ref other => panic!("Expected Operand::ComponentType, got {} instead", other),
        }
    }
    pub fn unwrap_tensor_operands(&self) -> spirv::TensorOperands {
        match *self {
            Self::TensorOperands(v) => v,
            ref other => panic!("Expected Operand::TensorOperands, got {} instead", other),
        }
    }
    pub fn unwrap_id_memory_semantics(&self) -> spirv::Word {
        match *self {
            Self::IdMemorySemantics(v) => v,
            ref other => panic!("Expected Operand::IdMemorySemantics, got {} instead", other),
        }
    }
    pub fn unwrap_id_scope(&self) -> spirv::Word {
        match *self {
            Self::IdScope(v) => v,
            ref other => panic!("Expected Operand::IdScope, got {} instead", other),
        }
    }
    pub fn unwrap_id_ref(&self) -> spirv::Word {
        match *self {
            Self::IdRef(v) => v,
            ref other => panic!("Expected Operand::IdRef, got {} instead", other),
        }
    }
    pub fn unwrap_literal_bit32(&self) -> u32 {
        match *self {
            Self::LiteralBit32(v) => v,
            ref other => panic!("Expected Operand::LiteralBit32, got {} instead", other),
        }
    }
    pub fn unwrap_literal_bit64(&self) -> u64 {
        match *self {
            Self::LiteralBit64(v) => v,
            ref other => panic!("Expected Operand::LiteralBit64, got {} instead", other),
        }
    }
    pub fn unwrap_literal_ext_inst_integer(&self) -> u32 {
        match *self {
            Self::LiteralExtInstInteger(v) => v,
            ref other => panic!("Expected Operand::LiteralExtInstInteger, got {} instead", other),
        }
    }
    pub fn unwrap_literal_spec_constant_op_integer(&self) -> spirv::Op {
        match *self {
            Self::LiteralSpecConstantOpInteger(v) => v,
            ref other => panic!("Expected Operand::LiteralSpecConstantOpInteger, got {} instead", other),
        }
    }
    pub fn unwrap_literal_string(&self) -> &str {
        match self {
            Self::LiteralString(v) => v,
            ref other => panic!("Expected Operand::LiteralString, got {} instead", other),
        }
    }
    pub fn id_ref_any(&self) -> Option<spirv::Word> {
        match *self {
            Self::IdRef(v) | Self::IdScope(v) | Self::IdMemorySemantics(v) => Some(v),
            _ => None,
        }
    }
    pub fn id_ref_any_mut(&mut self) -> Option<&mut spirv::Word> {
        match self {
            Self::IdRef(v) | Self::IdScope(v) | Self::IdMemorySemantics(v) => Some(v),
            _ => None,
        }
    }
    pub fn minimum_version(&self) -> Option<(u8, u8)> {
        use spirv as s;
        match self {
            Self::ImageOperands(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::ImageOperands::BIAS
                        | s::ImageOperands::LOD
                        | s::ImageOperands::GRAD
                        | s::ImageOperands::CONST_OFFSET
                        | s::ImageOperands::OFFSET
                        | s::ImageOperands::CONST_OFFSETS
                        | s::ImageOperands::SAMPLE
                        | s::ImageOperands::MIN_LOD
                        | s::ImageOperands::OFFSETS,
                ) {
                    result = result.max(Some((1u8, 0u8))?);
                };
                if v.intersects(s::ImageOperands::SIGN_EXTEND | s::ImageOperands::ZERO_EXTEND) {
                    result = result.max(Some((1u8, 4u8))?);
                };
                if v.intersects(
                    s::ImageOperands::MAKE_TEXEL_AVAILABLE
                        | s::ImageOperands::MAKE_TEXEL_VISIBLE
                        | s::ImageOperands::NON_PRIVATE_TEXEL
                        | s::ImageOperands::VOLATILE_TEXEL,
                ) {
                    result = result.max(Some((1u8, 5u8))?);
                };
                if v.intersects(s::ImageOperands::NONTEMPORAL) {
                    result = result.max(Some((1u8, 6u8))?);
                };
                Some(result)
            }
            Self::FPFastMathMode(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::FPFastMathMode::NONE
                        | s::FPFastMathMode::NOT_NAN
                        | s::FPFastMathMode::NOT_INF
                        | s::FPFastMathMode::NSZ
                        | s::FPFastMathMode::ALLOW_RECIP
                        | s::FPFastMathMode::FAST,
                ) {
                    result = result.max(Some((1u8, 0u8))?);
                };
                if v.intersects(
                    s::FPFastMathMode::ALLOW_CONTRACT
                        | s::FPFastMathMode::ALLOW_REASSOC
                        | s::FPFastMathMode::ALLOW_TRANSFORM,
                ) {
                    result = result.max(None?);
                };
                Some(result)
            }
            Self::SelectionControl(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::SelectionControl::NONE | s::SelectionControl::FLATTEN | s::SelectionControl::DONT_FLATTEN,
                ) {
                    result = result.max(Some((1u8, 0u8))?);
                };
                Some(result)
            }
            Self::LoopControl(v) => {
                let mut result = (1, 0);
                if v.intersects(s::LoopControl::NONE | s::LoopControl::UNROLL | s::LoopControl::DONT_UNROLL) {
                    result = result.max(Some((1u8, 0u8))?);
                };
                if v.intersects(s::LoopControl::DEPENDENCY_INFINITE | s::LoopControl::DEPENDENCY_LENGTH) {
                    result = result.max(Some((1u8, 1u8))?);
                };
                if v.intersects(
                    s::LoopControl::MIN_ITERATIONS
                        | s::LoopControl::MAX_ITERATIONS
                        | s::LoopControl::ITERATION_MULTIPLE
                        | s::LoopControl::PEEL_COUNT
                        | s::LoopControl::PARTIAL_COUNT,
                ) {
                    result = result.max(Some((1u8, 4u8))?);
                };
                if v.intersects(
                    s::LoopControl::INITIATION_INTERVAL_ALTERA
                        | s::LoopControl::MAX_CONCURRENCY_ALTERA
                        | s::LoopControl::DEPENDENCY_ARRAY_ALTERA
                        | s::LoopControl::PIPELINE_ENABLE_ALTERA
                        | s::LoopControl::LOOP_COALESCE_ALTERA
                        | s::LoopControl::MAX_INTERLEAVING_ALTERA
                        | s::LoopControl::SPECULATED_ITERATIONS_ALTERA
                        | s::LoopControl::NO_FUSION_ALTERA
                        | s::LoopControl::LOOP_COUNT_ALTERA
                        | s::LoopControl::MAX_REINVOCATION_DELAY_ALTERA,
                ) {
                    result = result.max(None?);
                };
                Some(result)
            }
            Self::FunctionControl(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::FunctionControl::NONE
                        | s::FunctionControl::INLINE
                        | s::FunctionControl::DONT_INLINE
                        | s::FunctionControl::PURE
                        | s::FunctionControl::CONST,
                ) {
                    result = result.max(Some((1u8, 0u8))?);
                };
                if v.intersects(s::FunctionControl::OPT_NONE_EXT) {
                    result = result.max(None?);
                };
                Some(result)
            }
            Self::MemorySemantics(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::MemorySemantics::RELAXED
                        | s::MemorySemantics::ACQUIRE
                        | s::MemorySemantics::RELEASE
                        | s::MemorySemantics::ACQUIRE_RELEASE
                        | s::MemorySemantics::SEQUENTIALLY_CONSISTENT
                        | s::MemorySemantics::UNIFORM_MEMORY
                        | s::MemorySemantics::SUBGROUP_MEMORY
                        | s::MemorySemantics::WORKGROUP_MEMORY
                        | s::MemorySemantics::CROSS_WORKGROUP_MEMORY
                        | s::MemorySemantics::ATOMIC_COUNTER_MEMORY
                        | s::MemorySemantics::IMAGE_MEMORY,
                ) {
                    result = result.max(Some((1u8, 0u8))?);
                };
                if v.intersects(
                    s::MemorySemantics::OUTPUT_MEMORY
                        | s::MemorySemantics::MAKE_AVAILABLE
                        | s::MemorySemantics::MAKE_VISIBLE
                        | s::MemorySemantics::VOLATILE,
                ) {
                    result = result.max(Some((1u8, 5u8))?);
                };
                Some(result)
            }
            Self::MemoryAccess(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::MemoryAccess::NONE
                        | s::MemoryAccess::VOLATILE
                        | s::MemoryAccess::ALIGNED
                        | s::MemoryAccess::NONTEMPORAL,
                ) {
                    result = result.max(Some((1u8, 0u8))?);
                };
                if v.intersects(
                    s::MemoryAccess::MAKE_POINTER_AVAILABLE
                        | s::MemoryAccess::MAKE_POINTER_VISIBLE
                        | s::MemoryAccess::NON_PRIVATE_POINTER,
                ) {
                    result = result.max(Some((1u8, 5u8))?);
                };
                if v.intersects(s::MemoryAccess::ALIAS_SCOPE_INTEL_MASK | s::MemoryAccess::NO_ALIAS_INTEL_MASK) {
                    result = result.max(None?);
                };
                Some(result)
            }
            Self::KernelProfilingInfo(v) => {
                let mut result = (1, 0);
                if v.intersects(s::KernelProfilingInfo::NONE | s::KernelProfilingInfo::CMD_EXEC_TIME) {
                    result = result.max(Some((1u8, 0u8))?);
                };
                Some(result)
            }
            Self::RayFlags(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::RayFlags::NONE_KHR
                        | s::RayFlags::OPAQUE_KHR
                        | s::RayFlags::NO_OPAQUE_KHR
                        | s::RayFlags::TERMINATE_ON_FIRST_HIT_KHR
                        | s::RayFlags::SKIP_CLOSEST_HIT_SHADER_KHR
                        | s::RayFlags::CULL_BACK_FACING_TRIANGLES_KHR
                        | s::RayFlags::CULL_FRONT_FACING_TRIANGLES_KHR
                        | s::RayFlags::CULL_OPAQUE_KHR
                        | s::RayFlags::CULL_NO_OPAQUE_KHR
                        | s::RayFlags::SKIP_TRIANGLES_KHR
                        | s::RayFlags::SKIP_AAB_BS_KHR
                        | s::RayFlags::FORCE_OPACITY_MICROMAP2_STATE_EXT,
                ) {
                    result = result.max(None?);
                };
                Some(result)
            }
            Self::FragmentShadingRate(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::FragmentShadingRate::VERTICAL2_PIXELS
                        | s::FragmentShadingRate::VERTICAL4_PIXELS
                        | s::FragmentShadingRate::HORIZONTAL2_PIXELS
                        | s::FragmentShadingRate::HORIZONTAL4_PIXELS,
                ) {
                    result = result.max(None?);
                };
                Some(result)
            }
            Self::RawAccessChainOperands(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::RawAccessChainOperands::ROBUSTNESS_PER_COMPONENT_NV
                        | s::RawAccessChainOperands::ROBUSTNESS_PER_ELEMENT_NV,
                ) {
                    result = result.max(None?);
                };
                Some(result)
            }
            Self::SourceLanguage(v) => match v {
                s::SourceLanguage::Unknown
                | s::SourceLanguage::ESSL
                | s::SourceLanguage::GLSL
                | s::SourceLanguage::OpenCL_C
                | s::SourceLanguage::OpenCL_CPP
                | s::SourceLanguage::HLSL
                | s::SourceLanguage::CPP_for_OpenCL
                | s::SourceLanguage::SYCL
                | s::SourceLanguage::HERO_C
                | s::SourceLanguage::NZSL
                | s::SourceLanguage::WGSL
                | s::SourceLanguage::Slang
                | s::SourceLanguage::Zig
                | s::SourceLanguage::Rust => Some((1u8, 0u8)),
            },
            Self::ExecutionModel(v) => match v {
                s::ExecutionModel::Vertex
                | s::ExecutionModel::TessellationControl
                | s::ExecutionModel::TessellationEvaluation
                | s::ExecutionModel::Geometry
                | s::ExecutionModel::Fragment
                | s::ExecutionModel::GLCompute
                | s::ExecutionModel::Kernel => Some((1u8, 0u8)),
                s::ExecutionModel::TaskNV
                | s::ExecutionModel::MeshNV
                | s::ExecutionModel::RayGenerationKHR
                | s::ExecutionModel::IntersectionKHR
                | s::ExecutionModel::AnyHitKHR
                | s::ExecutionModel::ClosestHitKHR
                | s::ExecutionModel::MissKHR
                | s::ExecutionModel::CallableKHR
                | s::ExecutionModel::TaskEXT
                | s::ExecutionModel::MeshEXT => None,
            },
            Self::AddressingModel(v) => match v {
                s::AddressingModel::Logical | s::AddressingModel::Physical32 | s::AddressingModel::Physical64 => {
                    Some((1u8, 0u8))
                }
                s::AddressingModel::PhysicalStorageBuffer64 => Some((1u8, 5u8)),
            },
            Self::MemoryModel(v) => match v {
                s::MemoryModel::Simple | s::MemoryModel::GLSL450 | s::MemoryModel::OpenCL => Some((1u8, 0u8)),
                s::MemoryModel::Vulkan => Some((1u8, 5u8)),
            },
            Self::ExecutionMode(v) => match v {
                s::ExecutionMode::Invocations
                | s::ExecutionMode::SpacingEqual
                | s::ExecutionMode::SpacingFractionalEven
                | s::ExecutionMode::SpacingFractionalOdd
                | s::ExecutionMode::VertexOrderCw
                | s::ExecutionMode::VertexOrderCcw
                | s::ExecutionMode::PixelCenterInteger
                | s::ExecutionMode::OriginUpperLeft
                | s::ExecutionMode::OriginLowerLeft
                | s::ExecutionMode::EarlyFragmentTests
                | s::ExecutionMode::PointMode
                | s::ExecutionMode::Xfb
                | s::ExecutionMode::DepthReplacing
                | s::ExecutionMode::DepthGreater
                | s::ExecutionMode::DepthLess
                | s::ExecutionMode::DepthUnchanged
                | s::ExecutionMode::LocalSize
                | s::ExecutionMode::LocalSizeHint
                | s::ExecutionMode::InputPoints
                | s::ExecutionMode::InputLines
                | s::ExecutionMode::InputLinesAdjacency
                | s::ExecutionMode::Triangles
                | s::ExecutionMode::InputTrianglesAdjacency
                | s::ExecutionMode::Quads
                | s::ExecutionMode::Isolines
                | s::ExecutionMode::OutputVertices
                | s::ExecutionMode::OutputPoints
                | s::ExecutionMode::OutputLineStrip
                | s::ExecutionMode::OutputTriangleStrip
                | s::ExecutionMode::VecTypeHint
                | s::ExecutionMode::ContractionOff => Some((1u8, 0u8)),
                s::ExecutionMode::Initializer
                | s::ExecutionMode::Finalizer
                | s::ExecutionMode::SubgroupSize
                | s::ExecutionMode::SubgroupsPerWorkgroup => Some((1u8, 1u8)),
                s::ExecutionMode::SubgroupsPerWorkgroupId
                | s::ExecutionMode::LocalSizeId
                | s::ExecutionMode::LocalSizeHintId => Some((1u8, 2u8)),
                s::ExecutionMode::DenormPreserve
                | s::ExecutionMode::DenormFlushToZero
                | s::ExecutionMode::SignedZeroInfNanPreserve
                | s::ExecutionMode::RoundingModeRTE
                | s::ExecutionMode::RoundingModeRTZ => Some((1u8, 4u8)),
                s::ExecutionMode::NonCoherentColorAttachmentReadEXT
                | s::ExecutionMode::NonCoherentDepthAttachmentReadEXT
                | s::ExecutionMode::NonCoherentStencilAttachmentReadEXT
                | s::ExecutionMode::SubgroupUniformControlFlowKHR
                | s::ExecutionMode::PostDepthCoverage
                | s::ExecutionMode::NonCoherentTileAttachmentReadQCOM
                | s::ExecutionMode::TileShadingRateQCOM
                | s::ExecutionMode::EarlyAndLateFragmentTestsAMD
                | s::ExecutionMode::StencilRefReplacingEXT
                | s::ExecutionMode::CoalescingAMDX
                | s::ExecutionMode::IsApiEntryAMDX
                | s::ExecutionMode::MaxNodeRecursionAMDX
                | s::ExecutionMode::StaticNumWorkgroupsAMDX
                | s::ExecutionMode::ShaderIndexAMDX
                | s::ExecutionMode::MaxNumWorkgroupsAMDX
                | s::ExecutionMode::StencilRefUnchangedFrontAMD
                | s::ExecutionMode::StencilRefGreaterFrontAMD
                | s::ExecutionMode::StencilRefLessFrontAMD
                | s::ExecutionMode::StencilRefUnchangedBackAMD
                | s::ExecutionMode::StencilRefGreaterBackAMD
                | s::ExecutionMode::StencilRefLessBackAMD
                | s::ExecutionMode::QuadDerivativesKHR
                | s::ExecutionMode::RequireFullQuadsKHR
                | s::ExecutionMode::SharesInputWithAMDX
                | s::ExecutionMode::ArithmeticPoisonKHR
                | s::ExecutionMode::OutputLinesEXT
                | s::ExecutionMode::OutputPrimitivesEXT
                | s::ExecutionMode::DerivativeGroupQuadsKHR
                | s::ExecutionMode::DerivativeGroupLinearKHR
                | s::ExecutionMode::OutputTrianglesEXT
                | s::ExecutionMode::PixelInterlockOrderedEXT
                | s::ExecutionMode::PixelInterlockUnorderedEXT
                | s::ExecutionMode::SampleInterlockOrderedEXT
                | s::ExecutionMode::SampleInterlockUnorderedEXT
                | s::ExecutionMode::ShadingRateInterlockOrderedEXT
                | s::ExecutionMode::ShadingRateInterlockUnorderedEXT
                | s::ExecutionMode::Shader64BitIndexingEXT
                | s::ExecutionMode::SharedLocalMemorySizeINTEL
                | s::ExecutionMode::RoundingModeRTPINTEL
                | s::ExecutionMode::RoundingModeRTNINTEL
                | s::ExecutionMode::FloatingPointModeALTINTEL
                | s::ExecutionMode::FloatingPointModeIEEEINTEL
                | s::ExecutionMode::MaxWorkgroupSizeINTEL
                | s::ExecutionMode::MaxWorkDimINTEL
                | s::ExecutionMode::NoGlobalOffsetINTEL
                | s::ExecutionMode::NumSIMDWorkitemsINTEL
                | s::ExecutionMode::SchedulerTargetFmaxMhzINTEL
                | s::ExecutionMode::MaximallyReconvergesKHR
                | s::ExecutionMode::FPFastMathDefault
                | s::ExecutionMode::StreamingInterfaceINTEL
                | s::ExecutionMode::RegisterMapInterfaceINTEL
                | s::ExecutionMode::NamedBarrierCountINTEL
                | s::ExecutionMode::MaximumRegistersINTEL
                | s::ExecutionMode::MaximumRegistersIdINTEL
                | s::ExecutionMode::NamedMaximumRegistersINTEL => None,
            },
            Self::StorageClass(v) => match v {
                s::StorageClass::UniformConstant
                | s::StorageClass::Input
                | s::StorageClass::Uniform
                | s::StorageClass::Output
                | s::StorageClass::Workgroup
                | s::StorageClass::CrossWorkgroup
                | s::StorageClass::Private
                | s::StorageClass::Function
                | s::StorageClass::Generic
                | s::StorageClass::PushConstant
                | s::StorageClass::AtomicCounter
                | s::StorageClass::Image => Some((1u8, 0u8)),
                s::StorageClass::StorageBuffer => Some((1u8, 3u8)),
                s::StorageClass::TaskPayloadWorkgroupEXT => Some((1u8, 4u8)),
                s::StorageClass::PhysicalStorageBuffer => Some((1u8, 5u8)),
                s::StorageClass::TileImageEXT
                | s::StorageClass::TileAttachmentQCOM
                | s::StorageClass::NodePayloadAMDX
                | s::StorageClass::CallableDataKHR
                | s::StorageClass::IncomingCallableDataKHR
                | s::StorageClass::RayPayloadKHR
                | s::StorageClass::HitAttributeKHR
                | s::StorageClass::IncomingRayPayloadKHR
                | s::StorageClass::ShaderRecordBufferKHR
                | s::StorageClass::HitObjectAttributeNV
                | s::StorageClass::HitObjectAttributeEXT
                | s::StorageClass::CodeSectionINTEL
                | s::StorageClass::DeviceOnlyALTERA
                | s::StorageClass::HostOnlyALTERA => None,
            },
            Self::Dim(v) => match v {
                s::Dim::Dim1D
                | s::Dim::Dim2D
                | s::Dim::Dim3D
                | s::Dim::DimCube
                | s::Dim::DimRect
                | s::Dim::DimBuffer
                | s::Dim::DimSubpassData => Some((1u8, 0u8)),
                s::Dim::DimTileImageDataEXT => None,
            },
            Self::SamplerAddressingMode(v) => match v {
                s::SamplerAddressingMode::None
                | s::SamplerAddressingMode::ClampToEdge
                | s::SamplerAddressingMode::Clamp
                | s::SamplerAddressingMode::Repeat
                | s::SamplerAddressingMode::RepeatMirrored => Some((1u8, 0u8)),
            },
            Self::SamplerFilterMode(v) => match v {
                s::SamplerFilterMode::Nearest | s::SamplerFilterMode::Linear => Some((1u8, 0u8)),
            },
            Self::ImageFormat(v) => match v {
                s::ImageFormat::Unknown
                | s::ImageFormat::Rgba32f
                | s::ImageFormat::Rgba16f
                | s::ImageFormat::R32f
                | s::ImageFormat::Rgba8
                | s::ImageFormat::Rgba8Snorm
                | s::ImageFormat::Rg32f
                | s::ImageFormat::Rg16f
                | s::ImageFormat::R11fG11fB10f
                | s::ImageFormat::R16f
                | s::ImageFormat::Rgba16
                | s::ImageFormat::Rgb10A2
                | s::ImageFormat::Rg16
                | s::ImageFormat::Rg8
                | s::ImageFormat::R16
                | s::ImageFormat::R8
                | s::ImageFormat::Rgba16Snorm
                | s::ImageFormat::Rg16Snorm
                | s::ImageFormat::Rg8Snorm
                | s::ImageFormat::R16Snorm
                | s::ImageFormat::R8Snorm
                | s::ImageFormat::Rgba32i
                | s::ImageFormat::Rgba16i
                | s::ImageFormat::Rgba8i
                | s::ImageFormat::R32i
                | s::ImageFormat::Rg32i
                | s::ImageFormat::Rg16i
                | s::ImageFormat::Rg8i
                | s::ImageFormat::R16i
                | s::ImageFormat::R8i
                | s::ImageFormat::Rgba32ui
                | s::ImageFormat::Rgba16ui
                | s::ImageFormat::Rgba8ui
                | s::ImageFormat::R32ui
                | s::ImageFormat::Rgb10a2ui
                | s::ImageFormat::Rg32ui
                | s::ImageFormat::Rg16ui
                | s::ImageFormat::Rg8ui
                | s::ImageFormat::R16ui
                | s::ImageFormat::R8ui
                | s::ImageFormat::R64ui
                | s::ImageFormat::R64i => Some((1u8, 0u8)),
            },
            Self::ImageChannelOrder(v) => match v {
                s::ImageChannelOrder::R
                | s::ImageChannelOrder::A
                | s::ImageChannelOrder::RG
                | s::ImageChannelOrder::RA
                | s::ImageChannelOrder::RGB
                | s::ImageChannelOrder::RGBA
                | s::ImageChannelOrder::BGRA
                | s::ImageChannelOrder::ARGB
                | s::ImageChannelOrder::Intensity
                | s::ImageChannelOrder::Luminance
                | s::ImageChannelOrder::Rx
                | s::ImageChannelOrder::RGx
                | s::ImageChannelOrder::RGBx
                | s::ImageChannelOrder::Depth
                | s::ImageChannelOrder::DepthStencil
                | s::ImageChannelOrder::sRGB
                | s::ImageChannelOrder::sRGBx
                | s::ImageChannelOrder::sRGBA
                | s::ImageChannelOrder::sBGRA
                | s::ImageChannelOrder::ABGR => Some((1u8, 0u8)),
            },
            Self::ImageChannelDataType(v) => match v {
                s::ImageChannelDataType::SnormInt8
                | s::ImageChannelDataType::SnormInt16
                | s::ImageChannelDataType::UnormInt8
                | s::ImageChannelDataType::UnormInt16
                | s::ImageChannelDataType::UnormShort565
                | s::ImageChannelDataType::UnormShort555
                | s::ImageChannelDataType::UnormInt101010
                | s::ImageChannelDataType::SignedInt8
                | s::ImageChannelDataType::SignedInt16
                | s::ImageChannelDataType::SignedInt32
                | s::ImageChannelDataType::UnsignedInt8
                | s::ImageChannelDataType::UnsignedInt16
                | s::ImageChannelDataType::UnsignedInt32
                | s::ImageChannelDataType::HalfFloat
                | s::ImageChannelDataType::Float
                | s::ImageChannelDataType::UnormInt24
                | s::ImageChannelDataType::UnormInt101010_2
                | s::ImageChannelDataType::UnormInt10X6EXT
                | s::ImageChannelDataType::UnsignedIntRaw10EXT
                | s::ImageChannelDataType::UnsignedIntRaw12EXT
                | s::ImageChannelDataType::UnormInt2_101010EXT
                | s::ImageChannelDataType::UnsignedInt10X6EXT
                | s::ImageChannelDataType::UnsignedInt12X4EXT
                | s::ImageChannelDataType::UnsignedInt14X2EXT
                | s::ImageChannelDataType::UnormInt12X4EXT
                | s::ImageChannelDataType::UnormInt14X2EXT => Some((1u8, 0u8)),
            },
            Self::FPRoundingMode(v) => match v {
                s::FPRoundingMode::RTE | s::FPRoundingMode::RTZ | s::FPRoundingMode::RTP | s::FPRoundingMode::RTN => {
                    Some((1u8, 0u8))
                }
            },
            Self::FPDenormMode(v) => match v {
                s::FPDenormMode::Preserve | s::FPDenormMode::FlushToZero => None,
            },
            Self::QuantizationModes(v) => match v {
                s::QuantizationModes::TRN
                | s::QuantizationModes::TRN_ZERO
                | s::QuantizationModes::RND
                | s::QuantizationModes::RND_ZERO
                | s::QuantizationModes::RND_INF
                | s::QuantizationModes::RND_MIN_INF
                | s::QuantizationModes::RND_CONV
                | s::QuantizationModes::RND_CONV_ODD => None,
            },
            Self::FPOperationMode(v) => match v {
                s::FPOperationMode::IEEE | s::FPOperationMode::ALT => None,
            },
            Self::OverflowModes(v) => match v {
                s::OverflowModes::WRAP
                | s::OverflowModes::SAT
                | s::OverflowModes::SAT_ZERO
                | s::OverflowModes::SAT_SYM => None,
            },
            Self::LinkageType(v) => match v {
                s::LinkageType::Export | s::LinkageType::Import => Some((1u8, 0u8)),
                s::LinkageType::LinkOnceODR => None,
            },
            Self::AccessQualifier(v) => match v {
                s::AccessQualifier::ReadOnly | s::AccessQualifier::WriteOnly | s::AccessQualifier::ReadWrite => {
                    Some((1u8, 0u8))
                }
            },
            Self::HostAccessQualifier(v) => match v {
                s::HostAccessQualifier::NoneINTEL
                | s::HostAccessQualifier::ReadINTEL
                | s::HostAccessQualifier::WriteINTEL
                | s::HostAccessQualifier::ReadWriteINTEL => None,
            },
            Self::FunctionParameterAttribute(v) => match v {
                s::FunctionParameterAttribute::Zext
                | s::FunctionParameterAttribute::Sext
                | s::FunctionParameterAttribute::ByVal
                | s::FunctionParameterAttribute::Sret
                | s::FunctionParameterAttribute::NoAlias
                | s::FunctionParameterAttribute::NoCapture
                | s::FunctionParameterAttribute::NoWrite
                | s::FunctionParameterAttribute::NoReadWrite
                | s::FunctionParameterAttribute::RuntimeAlignedALTERA => Some((1u8, 0u8)),
            },
            Self::Decoration(v) => match v {
                s::Decoration::RelaxedPrecision
                | s::Decoration::SpecId
                | s::Decoration::Block
                | s::Decoration::BufferBlock
                | s::Decoration::RowMajor
                | s::Decoration::ColMajor
                | s::Decoration::ArrayStride
                | s::Decoration::MatrixStride
                | s::Decoration::GLSLShared
                | s::Decoration::GLSLPacked
                | s::Decoration::CPacked
                | s::Decoration::BuiltIn
                | s::Decoration::NoPerspective
                | s::Decoration::Flat
                | s::Decoration::Patch
                | s::Decoration::Centroid
                | s::Decoration::Sample
                | s::Decoration::Invariant
                | s::Decoration::Restrict
                | s::Decoration::Aliased
                | s::Decoration::Volatile
                | s::Decoration::Constant
                | s::Decoration::Coherent
                | s::Decoration::NonWritable
                | s::Decoration::NonReadable
                | s::Decoration::Uniform
                | s::Decoration::SaturatedConversion
                | s::Decoration::Stream
                | s::Decoration::Location
                | s::Decoration::Component
                | s::Decoration::Index
                | s::Decoration::Binding
                | s::Decoration::DescriptorSet
                | s::Decoration::Offset
                | s::Decoration::XfbBuffer
                | s::Decoration::XfbStride
                | s::Decoration::FuncParamAttr
                | s::Decoration::FPRoundingMode
                | s::Decoration::FPFastMathMode
                | s::Decoration::LinkageAttributes
                | s::Decoration::NoContraction
                | s::Decoration::InputAttachmentIndex
                | s::Decoration::Alignment => Some((1u8, 0u8)),
                s::Decoration::MaxByteOffset => Some((1u8, 1u8)),
                s::Decoration::AlignmentId | s::Decoration::MaxByteOffsetId => Some((1u8, 2u8)),
                s::Decoration::UniformId
                | s::Decoration::NoSignedWrap
                | s::Decoration::NoUnsignedWrap
                | s::Decoration::CounterBuffer
                | s::Decoration::UserSemantic => Some((1u8, 4u8)),
                s::Decoration::NonUniform | s::Decoration::RestrictPointer | s::Decoration::AliasedPointer => {
                    Some((1u8, 5u8))
                }
                s::Decoration::SaturatedToLargestFloat8NormalConversionEXT
                | s::Decoration::WeightTextureQCOM
                | s::Decoration::BlockMatchTextureQCOM
                | s::Decoration::BlockMatchSamplerQCOM
                | s::Decoration::ExplicitInterpAMD
                | s::Decoration::NodeSharesPayloadLimitsWithAMDX
                | s::Decoration::NodeMaxPayloadsAMDX
                | s::Decoration::TrackFinishWritingAMDX
                | s::Decoration::PayloadNodeNameAMDX
                | s::Decoration::PayloadNodeBaseIndexAMDX
                | s::Decoration::PayloadNodeSparseArrayAMDX
                | s::Decoration::PayloadNodeArraySizeAMDX
                | s::Decoration::PayloadDispatchIndirectAMDX
                | s::Decoration::ArrayStrideIdEXT
                | s::Decoration::OffsetIdEXT
                | s::Decoration::UTFEncodedKHR
                | s::Decoration::OverrideCoverageNV
                | s::Decoration::PassthroughNV
                | s::Decoration::ViewportRelativeNV
                | s::Decoration::SecondaryViewportRelativeNV
                | s::Decoration::PerPrimitiveEXT
                | s::Decoration::PerViewNV
                | s::Decoration::PerTaskNV
                | s::Decoration::PerVertexKHR
                | s::Decoration::MemberOffsetNV
                | s::Decoration::HitObjectShaderRecordBufferNV
                | s::Decoration::HitObjectShaderRecordBufferEXT
                | s::Decoration::BankNV
                | s::Decoration::BindlessSamplerNV
                | s::Decoration::BindlessImageNV
                | s::Decoration::BoundSamplerNV
                | s::Decoration::BoundImageNV
                | s::Decoration::SIMTCallINTEL
                | s::Decoration::ReferencedIndirectlyINTEL
                | s::Decoration::ClobberINTEL
                | s::Decoration::SideEffectsINTEL
                | s::Decoration::VectorComputeVariableINTEL
                | s::Decoration::FuncParamIOKindINTEL
                | s::Decoration::VectorComputeFunctionINTEL
                | s::Decoration::StackCallINTEL
                | s::Decoration::GlobalVariableOffsetINTEL
                | s::Decoration::UserTypeGOOGLE
                | s::Decoration::FunctionRoundingModeINTEL
                | s::Decoration::FunctionDenormModeINTEL
                | s::Decoration::RegisterALTERA
                | s::Decoration::MemoryALTERA
                | s::Decoration::NumbanksALTERA
                | s::Decoration::BankwidthALTERA
                | s::Decoration::MaxPrivateCopiesALTERA
                | s::Decoration::SinglepumpALTERA
                | s::Decoration::DoublepumpALTERA
                | s::Decoration::MaxReplicatesALTERA
                | s::Decoration::SimpleDualPortALTERA
                | s::Decoration::MergeALTERA
                | s::Decoration::BankBitsALTERA
                | s::Decoration::ForcePow2DepthALTERA
                | s::Decoration::StridesizeALTERA
                | s::Decoration::WordsizeALTERA
                | s::Decoration::TrueDualPortALTERA
                | s::Decoration::BurstCoalesceALTERA
                | s::Decoration::CacheSizeALTERA
                | s::Decoration::DontStaticallyCoalesceALTERA
                | s::Decoration::PrefetchALTERA
                | s::Decoration::StallEnableALTERA
                | s::Decoration::FuseLoopsInFunctionALTERA
                | s::Decoration::MathOpDSPModeALTERA
                | s::Decoration::AliasScopeINTEL
                | s::Decoration::NoAliasINTEL
                | s::Decoration::InitiationIntervalALTERA
                | s::Decoration::MaxConcurrencyALTERA
                | s::Decoration::PipelineEnableALTERA
                | s::Decoration::BufferLocationALTERA
                | s::Decoration::IOPipeStorageALTERA
                | s::Decoration::FunctionFloatingPointModeINTEL
                | s::Decoration::SingleElementVectorINTEL
                | s::Decoration::VectorComputeCallableFunctionINTEL
                | s::Decoration::MediaBlockIOINTEL
                | s::Decoration::StallFreeALTERA
                | s::Decoration::FPMaxErrorDecorationINTEL
                | s::Decoration::LatencyControlLabelALTERA
                | s::Decoration::LatencyControlConstraintALTERA
                | s::Decoration::ConduitKernelArgumentALTERA
                | s::Decoration::RegisterMapKernelArgumentALTERA
                | s::Decoration::MMHostInterfaceAddressWidthALTERA
                | s::Decoration::MMHostInterfaceDataWidthALTERA
                | s::Decoration::MMHostInterfaceLatencyALTERA
                | s::Decoration::MMHostInterfaceReadWriteModeALTERA
                | s::Decoration::MMHostInterfaceMaxBurstALTERA
                | s::Decoration::MMHostInterfaceWaitRequestALTERA
                | s::Decoration::StableKernelArgumentALTERA
                | s::Decoration::HostAccessINTEL
                | s::Decoration::InitModeALTERA
                | s::Decoration::ImplementInRegisterMapALTERA
                | s::Decoration::ConditionalINTEL
                | s::Decoration::CacheControlLoadINTEL
                | s::Decoration::CacheControlStoreINTEL => None,
            },
            Self::BuiltIn(v) => match v {
                s::BuiltIn::Position
                | s::BuiltIn::PointSize
                | s::BuiltIn::ClipDistance
                | s::BuiltIn::CullDistance
                | s::BuiltIn::VertexId
                | s::BuiltIn::InstanceId
                | s::BuiltIn::PrimitiveId
                | s::BuiltIn::InvocationId
                | s::BuiltIn::Layer
                | s::BuiltIn::ViewportIndex
                | s::BuiltIn::TessLevelOuter
                | s::BuiltIn::TessLevelInner
                | s::BuiltIn::TessCoord
                | s::BuiltIn::PatchVertices
                | s::BuiltIn::FragCoord
                | s::BuiltIn::PointCoord
                | s::BuiltIn::FrontFacing
                | s::BuiltIn::SampleId
                | s::BuiltIn::SamplePosition
                | s::BuiltIn::SampleMask
                | s::BuiltIn::FragDepth
                | s::BuiltIn::HelperInvocation
                | s::BuiltIn::NumWorkgroups
                | s::BuiltIn::WorkgroupSize
                | s::BuiltIn::WorkgroupId
                | s::BuiltIn::LocalInvocationId
                | s::BuiltIn::GlobalInvocationId
                | s::BuiltIn::LocalInvocationIndex
                | s::BuiltIn::WorkDim
                | s::BuiltIn::GlobalSize
                | s::BuiltIn::EnqueuedWorkgroupSize
                | s::BuiltIn::GlobalOffset
                | s::BuiltIn::GlobalLinearId
                | s::BuiltIn::SubgroupSize
                | s::BuiltIn::SubgroupMaxSize
                | s::BuiltIn::NumSubgroups
                | s::BuiltIn::NumEnqueuedSubgroups
                | s::BuiltIn::SubgroupId
                | s::BuiltIn::SubgroupLocalInvocationId
                | s::BuiltIn::VertexIndex
                | s::BuiltIn::InstanceIndex
                | s::BuiltIn::CoreIDARM
                | s::BuiltIn::CoreCountARM
                | s::BuiltIn::CoreMaxIDARM
                | s::BuiltIn::WarpIDARM
                | s::BuiltIn::WarpMaxIDARM => Some((1u8, 0u8)),
                s::BuiltIn::SubgroupEqMask
                | s::BuiltIn::SubgroupGeMask
                | s::BuiltIn::SubgroupGtMask
                | s::BuiltIn::SubgroupLeMask
                | s::BuiltIn::SubgroupLtMask
                | s::BuiltIn::BaseVertex
                | s::BuiltIn::BaseInstance
                | s::BuiltIn::DrawIndex
                | s::BuiltIn::DeviceIndex
                | s::BuiltIn::ViewIndex => Some((1u8, 3u8)),
                s::BuiltIn::PrimitiveShadingRateKHR
                | s::BuiltIn::ShadingRateKHR
                | s::BuiltIn::TileOffsetQCOM
                | s::BuiltIn::TileDimensionQCOM
                | s::BuiltIn::TileApronSizeQCOM
                | s::BuiltIn::BaryCoordNoPerspAMD
                | s::BuiltIn::BaryCoordNoPerspCentroidAMD
                | s::BuiltIn::BaryCoordNoPerspSampleAMD
                | s::BuiltIn::BaryCoordSmoothAMD
                | s::BuiltIn::BaryCoordSmoothCentroidAMD
                | s::BuiltIn::BaryCoordSmoothSampleAMD
                | s::BuiltIn::BaryCoordPullModelAMD
                | s::BuiltIn::FragStencilRefEXT
                | s::BuiltIn::RemainingRecursionLevelsAMDX
                | s::BuiltIn::ShaderIndexAMDX
                | s::BuiltIn::SamplerHeapEXT
                | s::BuiltIn::ResourceHeapEXT
                | s::BuiltIn::ViewportMaskNV
                | s::BuiltIn::SecondaryPositionNV
                | s::BuiltIn::SecondaryViewportMaskNV
                | s::BuiltIn::PositionPerViewNV
                | s::BuiltIn::ViewportMaskPerViewNV
                | s::BuiltIn::FullyCoveredEXT
                | s::BuiltIn::TaskCountNV
                | s::BuiltIn::PrimitiveCountNV
                | s::BuiltIn::PrimitiveIndicesNV
                | s::BuiltIn::ClipDistancePerViewNV
                | s::BuiltIn::CullDistancePerViewNV
                | s::BuiltIn::LayerPerViewNV
                | s::BuiltIn::MeshViewCountNV
                | s::BuiltIn::MeshViewIndicesNV
                | s::BuiltIn::BaryCoordKHR
                | s::BuiltIn::BaryCoordNoPerspKHR
                | s::BuiltIn::FragSizeEXT
                | s::BuiltIn::FragInvocationCountEXT
                | s::BuiltIn::PrimitivePointIndicesEXT
                | s::BuiltIn::PrimitiveLineIndicesEXT
                | s::BuiltIn::PrimitiveTriangleIndicesEXT
                | s::BuiltIn::CullPrimitiveEXT
                | s::BuiltIn::LaunchIdKHR
                | s::BuiltIn::LaunchSizeKHR
                | s::BuiltIn::WorldRayOriginKHR
                | s::BuiltIn::WorldRayDirectionKHR
                | s::BuiltIn::ObjectRayOriginKHR
                | s::BuiltIn::ObjectRayDirectionKHR
                | s::BuiltIn::RayTminKHR
                | s::BuiltIn::RayTmaxKHR
                | s::BuiltIn::InstanceCustomIndexKHR
                | s::BuiltIn::ObjectToWorldKHR
                | s::BuiltIn::WorldToObjectKHR
                | s::BuiltIn::HitTNV
                | s::BuiltIn::HitKindKHR
                | s::BuiltIn::CurrentRayTimeNV
                | s::BuiltIn::HitTriangleVertexPositionsKHR
                | s::BuiltIn::HitMicroTriangleVertexPositionsNV
                | s::BuiltIn::HitMicroTriangleVertexBarycentricsNV
                | s::BuiltIn::IncomingRayFlagsKHR
                | s::BuiltIn::RayGeometryIndexKHR
                | s::BuiltIn::HitIsSphereNV
                | s::BuiltIn::HitIsLSSNV
                | s::BuiltIn::HitSpherePositionNV
                | s::BuiltIn::WarpsPerSMNV
                | s::BuiltIn::SMCountNV
                | s::BuiltIn::WarpIDNV
                | s::BuiltIn::SMIDNV
                | s::BuiltIn::HitLSSPositionsNV
                | s::BuiltIn::HitKindFrontFacingMicroTriangleNV
                | s::BuiltIn::HitKindBackFacingMicroTriangleNV
                | s::BuiltIn::HitSphereRadiusNV
                | s::BuiltIn::HitLSSRadiiNV
                | s::BuiltIn::ClusterIDNV
                | s::BuiltIn::CullMaskKHR => None,
            },
            Self::Scope(v) => match v {
                s::Scope::CrossDevice
                | s::Scope::Device
                | s::Scope::Workgroup
                | s::Scope::Subgroup
                | s::Scope::Invocation => Some((1u8, 0u8)),
                s::Scope::QueueFamily => Some((1u8, 5u8)),
                s::Scope::ShaderCallKHR => None,
            },
            Self::GroupOperation(v) => match v {
                s::GroupOperation::Reduce | s::GroupOperation::InclusiveScan | s::GroupOperation::ExclusiveScan => {
                    Some((1u8, 0u8))
                }
                s::GroupOperation::ClusteredReduce => Some((1u8, 3u8)),
                s::GroupOperation::PartitionedReduceEXT
                | s::GroupOperation::PartitionedInclusiveScanEXT
                | s::GroupOperation::PartitionedExclusiveScanEXT => None,
            },
            Self::KernelEnqueueFlags(v) => match v {
                s::KernelEnqueueFlags::NoWait
                | s::KernelEnqueueFlags::WaitKernel
                | s::KernelEnqueueFlags::WaitWorkGroup => Some((1u8, 0u8)),
            },
            Self::Capability(v) => match v {
                s::Capability::Matrix
                | s::Capability::Shader
                | s::Capability::Geometry
                | s::Capability::Tessellation
                | s::Capability::Addresses
                | s::Capability::Linkage
                | s::Capability::Kernel
                | s::Capability::Vector16
                | s::Capability::Float16Buffer
                | s::Capability::Float16
                | s::Capability::Float64
                | s::Capability::Int64
                | s::Capability::Int64Atomics
                | s::Capability::ImageBasic
                | s::Capability::ImageReadWrite
                | s::Capability::ImageMipmap
                | s::Capability::Pipes
                | s::Capability::Groups
                | s::Capability::DeviceEnqueue
                | s::Capability::LiteralSampler
                | s::Capability::AtomicStorage
                | s::Capability::Int16
                | s::Capability::TessellationPointSize
                | s::Capability::GeometryPointSize
                | s::Capability::ImageGatherExtended
                | s::Capability::StorageImageMultisample
                | s::Capability::UniformBufferArrayDynamicIndexing
                | s::Capability::SampledImageArrayDynamicIndexing
                | s::Capability::StorageBufferArrayDynamicIndexing
                | s::Capability::StorageImageArrayDynamicIndexing
                | s::Capability::ClipDistance
                | s::Capability::CullDistance
                | s::Capability::ImageCubeArray
                | s::Capability::SampleRateShading
                | s::Capability::ImageRect
                | s::Capability::SampledRect
                | s::Capability::GenericPointer
                | s::Capability::Int8
                | s::Capability::InputAttachment
                | s::Capability::SparseResidency
                | s::Capability::MinLod
                | s::Capability::Sampled1D
                | s::Capability::Image1D
                | s::Capability::SampledCubeArray
                | s::Capability::SampledBuffer
                | s::Capability::ImageBuffer
                | s::Capability::ImageMSArray
                | s::Capability::StorageImageExtendedFormats
                | s::Capability::ImageQuery
                | s::Capability::DerivativeControl
                | s::Capability::InterpolationFunction
                | s::Capability::TransformFeedback
                | s::Capability::GeometryStreams
                | s::Capability::StorageImageReadWithoutFormat
                | s::Capability::StorageImageWriteWithoutFormat
                | s::Capability::MultiViewport => Some((1u8, 0u8)),
                s::Capability::SubgroupDispatch | s::Capability::NamedBarrier | s::Capability::PipeStorage => {
                    Some((1u8, 1u8))
                }
                s::Capability::GroupNonUniform
                | s::Capability::GroupNonUniformVote
                | s::Capability::GroupNonUniformArithmetic
                | s::Capability::GroupNonUniformBallot
                | s::Capability::GroupNonUniformShuffle
                | s::Capability::GroupNonUniformShuffleRelative
                | s::Capability::GroupNonUniformClustered
                | s::Capability::GroupNonUniformQuad
                | s::Capability::DrawParameters
                | s::Capability::StorageBuffer16BitAccess
                | s::Capability::UniformAndStorageBuffer16BitAccess
                | s::Capability::StoragePushConstant16
                | s::Capability::StorageInputOutput16
                | s::Capability::DeviceGroup
                | s::Capability::MultiView
                | s::Capability::VariablePointersStorageBuffer
                | s::Capability::VariablePointers => Some((1u8, 3u8)),
                s::Capability::DenormPreserve
                | s::Capability::DenormFlushToZero
                | s::Capability::SignedZeroInfNanPreserve
                | s::Capability::RoundingModeRTE
                | s::Capability::RoundingModeRTZ => Some((1u8, 4u8)),
                s::Capability::ShaderLayer
                | s::Capability::ShaderViewportIndex
                | s::Capability::StorageBuffer8BitAccess
                | s::Capability::UniformAndStorageBuffer8BitAccess
                | s::Capability::StoragePushConstant8
                | s::Capability::ShaderNonUniform
                | s::Capability::RuntimeDescriptorArray
                | s::Capability::InputAttachmentArrayDynamicIndexing
                | s::Capability::UniformTexelBufferArrayDynamicIndexing
                | s::Capability::StorageTexelBufferArrayDynamicIndexing
                | s::Capability::UniformBufferArrayNonUniformIndexing
                | s::Capability::SampledImageArrayNonUniformIndexing
                | s::Capability::StorageBufferArrayNonUniformIndexing
                | s::Capability::StorageImageArrayNonUniformIndexing
                | s::Capability::InputAttachmentArrayNonUniformIndexing
                | s::Capability::UniformTexelBufferArrayNonUniformIndexing
                | s::Capability::StorageTexelBufferArrayNonUniformIndexing
                | s::Capability::VulkanMemoryModel
                | s::Capability::VulkanMemoryModelDeviceScope
                | s::Capability::PhysicalStorageBufferAddresses => Some((1u8, 5u8)),
                s::Capability::UniformDecoration
                | s::Capability::DemoteToHelperInvocation
                | s::Capability::DotProductInputAll
                | s::Capability::DotProductInput4x8Bit
                | s::Capability::DotProductInput4x8BitPacked
                | s::Capability::DotProduct => Some((1u8, 6u8)),
                s::Capability::CoreBuiltinsARM
                | s::Capability::TileImageColorReadAccessEXT
                | s::Capability::TileImageDepthReadAccessEXT
                | s::Capability::TileImageStencilReadAccessEXT
                | s::Capability::TensorsARM
                | s::Capability::StorageTensorArrayDynamicIndexingARM
                | s::Capability::StorageTensorArrayNonUniformIndexingARM
                | s::Capability::GraphARM
                | s::Capability::CooperativeMatrixLayoutsARM
                | s::Capability::Float8EXT
                | s::Capability::Float8CooperativeMatrixEXT
                | s::Capability::FragmentShadingRateKHR
                | s::Capability::SubgroupBallotKHR
                | s::Capability::WorkgroupMemoryExplicitLayoutKHR
                | s::Capability::WorkgroupMemoryExplicitLayout8BitAccessKHR
                | s::Capability::WorkgroupMemoryExplicitLayout16BitAccessKHR
                | s::Capability::SubgroupVoteKHR
                | s::Capability::AtomicStorageOps
                | s::Capability::SampleMaskPostDepthCoverage
                | s::Capability::RayQueryProvisionalKHR
                | s::Capability::RayQueryKHR
                | s::Capability::UntypedPointersKHR
                | s::Capability::RayTraversalPrimitiveCullingKHR
                | s::Capability::RayTracingKHR
                | s::Capability::TextureSampleWeightedQCOM
                | s::Capability::TextureBoxFilterQCOM
                | s::Capability::TextureBlockMatchQCOM
                | s::Capability::TileShadingQCOM
                | s::Capability::CooperativeMatrixConversionQCOM
                | s::Capability::TextureBlockMatch2QCOM
                | s::Capability::Float16ImageAMD
                | s::Capability::ImageGatherBiasLodAMD
                | s::Capability::FragmentMaskAMD
                | s::Capability::StencilExportEXT
                | s::Capability::ImageReadWriteLodAMD
                | s::Capability::Int64ImageEXT
                | s::Capability::ShaderClockKHR
                | s::Capability::ShaderEnqueueAMDX
                | s::Capability::QuadControlKHR
                | s::Capability::Int4TypeINTEL
                | s::Capability::Int4CooperativeMatrixINTEL
                | s::Capability::BFloat16TypeKHR
                | s::Capability::BFloat16DotProductKHR
                | s::Capability::BFloat16CooperativeMatrixKHR
                | s::Capability::AbortKHR
                | s::Capability::DescriptorHeapEXT
                | s::Capability::ConstantDataKHR
                | s::Capability::PoisonFreezeKHR
                | s::Capability::SampleMaskOverrideCoverageNV
                | s::Capability::GeometryShaderPassthroughNV
                | s::Capability::ShaderViewportIndexLayerEXT
                | s::Capability::ShaderViewportMaskNV
                | s::Capability::ShaderStereoViewNV
                | s::Capability::PerViewAttributesNV
                | s::Capability::FragmentFullyCoveredEXT
                | s::Capability::MeshShadingNV
                | s::Capability::ImageFootprintNV
                | s::Capability::MeshShadingEXT
                | s::Capability::FragmentBarycentricKHR
                | s::Capability::ComputeDerivativeGroupQuadsKHR
                | s::Capability::FragmentDensityEXT
                | s::Capability::GroupNonUniformPartitionedEXT
                | s::Capability::RayTracingPositionFetchKHR
                | s::Capability::RayTracingNV
                | s::Capability::RayTracingMotionBlurNV
                | s::Capability::ComputeDerivativeGroupLinearKHR
                | s::Capability::RayTracingProvisionalKHR
                | s::Capability::CooperativeMatrixNV
                | s::Capability::FragmentShaderSampleInterlockEXT
                | s::Capability::FragmentShaderShadingRateInterlockEXT
                | s::Capability::ShaderSMBuiltinsNV
                | s::Capability::FragmentShaderPixelInterlockEXT
                | s::Capability::DisplacementMicromapNV
                | s::Capability::RayTracingOpacityMicromapEXT
                | s::Capability::ShaderInvocationReorderNV
                | s::Capability::ShaderInvocationReorderEXT
                | s::Capability::BindlessTextureNV
                | s::Capability::RayQueryPositionFetchKHR
                | s::Capability::CooperativeVectorNV
                | s::Capability::AtomicFloat16VectorNV
                | s::Capability::RayTracingDisplacementMicromapNV
                | s::Capability::RawAccessChainsNV
                | s::Capability::RayTracingSpheresGeometryNV
                | s::Capability::RayTracingLinearSweptSpheresGeometryNV
                | s::Capability::PushConstantBanksNV
                | s::Capability::LongVectorEXT
                | s::Capability::Shader64BitIndexingEXT
                | s::Capability::CooperativeMatrixReductionsNV
                | s::Capability::CooperativeMatrixConversionsNV
                | s::Capability::CooperativeMatrixPerElementOperationsNV
                | s::Capability::CooperativeMatrixTensorAddressingNV
                | s::Capability::CooperativeMatrixBlockLoadsNV
                | s::Capability::CooperativeVectorTrainingNV
                | s::Capability::RayTracingClusterAccelerationStructureNV
                | s::Capability::TensorAddressingNV
                | s::Capability::SubgroupShuffleINTEL
                | s::Capability::SubgroupBufferBlockIOINTEL
                | s::Capability::SubgroupImageBlockIOINTEL
                | s::Capability::SubgroupImageMediaBlockIOINTEL
                | s::Capability::RoundToInfinityINTEL
                | s::Capability::FloatingPointModeINTEL
                | s::Capability::IntegerFunctions2INTEL
                | s::Capability::FunctionPointersINTEL
                | s::Capability::IndirectReferencesINTEL
                | s::Capability::AsmINTEL
                | s::Capability::AtomicFloat32MinMaxEXT
                | s::Capability::AtomicFloat64MinMaxEXT
                | s::Capability::AtomicFloat16MinMaxEXT
                | s::Capability::VectorComputeINTEL
                | s::Capability::VectorAnyINTEL
                | s::Capability::ExpectAssumeKHR
                | s::Capability::SubgroupAvcMotionEstimationINTEL
                | s::Capability::SubgroupAvcMotionEstimationIntraINTEL
                | s::Capability::SubgroupAvcMotionEstimationChromaINTEL
                | s::Capability::VariableLengthArrayINTEL
                | s::Capability::FunctionFloatControlINTEL
                | s::Capability::FPGAMemoryAttributesALTERA
                | s::Capability::FPFastMathModeINTEL
                | s::Capability::ArbitraryPrecisionIntegersALTERA
                | s::Capability::ArbitraryPrecisionFloatingPointALTERA
                | s::Capability::UnstructuredLoopControlsINTEL
                | s::Capability::FPGALoopControlsALTERA
                | s::Capability::KernelAttributesINTEL
                | s::Capability::FPGAKernelAttributesINTEL
                | s::Capability::FPGAMemoryAccessesALTERA
                | s::Capability::FPGAClusterAttributesALTERA
                | s::Capability::LoopFuseALTERA
                | s::Capability::FPGADSPControlALTERA
                | s::Capability::MemoryAccessAliasingINTEL
                | s::Capability::FPGAInvocationPipeliningAttributesALTERA
                | s::Capability::FPGABufferLocationALTERA
                | s::Capability::ArbitraryPrecisionFixedPointALTERA
                | s::Capability::USMStorageClassesALTERA
                | s::Capability::RuntimeAlignedAttributeALTERA
                | s::Capability::IOPipesALTERA
                | s::Capability::BlockingPipesALTERA
                | s::Capability::FPGARegALTERA
                | s::Capability::RayCullMaskKHR
                | s::Capability::CooperativeMatrixKHR
                | s::Capability::ReplicatedCompositesEXT
                | s::Capability::BitInstructions
                | s::Capability::GroupNonUniformRotateKHR
                | s::Capability::FloatControls2
                | s::Capability::FMAKHR
                | s::Capability::AtomicFloat32AddEXT
                | s::Capability::AtomicFloat64AddEXT
                | s::Capability::LongCompositesINTEL
                | s::Capability::OptNoneEXT
                | s::Capability::AtomicFloat16AddEXT
                | s::Capability::DebugInfoModuleINTEL
                | s::Capability::BFloat16ConversionINTEL
                | s::Capability::SplitBarrierINTEL
                | s::Capability::ArithmeticFenceEXT
                | s::Capability::FPGAClusterAttributesV2ALTERA
                | s::Capability::FPGAKernelAttributesv2INTEL
                | s::Capability::TaskSequenceALTERA
                | s::Capability::FPMaxErrorINTEL
                | s::Capability::FPGALatencyControlALTERA
                | s::Capability::FPGAArgumentInterfacesALTERA
                | s::Capability::GlobalVariableHostAccessINTEL
                | s::Capability::GlobalVariableFPGADecorationsALTERA
                | s::Capability::SubgroupBufferPrefetchINTEL
                | s::Capability::Subgroup2DBlockIOINTEL
                | s::Capability::Subgroup2DBlockTransformINTEL
                | s::Capability::Subgroup2DBlockTransposeINTEL
                | s::Capability::SubgroupMatrixMultiplyAccumulateINTEL
                | s::Capability::TernaryBitwiseFunctionINTEL
                | s::Capability::UntypedVariableLengthArrayINTEL
                | s::Capability::SpecConditionalINTEL
                | s::Capability::FunctionVariantsINTEL
                | s::Capability::GroupUniformArithmeticKHR
                | s::Capability::TensorFloat32RoundingINTEL
                | s::Capability::MaskedGatherScatterINTEL
                | s::Capability::CacheControlsINTEL
                | s::Capability::RegisterLimitsINTEL
                | s::Capability::BindlessImagesINTEL
                | s::Capability::DotProductFloat16AccFloat32VALVE
                | s::Capability::DotProductFloat16AccFloat16VALVE
                | s::Capability::DotProductBFloat16AccVALVE
                | s::Capability::DotProductFloat8AccFloat32VALVE => None,
            },
            Self::RayQueryIntersection(v) => match v {
                s::RayQueryIntersection::RayQueryCandidateIntersectionKHR
                | s::RayQueryIntersection::RayQueryCommittedIntersectionKHR => None,
            },
            Self::RayQueryCommittedIntersectionType(v) => match v {
                s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionNoneKHR
                | s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionTriangleKHR
                | s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionGeneratedKHR => None,
            },
            Self::RayQueryCandidateIntersectionType(v) => match v {
                s::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionTriangleKHR
                | s::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionAABBKHR => None,
            },
            Self::PackedVectorFormat(v) => match v {
                s::PackedVectorFormat::PackedVectorFormat4x8Bit => Some((1u8, 6u8)),
            },
            Self::CooperativeMatrixOperands(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::CooperativeMatrixOperands::NONE_KHR
                        | s::CooperativeMatrixOperands::MATRIX_A_SIGNED_COMPONENTS_KHR
                        | s::CooperativeMatrixOperands::MATRIX_B_SIGNED_COMPONENTS_KHR
                        | s::CooperativeMatrixOperands::MATRIX_C_SIGNED_COMPONENTS_KHR
                        | s::CooperativeMatrixOperands::MATRIX_RESULT_SIGNED_COMPONENTS_KHR
                        | s::CooperativeMatrixOperands::SATURATING_ACCUMULATION_KHR,
                ) {
                    result = result.max(None?);
                };
                Some(result)
            }
            Self::CooperativeMatrixLayout(v) => match v {
                s::CooperativeMatrixLayout::RowMajorKHR
                | s::CooperativeMatrixLayout::ColumnMajorKHR
                | s::CooperativeMatrixLayout::RowBlockedInterleavedARM
                | s::CooperativeMatrixLayout::ColumnBlockedInterleavedARM => None,
            },
            Self::CooperativeMatrixUse(v) => match v {
                s::CooperativeMatrixUse::MatrixAKHR
                | s::CooperativeMatrixUse::MatrixBKHR
                | s::CooperativeMatrixUse::MatrixAccumulatorKHR => None,
            },
            Self::CooperativeMatrixReduce(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::CooperativeMatrixReduce::ROW
                        | s::CooperativeMatrixReduce::COLUMN
                        | s::CooperativeMatrixReduce::_2X2,
                ) {
                    result = result.max(None?);
                };
                Some(result)
            }
            Self::TensorClampMode(v) => match v {
                s::TensorClampMode::Undefined
                | s::TensorClampMode::Constant
                | s::TensorClampMode::ClampToEdge
                | s::TensorClampMode::Repeat
                | s::TensorClampMode::RepeatMirrored => None,
            },
            Self::TensorAddressingOperands(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::TensorAddressingOperands::NONE
                        | s::TensorAddressingOperands::TENSOR_VIEW
                        | s::TensorAddressingOperands::DECODE_FUNC,
                ) {
                    result = result.max(None?);
                };
                Some(result)
            }
            Self::InitializationModeQualifier(v) => match v {
                s::InitializationModeQualifier::InitOnDeviceReprogramALTERA
                | s::InitializationModeQualifier::InitOnDeviceResetALTERA => None,
            },
            Self::LoadCacheControl(v) => match v {
                s::LoadCacheControl::UncachedINTEL
                | s::LoadCacheControl::CachedINTEL
                | s::LoadCacheControl::StreamingINTEL
                | s::LoadCacheControl::InvalidateAfterReadINTEL
                | s::LoadCacheControl::ConstCachedINTEL => None,
            },
            Self::StoreCacheControl(v) => match v {
                s::StoreCacheControl::UncachedINTEL
                | s::StoreCacheControl::WriteThroughINTEL
                | s::StoreCacheControl::WriteBackINTEL
                | s::StoreCacheControl::StreamingINTEL => None,
            },
            Self::NamedMaximumNumberOfRegisters(v) => match v {
                s::NamedMaximumNumberOfRegisters::AutoINTEL => None,
            },
            Self::MatrixMultiplyAccumulateOperands(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::MatrixMultiplyAccumulateOperands::NONE
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_A_SIGNED_COMPONENTS_INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_B_SIGNED_COMPONENTS_INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_CB_FLOAT16_INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_RESULT_B_FLOAT16_INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_A_PACKED_INT8_INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_B_PACKED_INT8_INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_A_PACKED_INT4_INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_B_PACKED_INT4_INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_ATF32INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_BTF32INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_A_PACKED_FLOAT16_INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_B_PACKED_FLOAT16_INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_A_PACKED_B_FLOAT16_INTEL
                        | s::MatrixMultiplyAccumulateOperands::MATRIX_B_PACKED_B_FLOAT16_INTEL,
                ) {
                    result = result.max(None?);
                };
                Some(result)
            }
            Self::FPEncoding(v) => match v {
                s::FPEncoding::BFloat16KHR | s::FPEncoding::Float8E4M3EXT | s::FPEncoding::Float8E5M2EXT => None,
            },
            Self::CooperativeVectorMatrixLayout(v) => match v {
                s::CooperativeVectorMatrixLayout::RowMajorNV
                | s::CooperativeVectorMatrixLayout::ColumnMajorNV
                | s::CooperativeVectorMatrixLayout::InferencingOptimalNV
                | s::CooperativeVectorMatrixLayout::TrainingOptimalNV => None,
            },
            Self::ComponentType(v) => match v {
                s::ComponentType::Float16NV
                | s::ComponentType::Float32NV
                | s::ComponentType::Float64NV
                | s::ComponentType::SignedInt8NV
                | s::ComponentType::SignedInt16NV
                | s::ComponentType::SignedInt32NV
                | s::ComponentType::SignedInt64NV
                | s::ComponentType::UnsignedInt8NV
                | s::ComponentType::UnsignedInt16NV
                | s::ComponentType::UnsignedInt32NV
                | s::ComponentType::UnsignedInt64NV
                | s::ComponentType::SignedInt8PackedNV
                | s::ComponentType::UnsignedInt8PackedNV
                | s::ComponentType::FloatE4M3NV
                | s::ComponentType::FloatE5M2NV => None,
            },
            Self::TensorOperands(v) => {
                let mut result = (1, 0);
                if v.intersects(
                    s::TensorOperands::NONE_ARM
                        | s::TensorOperands::NONTEMPORAL_ARM
                        | s::TensorOperands::OUT_OF_BOUNDS_VALUE_ARM
                        | s::TensorOperands::MAKE_ELEMENT_AVAILABLE_ARM
                        | s::TensorOperands::MAKE_ELEMENT_VISIBLE_ARM
                        | s::TensorOperands::NON_PRIVATE_ELEMENT_ARM,
                ) {
                    result = result.max(None?);
                };
                Some(result)
            }
            _ => Some((1, 0)),
        }
    }
    pub fn required_capabilities(&self) -> Vec<Vec<spirv::Capability>> {
        use spirv as s;
        match self {
            Self::ImageOperands(v) => {
                let mut result = vec![];
                if v.intersects(s::ImageOperands::OFFSET | s::ImageOperands::CONST_OFFSETS) {
                    result.push(vec![spirv::Capability::ImageGatherExtended])
                };
                if v.intersects(s::ImageOperands::MIN_LOD) {
                    result.push(vec![spirv::Capability::MinLod])
                };
                if v.intersects(s::ImageOperands::BIAS) {
                    result.push(vec![spirv::Capability::Shader])
                };
                if v.intersects(
                    s::ImageOperands::MAKE_TEXEL_AVAILABLE
                        | s::ImageOperands::MAKE_TEXEL_VISIBLE
                        | s::ImageOperands::NON_PRIVATE_TEXEL
                        | s::ImageOperands::VOLATILE_TEXEL,
                ) {
                    result.push(vec![spirv::Capability::VulkanMemoryModel])
                };
                result
            }
            Self::FPFastMathMode(v) => {
                let mut result = vec![];
                if v.intersects(s::FPFastMathMode::ALLOW_TRANSFORM) {
                    result.push(vec![spirv::Capability::FloatControls2])
                };
                if v.intersects(s::FPFastMathMode::ALLOW_CONTRACT | s::FPFastMathMode::ALLOW_REASSOC) {
                    result.push(vec![
                        spirv::Capability::FloatControls2,
                        spirv::Capability::FPFastMathModeINTEL,
                    ])
                };
                result
            }
            Self::LoopControl(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::LoopControl::INITIATION_INTERVAL_ALTERA
                        | s::LoopControl::MAX_CONCURRENCY_ALTERA
                        | s::LoopControl::DEPENDENCY_ARRAY_ALTERA
                        | s::LoopControl::PIPELINE_ENABLE_ALTERA
                        | s::LoopControl::LOOP_COALESCE_ALTERA
                        | s::LoopControl::MAX_INTERLEAVING_ALTERA
                        | s::LoopControl::SPECULATED_ITERATIONS_ALTERA
                        | s::LoopControl::NO_FUSION_ALTERA
                        | s::LoopControl::LOOP_COUNT_ALTERA
                        | s::LoopControl::MAX_REINVOCATION_DELAY_ALTERA,
                ) {
                    result.push(vec![spirv::Capability::FPGALoopControlsALTERA])
                };
                result
            }
            Self::FunctionControl(v) => {
                let mut result = vec![];
                if v.intersects(s::FunctionControl::OPT_NONE_EXT) {
                    result.push(vec![spirv::Capability::OptNoneEXT])
                };
                result
            }
            Self::MemorySemantics(v) => {
                let mut result = vec![];
                if v.intersects(s::MemorySemantics::ATOMIC_COUNTER_MEMORY) {
                    result.push(vec![spirv::Capability::AtomicStorage])
                };
                if v.intersects(s::MemorySemantics::UNIFORM_MEMORY) {
                    result.push(vec![spirv::Capability::Shader])
                };
                if v.intersects(
                    s::MemorySemantics::OUTPUT_MEMORY
                        | s::MemorySemantics::MAKE_AVAILABLE
                        | s::MemorySemantics::MAKE_VISIBLE
                        | s::MemorySemantics::VOLATILE,
                ) {
                    result.push(vec![spirv::Capability::VulkanMemoryModel])
                };
                result
            }
            Self::MemoryAccess(v) => {
                let mut result = vec![];
                if v.intersects(s::MemoryAccess::ALIAS_SCOPE_INTEL_MASK | s::MemoryAccess::NO_ALIAS_INTEL_MASK) {
                    result.push(vec![spirv::Capability::MemoryAccessAliasingINTEL])
                };
                if v.intersects(
                    s::MemoryAccess::MAKE_POINTER_AVAILABLE
                        | s::MemoryAccess::MAKE_POINTER_VISIBLE
                        | s::MemoryAccess::NON_PRIVATE_POINTER,
                ) {
                    result.push(vec![spirv::Capability::VulkanMemoryModel])
                };
                result
            }
            Self::KernelProfilingInfo(v) => {
                let mut result = vec![];
                if v.intersects(s::KernelProfilingInfo::CMD_EXEC_TIME) {
                    result.push(vec![spirv::Capability::Kernel])
                };
                result
            }
            Self::RayFlags(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::RayFlags::NONE_KHR
                        | s::RayFlags::OPAQUE_KHR
                        | s::RayFlags::NO_OPAQUE_KHR
                        | s::RayFlags::TERMINATE_ON_FIRST_HIT_KHR
                        | s::RayFlags::SKIP_CLOSEST_HIT_SHADER_KHR
                        | s::RayFlags::CULL_BACK_FACING_TRIANGLES_KHR
                        | s::RayFlags::CULL_FRONT_FACING_TRIANGLES_KHR
                        | s::RayFlags::CULL_OPAQUE_KHR
                        | s::RayFlags::CULL_NO_OPAQUE_KHR,
                ) {
                    result.push(vec![spirv::Capability::RayQueryKHR, spirv::Capability::RayTracingKHR])
                };
                if v.intersects(s::RayFlags::FORCE_OPACITY_MICROMAP2_STATE_EXT) {
                    result.push(vec![spirv::Capability::RayTracingOpacityMicromapEXT])
                };
                if v.intersects(s::RayFlags::SKIP_TRIANGLES_KHR | s::RayFlags::SKIP_AAB_BS_KHR) {
                    result.push(vec![spirv::Capability::RayTraversalPrimitiveCullingKHR])
                };
                result
            }
            Self::FragmentShadingRate(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::FragmentShadingRate::VERTICAL2_PIXELS
                        | s::FragmentShadingRate::VERTICAL4_PIXELS
                        | s::FragmentShadingRate::HORIZONTAL2_PIXELS
                        | s::FragmentShadingRate::HORIZONTAL4_PIXELS,
                ) {
                    result.push(vec![spirv::Capability::FragmentShadingRateKHR])
                };
                result
            }
            Self::RawAccessChainOperands(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::RawAccessChainOperands::ROBUSTNESS_PER_COMPONENT_NV
                        | s::RawAccessChainOperands::ROBUSTNESS_PER_ELEMENT_NV,
                ) {
                    result.push(vec![spirv::Capability::RawAccessChainsNV])
                };
                result
            }
            Self::SourceLanguage(v) => match v {
                s::SourceLanguage::Unknown
                | s::SourceLanguage::ESSL
                | s::SourceLanguage::GLSL
                | s::SourceLanguage::OpenCL_C
                | s::SourceLanguage::OpenCL_CPP
                | s::SourceLanguage::HLSL
                | s::SourceLanguage::CPP_for_OpenCL
                | s::SourceLanguage::SYCL
                | s::SourceLanguage::HERO_C
                | s::SourceLanguage::NZSL
                | s::SourceLanguage::WGSL
                | s::SourceLanguage::Slang
                | s::SourceLanguage::Zig
                | s::SourceLanguage::Rust => vec![vec![]],
            },
            Self::ExecutionModel(v) => match v {
                s::ExecutionModel::Geometry => vec![vec![spirv::Capability::Geometry]],
                s::ExecutionModel::Kernel => vec![vec![spirv::Capability::Kernel]],
                s::ExecutionModel::TaskEXT | s::ExecutionModel::MeshEXT => {
                    vec![vec![spirv::Capability::MeshShadingEXT]]
                }
                s::ExecutionModel::TaskNV | s::ExecutionModel::MeshNV => vec![vec![spirv::Capability::MeshShadingNV]],
                s::ExecutionModel::RayGenerationKHR
                | s::ExecutionModel::IntersectionKHR
                | s::ExecutionModel::AnyHitKHR
                | s::ExecutionModel::ClosestHitKHR
                | s::ExecutionModel::MissKHR
                | s::ExecutionModel::CallableKHR => {
                    vec![vec![spirv::Capability::RayTracingNV, spirv::Capability::RayTracingKHR]]
                }
                s::ExecutionModel::Vertex | s::ExecutionModel::Fragment | s::ExecutionModel::GLCompute => {
                    vec![vec![spirv::Capability::Shader]]
                }
                s::ExecutionModel::TessellationControl | s::ExecutionModel::TessellationEvaluation => {
                    vec![vec![spirv::Capability::Tessellation]]
                }
            },
            Self::AddressingModel(v) => match v {
                s::AddressingModel::Logical => vec![vec![]],
                s::AddressingModel::Physical32 | s::AddressingModel::Physical64 => {
                    vec![vec![spirv::Capability::Addresses]]
                }
                s::AddressingModel::PhysicalStorageBuffer64 => {
                    vec![vec![spirv::Capability::PhysicalStorageBufferAddresses]]
                }
            },
            Self::MemoryModel(v) => match v {
                s::MemoryModel::OpenCL => vec![vec![spirv::Capability::Kernel]],
                s::MemoryModel::Simple | s::MemoryModel::GLSL450 => vec![vec![spirv::Capability::Shader]],
                s::MemoryModel::Vulkan => vec![vec![spirv::Capability::VulkanMemoryModel]],
            },
            Self::ExecutionMode(v) => match v {
                s::ExecutionMode::LocalSize | s::ExecutionMode::LocalSizeId => vec![vec![]],
                s::ExecutionMode::DerivativeGroupLinearKHR => {
                    vec![vec![spirv::Capability::ComputeDerivativeGroupLinearKHR]]
                }
                s::ExecutionMode::DerivativeGroupQuadsKHR => {
                    vec![vec![spirv::Capability::ComputeDerivativeGroupQuadsKHR]]
                }
                s::ExecutionMode::DenormFlushToZero => vec![vec![spirv::Capability::DenormFlushToZero]],
                s::ExecutionMode::DenormPreserve => vec![vec![spirv::Capability::DenormPreserve]],
                s::ExecutionMode::NumSIMDWorkitemsINTEL
                | s::ExecutionMode::SchedulerTargetFmaxMhzINTEL
                | s::ExecutionMode::StreamingInterfaceINTEL => vec![vec![spirv::Capability::FPGAKernelAttributesINTEL]],
                s::ExecutionMode::RegisterMapInterfaceINTEL => {
                    vec![vec![spirv::Capability::FPGAKernelAttributesv2INTEL]]
                }
                s::ExecutionMode::FPFastMathDefault => vec![vec![spirv::Capability::FloatControls2]],
                s::ExecutionMode::PixelInterlockOrderedEXT | s::ExecutionMode::PixelInterlockUnorderedEXT => {
                    vec![vec![spirv::Capability::FragmentShaderPixelInterlockEXT]]
                }
                s::ExecutionMode::SampleInterlockOrderedEXT | s::ExecutionMode::SampleInterlockUnorderedEXT => {
                    vec![vec![spirv::Capability::FragmentShaderSampleInterlockEXT]]
                }
                s::ExecutionMode::ShadingRateInterlockOrderedEXT
                | s::ExecutionMode::ShadingRateInterlockUnorderedEXT => {
                    vec![vec![spirv::Capability::FragmentShaderShadingRateInterlockEXT]]
                }
                s::ExecutionMode::Invocations
                | s::ExecutionMode::InputPoints
                | s::ExecutionMode::InputLines
                | s::ExecutionMode::InputLinesAdjacency
                | s::ExecutionMode::InputTrianglesAdjacency
                | s::ExecutionMode::OutputLineStrip
                | s::ExecutionMode::OutputTriangleStrip => vec![vec![spirv::Capability::Geometry]],
                s::ExecutionMode::OutputPoints => vec![vec![
                    spirv::Capability::Geometry,
                    spirv::Capability::MeshShadingNV,
                    spirv::Capability::MeshShadingEXT,
                ]],
                s::ExecutionMode::Triangles => vec![vec![spirv::Capability::Geometry, spirv::Capability::Tessellation]],
                s::ExecutionMode::OutputVertices => vec![vec![
                    spirv::Capability::Geometry,
                    spirv::Capability::Tessellation,
                    spirv::Capability::MeshShadingNV,
                    spirv::Capability::MeshShadingEXT,
                ]],
                s::ExecutionMode::LocalSizeHint
                | s::ExecutionMode::VecTypeHint
                | s::ExecutionMode::ContractionOff
                | s::ExecutionMode::Initializer
                | s::ExecutionMode::Finalizer
                | s::ExecutionMode::LocalSizeHintId => vec![vec![spirv::Capability::Kernel]],
                s::ExecutionMode::MaxWorkgroupSizeINTEL
                | s::ExecutionMode::MaxWorkDimINTEL
                | s::ExecutionMode::NoGlobalOffsetINTEL => vec![vec![spirv::Capability::KernelAttributesINTEL]],
                s::ExecutionMode::OutputLinesEXT
                | s::ExecutionMode::OutputPrimitivesEXT
                | s::ExecutionMode::OutputTrianglesEXT => vec![vec![
                    spirv::Capability::MeshShadingNV,
                    spirv::Capability::MeshShadingEXT,
                ]],
                s::ExecutionMode::ArithmeticPoisonKHR => vec![vec![spirv::Capability::PoisonFreezeKHR]],
                s::ExecutionMode::QuadDerivativesKHR | s::ExecutionMode::RequireFullQuadsKHR => {
                    vec![vec![spirv::Capability::QuadControlKHR]]
                }
                s::ExecutionMode::MaximumRegistersINTEL
                | s::ExecutionMode::MaximumRegistersIdINTEL
                | s::ExecutionMode::NamedMaximumRegistersINTEL => vec![vec![spirv::Capability::RegisterLimitsINTEL]],
                s::ExecutionMode::RoundingModeRTPINTEL
                | s::ExecutionMode::RoundingModeRTNINTEL
                | s::ExecutionMode::FloatingPointModeALTINTEL
                | s::ExecutionMode::FloatingPointModeIEEEINTEL => vec![vec![spirv::Capability::RoundToInfinityINTEL]],
                s::ExecutionMode::RoundingModeRTE => vec![vec![spirv::Capability::RoundingModeRTE]],
                s::ExecutionMode::RoundingModeRTZ => vec![vec![spirv::Capability::RoundingModeRTZ]],
                s::ExecutionMode::PostDepthCoverage => vec![vec![spirv::Capability::SampleMaskPostDepthCoverage]],
                s::ExecutionMode::PixelCenterInteger
                | s::ExecutionMode::OriginUpperLeft
                | s::ExecutionMode::OriginLowerLeft
                | s::ExecutionMode::EarlyFragmentTests
                | s::ExecutionMode::DepthReplacing
                | s::ExecutionMode::DepthGreater
                | s::ExecutionMode::DepthLess
                | s::ExecutionMode::DepthUnchanged
                | s::ExecutionMode::SubgroupUniformControlFlowKHR
                | s::ExecutionMode::EarlyAndLateFragmentTestsAMD
                | s::ExecutionMode::MaximallyReconvergesKHR => vec![vec![spirv::Capability::Shader]],
                s::ExecutionMode::Shader64BitIndexingEXT => vec![vec![spirv::Capability::Shader64BitIndexingEXT]],
                s::ExecutionMode::CoalescingAMDX
                | s::ExecutionMode::IsApiEntryAMDX
                | s::ExecutionMode::MaxNodeRecursionAMDX
                | s::ExecutionMode::StaticNumWorkgroupsAMDX
                | s::ExecutionMode::ShaderIndexAMDX
                | s::ExecutionMode::MaxNumWorkgroupsAMDX
                | s::ExecutionMode::SharesInputWithAMDX => vec![vec![spirv::Capability::ShaderEnqueueAMDX]],
                s::ExecutionMode::SignedZeroInfNanPreserve => vec![vec![spirv::Capability::SignedZeroInfNanPreserve]],
                s::ExecutionMode::StencilRefReplacingEXT
                | s::ExecutionMode::StencilRefUnchangedFrontAMD
                | s::ExecutionMode::StencilRefGreaterFrontAMD
                | s::ExecutionMode::StencilRefLessFrontAMD
                | s::ExecutionMode::StencilRefUnchangedBackAMD
                | s::ExecutionMode::StencilRefGreaterBackAMD
                | s::ExecutionMode::StencilRefLessBackAMD => vec![vec![spirv::Capability::StencilExportEXT]],
                s::ExecutionMode::SubgroupSize
                | s::ExecutionMode::SubgroupsPerWorkgroup
                | s::ExecutionMode::SubgroupsPerWorkgroupId => vec![vec![spirv::Capability::SubgroupDispatch]],
                s::ExecutionMode::SpacingEqual
                | s::ExecutionMode::SpacingFractionalEven
                | s::ExecutionMode::SpacingFractionalOdd
                | s::ExecutionMode::VertexOrderCw
                | s::ExecutionMode::VertexOrderCcw
                | s::ExecutionMode::PointMode
                | s::ExecutionMode::Quads
                | s::ExecutionMode::Isolines => vec![vec![spirv::Capability::Tessellation]],
                s::ExecutionMode::NonCoherentColorAttachmentReadEXT => {
                    vec![vec![spirv::Capability::TileImageColorReadAccessEXT]]
                }
                s::ExecutionMode::NonCoherentDepthAttachmentReadEXT => {
                    vec![vec![spirv::Capability::TileImageDepthReadAccessEXT]]
                }
                s::ExecutionMode::NonCoherentStencilAttachmentReadEXT => {
                    vec![vec![spirv::Capability::TileImageStencilReadAccessEXT]]
                }
                s::ExecutionMode::NonCoherentTileAttachmentReadQCOM | s::ExecutionMode::TileShadingRateQCOM => {
                    vec![vec![spirv::Capability::TileShadingQCOM]]
                }
                s::ExecutionMode::Xfb => vec![vec![spirv::Capability::TransformFeedback]],
                s::ExecutionMode::SharedLocalMemorySizeINTEL | s::ExecutionMode::NamedBarrierCountINTEL => {
                    vec![vec![spirv::Capability::VectorComputeINTEL]]
                }
            },
            Self::StorageClass(v) => match v {
                s::StorageClass::UniformConstant
                | s::StorageClass::Input
                | s::StorageClass::Workgroup
                | s::StorageClass::CrossWorkgroup
                | s::StorageClass::Function
                | s::StorageClass::Image => vec![vec![]],
                s::StorageClass::AtomicCounter => vec![vec![spirv::Capability::AtomicStorage]],
                s::StorageClass::CodeSectionINTEL => vec![vec![spirv::Capability::FunctionPointersINTEL]],
                s::StorageClass::Generic => vec![vec![spirv::Capability::GenericPointer]],
                s::StorageClass::TaskPayloadWorkgroupEXT => vec![vec![spirv::Capability::MeshShadingEXT]],
                s::StorageClass::PhysicalStorageBuffer => vec![vec![spirv::Capability::PhysicalStorageBufferAddresses]],
                s::StorageClass::CallableDataKHR
                | s::StorageClass::IncomingCallableDataKHR
                | s::StorageClass::RayPayloadKHR
                | s::StorageClass::HitAttributeKHR
                | s::StorageClass::IncomingRayPayloadKHR
                | s::StorageClass::ShaderRecordBufferKHR => {
                    vec![vec![spirv::Capability::RayTracingNV, spirv::Capability::RayTracingKHR]]
                }
                s::StorageClass::Uniform
                | s::StorageClass::Output
                | s::StorageClass::PushConstant
                | s::StorageClass::StorageBuffer => vec![vec![spirv::Capability::Shader]],
                s::StorageClass::Private => {
                    vec![vec![spirv::Capability::Shader, spirv::Capability::VectorComputeINTEL]]
                }
                s::StorageClass::NodePayloadAMDX => vec![vec![spirv::Capability::ShaderEnqueueAMDX]],
                s::StorageClass::HitObjectAttributeEXT => vec![vec![spirv::Capability::ShaderInvocationReorderEXT]],
                s::StorageClass::HitObjectAttributeNV => vec![vec![spirv::Capability::ShaderInvocationReorderNV]],
                s::StorageClass::TileImageEXT => vec![vec![spirv::Capability::TileImageColorReadAccessEXT]],
                s::StorageClass::TileAttachmentQCOM => vec![vec![spirv::Capability::TileShadingQCOM]],
                s::StorageClass::DeviceOnlyALTERA | s::StorageClass::HostOnlyALTERA => {
                    vec![vec![spirv::Capability::USMStorageClassesALTERA]]
                }
            },
            Self::Dim(v) => match v {
                s::Dim::Dim2D | s::Dim::Dim3D => vec![vec![]],
                s::Dim::DimSubpassData => vec![vec![spirv::Capability::InputAttachment]],
                s::Dim::Dim1D => vec![vec![spirv::Capability::Sampled1D]],
                s::Dim::DimBuffer => vec![vec![spirv::Capability::SampledBuffer]],
                s::Dim::DimRect => vec![vec![spirv::Capability::SampledRect]],
                s::Dim::DimCube => vec![vec![spirv::Capability::Shader]],
                s::Dim::DimTileImageDataEXT => vec![vec![spirv::Capability::TileImageColorReadAccessEXT]],
            },
            Self::SamplerAddressingMode(v) => match v {
                s::SamplerAddressingMode::None
                | s::SamplerAddressingMode::ClampToEdge
                | s::SamplerAddressingMode::Clamp
                | s::SamplerAddressingMode::Repeat
                | s::SamplerAddressingMode::RepeatMirrored => vec![vec![]],
            },
            Self::SamplerFilterMode(v) => match v {
                s::SamplerFilterMode::Nearest | s::SamplerFilterMode::Linear => vec![vec![]],
            },
            Self::ImageFormat(v) => match v {
                s::ImageFormat::Unknown => vec![vec![]],
                s::ImageFormat::R64ui | s::ImageFormat::R64i => vec![vec![spirv::Capability::Int64ImageEXT]],
                s::ImageFormat::Rgba32f
                | s::ImageFormat::Rgba16f
                | s::ImageFormat::R32f
                | s::ImageFormat::Rgba8
                | s::ImageFormat::Rgba8Snorm
                | s::ImageFormat::Rgba32i
                | s::ImageFormat::Rgba16i
                | s::ImageFormat::Rgba8i
                | s::ImageFormat::R32i
                | s::ImageFormat::Rgba32ui
                | s::ImageFormat::Rgba16ui
                | s::ImageFormat::Rgba8ui
                | s::ImageFormat::R32ui => vec![vec![spirv::Capability::Shader]],
                s::ImageFormat::Rg32f
                | s::ImageFormat::Rg16f
                | s::ImageFormat::R11fG11fB10f
                | s::ImageFormat::R16f
                | s::ImageFormat::Rgba16
                | s::ImageFormat::Rgb10A2
                | s::ImageFormat::Rg16
                | s::ImageFormat::Rg8
                | s::ImageFormat::R16
                | s::ImageFormat::R8
                | s::ImageFormat::Rgba16Snorm
                | s::ImageFormat::Rg16Snorm
                | s::ImageFormat::Rg8Snorm
                | s::ImageFormat::R16Snorm
                | s::ImageFormat::R8Snorm
                | s::ImageFormat::Rg32i
                | s::ImageFormat::Rg16i
                | s::ImageFormat::Rg8i
                | s::ImageFormat::R16i
                | s::ImageFormat::R8i
                | s::ImageFormat::Rgb10a2ui
                | s::ImageFormat::Rg32ui
                | s::ImageFormat::Rg16ui
                | s::ImageFormat::Rg8ui
                | s::ImageFormat::R16ui
                | s::ImageFormat::R8ui => vec![vec![spirv::Capability::StorageImageExtendedFormats]],
            },
            Self::ImageChannelOrder(v) => match v {
                s::ImageChannelOrder::R
                | s::ImageChannelOrder::A
                | s::ImageChannelOrder::RG
                | s::ImageChannelOrder::RA
                | s::ImageChannelOrder::RGB
                | s::ImageChannelOrder::RGBA
                | s::ImageChannelOrder::BGRA
                | s::ImageChannelOrder::ARGB
                | s::ImageChannelOrder::Intensity
                | s::ImageChannelOrder::Luminance
                | s::ImageChannelOrder::Rx
                | s::ImageChannelOrder::RGx
                | s::ImageChannelOrder::RGBx
                | s::ImageChannelOrder::Depth
                | s::ImageChannelOrder::DepthStencil
                | s::ImageChannelOrder::sRGB
                | s::ImageChannelOrder::sRGBx
                | s::ImageChannelOrder::sRGBA
                | s::ImageChannelOrder::sBGRA
                | s::ImageChannelOrder::ABGR => vec![vec![]],
            },
            Self::ImageChannelDataType(v) => match v {
                s::ImageChannelDataType::SnormInt8
                | s::ImageChannelDataType::SnormInt16
                | s::ImageChannelDataType::UnormInt8
                | s::ImageChannelDataType::UnormInt16
                | s::ImageChannelDataType::UnormShort565
                | s::ImageChannelDataType::UnormShort555
                | s::ImageChannelDataType::UnormInt101010
                | s::ImageChannelDataType::SignedInt8
                | s::ImageChannelDataType::SignedInt16
                | s::ImageChannelDataType::SignedInt32
                | s::ImageChannelDataType::UnsignedInt8
                | s::ImageChannelDataType::UnsignedInt16
                | s::ImageChannelDataType::UnsignedInt32
                | s::ImageChannelDataType::HalfFloat
                | s::ImageChannelDataType::Float
                | s::ImageChannelDataType::UnormInt24
                | s::ImageChannelDataType::UnormInt101010_2
                | s::ImageChannelDataType::UnormInt10X6EXT
                | s::ImageChannelDataType::UnsignedIntRaw10EXT
                | s::ImageChannelDataType::UnsignedIntRaw12EXT
                | s::ImageChannelDataType::UnormInt2_101010EXT
                | s::ImageChannelDataType::UnsignedInt10X6EXT
                | s::ImageChannelDataType::UnsignedInt12X4EXT
                | s::ImageChannelDataType::UnsignedInt14X2EXT
                | s::ImageChannelDataType::UnormInt12X4EXT
                | s::ImageChannelDataType::UnormInt14X2EXT => vec![vec![]],
            },
            Self::FPRoundingMode(v) => match v {
                s::FPRoundingMode::RTE | s::FPRoundingMode::RTZ | s::FPRoundingMode::RTP | s::FPRoundingMode::RTN => {
                    vec![vec![]]
                }
            },
            Self::FPDenormMode(v) => match v {
                s::FPDenormMode::Preserve | s::FPDenormMode::FlushToZero => {
                    vec![vec![spirv::Capability::FunctionFloatControlINTEL]]
                }
            },
            Self::QuantizationModes(v) => match v {
                s::QuantizationModes::TRN
                | s::QuantizationModes::TRN_ZERO
                | s::QuantizationModes::RND
                | s::QuantizationModes::RND_ZERO
                | s::QuantizationModes::RND_INF
                | s::QuantizationModes::RND_MIN_INF
                | s::QuantizationModes::RND_CONV
                | s::QuantizationModes::RND_CONV_ODD => {
                    vec![vec![spirv::Capability::ArbitraryPrecisionFixedPointALTERA]]
                }
            },
            Self::FPOperationMode(v) => match v {
                s::FPOperationMode::IEEE | s::FPOperationMode::ALT => {
                    vec![vec![spirv::Capability::FunctionFloatControlINTEL]]
                }
            },
            Self::OverflowModes(v) => match v {
                s::OverflowModes::WRAP
                | s::OverflowModes::SAT
                | s::OverflowModes::SAT_ZERO
                | s::OverflowModes::SAT_SYM => vec![vec![spirv::Capability::ArbitraryPrecisionFixedPointALTERA]],
            },
            Self::LinkageType(v) => match v {
                s::LinkageType::Export | s::LinkageType::Import | s::LinkageType::LinkOnceODR => {
                    vec![vec![spirv::Capability::Linkage]]
                }
            },
            Self::AccessQualifier(v) => match v {
                s::AccessQualifier::ReadOnly | s::AccessQualifier::WriteOnly | s::AccessQualifier::ReadWrite => {
                    vec![vec![spirv::Capability::Kernel]]
                }
            },
            Self::HostAccessQualifier(v) => match v {
                s::HostAccessQualifier::NoneINTEL
                | s::HostAccessQualifier::ReadINTEL
                | s::HostAccessQualifier::WriteINTEL
                | s::HostAccessQualifier::ReadWriteINTEL => {
                    vec![vec![spirv::Capability::GlobalVariableHostAccessINTEL]]
                }
            },
            Self::FunctionParameterAttribute(v) => match v {
                s::FunctionParameterAttribute::Zext
                | s::FunctionParameterAttribute::Sext
                | s::FunctionParameterAttribute::ByVal
                | s::FunctionParameterAttribute::Sret
                | s::FunctionParameterAttribute::NoAlias
                | s::FunctionParameterAttribute::NoCapture
                | s::FunctionParameterAttribute::NoWrite
                | s::FunctionParameterAttribute::NoReadWrite => vec![vec![spirv::Capability::Kernel]],
                s::FunctionParameterAttribute::RuntimeAlignedALTERA => {
                    vec![vec![spirv::Capability::RuntimeAlignedAttributeALTERA]]
                }
            },
            Self::Decoration(v) => match v {
                s::Decoration::BuiltIn
                | s::Decoration::Restrict
                | s::Decoration::Aliased
                | s::Decoration::Volatile
                | s::Decoration::Coherent
                | s::Decoration::NonWritable
                | s::Decoration::NonReadable
                | s::Decoration::FPRoundingMode
                | s::Decoration::NoSignedWrap
                | s::Decoration::NoUnsignedWrap
                | s::Decoration::WeightTextureQCOM
                | s::Decoration::BlockMatchTextureQCOM
                | s::Decoration::BlockMatchSamplerQCOM
                | s::Decoration::ExplicitInterpAMD
                | s::Decoration::CounterBuffer
                | s::Decoration::UserSemantic
                | s::Decoration::UserTypeGOOGLE => vec![vec![]],
                s::Decoration::MaxByteOffset | s::Decoration::MaxByteOffsetId => {
                    vec![vec![spirv::Capability::Addresses]]
                }
                s::Decoration::ClobberINTEL | s::Decoration::SideEffectsINTEL => {
                    vec![vec![spirv::Capability::AsmINTEL]]
                }
                s::Decoration::BindlessSamplerNV
                | s::Decoration::BindlessImageNV
                | s::Decoration::BoundSamplerNV
                | s::Decoration::BoundImageNV => vec![vec![spirv::Capability::BindlessTextureNV]],
                s::Decoration::CacheControlLoadINTEL | s::Decoration::CacheControlStoreINTEL => {
                    vec![vec![spirv::Capability::CacheControlsINTEL]]
                }
                s::Decoration::UTFEncodedKHR => vec![vec![spirv::Capability::ConstantDataKHR]],
                s::Decoration::ArrayStrideIdEXT | s::Decoration::OffsetIdEXT => {
                    vec![vec![spirv::Capability::DescriptorHeapEXT]]
                }
                s::Decoration::ConduitKernelArgumentALTERA
                | s::Decoration::RegisterMapKernelArgumentALTERA
                | s::Decoration::MMHostInterfaceAddressWidthALTERA
                | s::Decoration::MMHostInterfaceDataWidthALTERA
                | s::Decoration::MMHostInterfaceLatencyALTERA
                | s::Decoration::MMHostInterfaceReadWriteModeALTERA
                | s::Decoration::MMHostInterfaceMaxBurstALTERA
                | s::Decoration::MMHostInterfaceWaitRequestALTERA
                | s::Decoration::StableKernelArgumentALTERA => {
                    vec![vec![spirv::Capability::FPGAArgumentInterfacesALTERA]]
                }
                s::Decoration::BufferLocationALTERA => vec![vec![spirv::Capability::FPGABufferLocationALTERA]],
                s::Decoration::StallEnableALTERA => vec![vec![spirv::Capability::FPGAClusterAttributesALTERA]],
                s::Decoration::StallFreeALTERA => vec![vec![spirv::Capability::FPGAClusterAttributesV2ALTERA]],
                s::Decoration::MathOpDSPModeALTERA => vec![vec![spirv::Capability::FPGADSPControlALTERA]],
                s::Decoration::InitiationIntervalALTERA
                | s::Decoration::MaxConcurrencyALTERA
                | s::Decoration::PipelineEnableALTERA => {
                    vec![vec![spirv::Capability::FPGAInvocationPipeliningAttributesALTERA]]
                }
                s::Decoration::LatencyControlLabelALTERA | s::Decoration::LatencyControlConstraintALTERA => {
                    vec![vec![spirv::Capability::FPGALatencyControlALTERA]]
                }
                s::Decoration::BurstCoalesceALTERA
                | s::Decoration::CacheSizeALTERA
                | s::Decoration::DontStaticallyCoalesceALTERA
                | s::Decoration::PrefetchALTERA => vec![vec![spirv::Capability::FPGAMemoryAccessesALTERA]],
                s::Decoration::RegisterALTERA
                | s::Decoration::MemoryALTERA
                | s::Decoration::NumbanksALTERA
                | s::Decoration::BankwidthALTERA
                | s::Decoration::MaxPrivateCopiesALTERA
                | s::Decoration::SinglepumpALTERA
                | s::Decoration::DoublepumpALTERA
                | s::Decoration::MaxReplicatesALTERA
                | s::Decoration::SimpleDualPortALTERA
                | s::Decoration::MergeALTERA
                | s::Decoration::BankBitsALTERA
                | s::Decoration::ForcePow2DepthALTERA
                | s::Decoration::StridesizeALTERA
                | s::Decoration::WordsizeALTERA
                | s::Decoration::TrueDualPortALTERA => vec![vec![spirv::Capability::FPGAMemoryAttributesALTERA]],
                s::Decoration::FPMaxErrorDecorationINTEL => vec![vec![spirv::Capability::FPMaxErrorINTEL]],
                s::Decoration::SaturatedToLargestFloat8NormalConversionEXT => vec![vec![spirv::Capability::Float8EXT]],
                s::Decoration::PerVertexKHR => vec![vec![spirv::Capability::FragmentBarycentricKHR]],
                s::Decoration::FunctionRoundingModeINTEL
                | s::Decoration::FunctionDenormModeINTEL
                | s::Decoration::FunctionFloatingPointModeINTEL => {
                    vec![vec![spirv::Capability::FunctionFloatControlINTEL]]
                }
                s::Decoration::PassthroughNV => vec![vec![spirv::Capability::GeometryShaderPassthroughNV]],
                s::Decoration::Stream => vec![vec![spirv::Capability::GeometryStreams]],
                s::Decoration::InitModeALTERA | s::Decoration::ImplementInRegisterMapALTERA => {
                    vec![vec![spirv::Capability::GlobalVariableFPGADecorationsALTERA]]
                }
                s::Decoration::HostAccessINTEL => vec![vec![spirv::Capability::GlobalVariableHostAccessINTEL]],
                s::Decoration::IOPipeStorageALTERA => vec![vec![spirv::Capability::IOPipesALTERA]],
                s::Decoration::ReferencedIndirectlyINTEL => vec![vec![spirv::Capability::IndirectReferencesINTEL]],
                s::Decoration::InputAttachmentIndex => vec![vec![spirv::Capability::InputAttachment]],
                s::Decoration::CPacked
                | s::Decoration::Constant
                | s::Decoration::SaturatedConversion
                | s::Decoration::FuncParamAttr
                | s::Decoration::Alignment
                | s::Decoration::AlignmentId => vec![vec![spirv::Capability::Kernel]],
                s::Decoration::FPFastMathMode => {
                    vec![vec![spirv::Capability::Kernel, spirv::Capability::FloatControls2]]
                }
                s::Decoration::LinkageAttributes => vec![vec![spirv::Capability::Linkage]],
                s::Decoration::FuseLoopsInFunctionALTERA => vec![vec![spirv::Capability::LoopFuseALTERA]],
                s::Decoration::RowMajor | s::Decoration::ColMajor | s::Decoration::MatrixStride => {
                    vec![vec![spirv::Capability::Matrix]]
                }
                s::Decoration::AliasScopeINTEL | s::Decoration::NoAliasINTEL => {
                    vec![vec![spirv::Capability::MemoryAccessAliasingINTEL]]
                }
                s::Decoration::PerViewNV | s::Decoration::PerTaskNV => vec![vec![spirv::Capability::MeshShadingNV]],
                s::Decoration::PerPrimitiveEXT => vec![vec![
                    spirv::Capability::MeshShadingNV,
                    spirv::Capability::MeshShadingEXT,
                ]],
                s::Decoration::RestrictPointer | s::Decoration::AliasedPointer => {
                    vec![vec![spirv::Capability::PhysicalStorageBufferAddresses]]
                }
                s::Decoration::MemberOffsetNV | s::Decoration::BankNV => {
                    vec![vec![spirv::Capability::PushConstantBanksNV]]
                }
                s::Decoration::OverrideCoverageNV => vec![vec![spirv::Capability::SampleMaskOverrideCoverageNV]],
                s::Decoration::Sample => vec![vec![spirv::Capability::SampleRateShading]],
                s::Decoration::RelaxedPrecision
                | s::Decoration::Block
                | s::Decoration::BufferBlock
                | s::Decoration::ArrayStride
                | s::Decoration::GLSLShared
                | s::Decoration::GLSLPacked
                | s::Decoration::NoPerspective
                | s::Decoration::Flat
                | s::Decoration::Centroid
                | s::Decoration::Invariant
                | s::Decoration::Location
                | s::Decoration::Component
                | s::Decoration::Index
                | s::Decoration::Binding
                | s::Decoration::DescriptorSet
                | s::Decoration::Offset
                | s::Decoration::NoContraction => vec![vec![spirv::Capability::Shader]],
                s::Decoration::SpecId => vec![vec![spirv::Capability::Shader, spirv::Capability::Kernel]],
                s::Decoration::Uniform | s::Decoration::UniformId => {
                    vec![vec![spirv::Capability::Shader, spirv::Capability::UniformDecoration]]
                }
                s::Decoration::NodeSharesPayloadLimitsWithAMDX
                | s::Decoration::NodeMaxPayloadsAMDX
                | s::Decoration::TrackFinishWritingAMDX
                | s::Decoration::PayloadNodeNameAMDX
                | s::Decoration::PayloadNodeBaseIndexAMDX
                | s::Decoration::PayloadNodeSparseArrayAMDX
                | s::Decoration::PayloadNodeArraySizeAMDX
                | s::Decoration::PayloadDispatchIndirectAMDX => vec![vec![spirv::Capability::ShaderEnqueueAMDX]],
                s::Decoration::HitObjectShaderRecordBufferEXT => {
                    vec![vec![spirv::Capability::ShaderInvocationReorderEXT]]
                }
                s::Decoration::HitObjectShaderRecordBufferNV => {
                    vec![vec![spirv::Capability::ShaderInvocationReorderNV]]
                }
                s::Decoration::NonUniform => vec![vec![spirv::Capability::ShaderNonUniform]],
                s::Decoration::SecondaryViewportRelativeNV => vec![vec![spirv::Capability::ShaderStereoViewNV]],
                s::Decoration::ViewportRelativeNV => vec![vec![spirv::Capability::ShaderViewportMaskNV]],
                s::Decoration::ConditionalINTEL => vec![vec![spirv::Capability::SpecConditionalINTEL]],
                s::Decoration::Patch => vec![vec![spirv::Capability::Tessellation]],
                s::Decoration::XfbBuffer | s::Decoration::XfbStride => vec![vec![spirv::Capability::TransformFeedback]],
                s::Decoration::SIMTCallINTEL
                | s::Decoration::VectorComputeVariableINTEL
                | s::Decoration::FuncParamIOKindINTEL
                | s::Decoration::VectorComputeFunctionINTEL
                | s::Decoration::StackCallINTEL
                | s::Decoration::GlobalVariableOffsetINTEL
                | s::Decoration::SingleElementVectorINTEL
                | s::Decoration::VectorComputeCallableFunctionINTEL
                | s::Decoration::MediaBlockIOINTEL => vec![vec![spirv::Capability::VectorComputeINTEL]],
            },
            Self::BuiltIn(v) => match v {
                s::BuiltIn::NumWorkgroups
                | s::BuiltIn::WorkgroupSize
                | s::BuiltIn::WorkgroupId
                | s::BuiltIn::LocalInvocationId
                | s::BuiltIn::GlobalInvocationId
                | s::BuiltIn::LocalInvocationIndex
                | s::BuiltIn::BaryCoordNoPerspAMD
                | s::BuiltIn::BaryCoordNoPerspCentroidAMD
                | s::BuiltIn::BaryCoordNoPerspSampleAMD
                | s::BuiltIn::BaryCoordSmoothAMD
                | s::BuiltIn::BaryCoordSmoothCentroidAMD
                | s::BuiltIn::BaryCoordSmoothSampleAMD
                | s::BuiltIn::BaryCoordPullModelAMD => vec![vec![]],
                s::BuiltIn::ClipDistance => vec![vec![spirv::Capability::ClipDistance]],
                s::BuiltIn::CoreIDARM
                | s::BuiltIn::CoreCountARM
                | s::BuiltIn::CoreMaxIDARM
                | s::BuiltIn::WarpIDARM
                | s::BuiltIn::WarpMaxIDARM => vec![vec![spirv::Capability::CoreBuiltinsARM]],
                s::BuiltIn::CullDistance => vec![vec![spirv::Capability::CullDistance]],
                s::BuiltIn::SamplerHeapEXT | s::BuiltIn::ResourceHeapEXT => {
                    vec![vec![spirv::Capability::DescriptorHeapEXT]]
                }
                s::BuiltIn::DeviceIndex => vec![vec![spirv::Capability::DeviceGroup]],
                s::BuiltIn::BaseVertex | s::BuiltIn::BaseInstance => vec![vec![spirv::Capability::DrawParameters]],
                s::BuiltIn::DrawIndex => vec![vec![
                    spirv::Capability::DrawParameters,
                    spirv::Capability::MeshShadingNV,
                    spirv::Capability::MeshShadingEXT,
                ]],
                s::BuiltIn::BaryCoordKHR | s::BuiltIn::BaryCoordNoPerspKHR => {
                    vec![vec![spirv::Capability::FragmentBarycentricKHR]]
                }
                s::BuiltIn::FragSizeEXT | s::BuiltIn::FragInvocationCountEXT => {
                    vec![vec![spirv::Capability::FragmentDensityEXT]]
                }
                s::BuiltIn::FullyCoveredEXT => vec![vec![spirv::Capability::FragmentFullyCoveredEXT]],
                s::BuiltIn::PrimitiveShadingRateKHR | s::BuiltIn::ShadingRateKHR => {
                    vec![vec![spirv::Capability::FragmentShadingRateKHR]]
                }
                s::BuiltIn::Layer => vec![vec![
                    spirv::Capability::Geometry,
                    spirv::Capability::ShaderLayer,
                    spirv::Capability::ShaderViewportIndexLayerEXT,
                    spirv::Capability::MeshShadingNV,
                    spirv::Capability::MeshShadingEXT,
                ]],
                s::BuiltIn::InvocationId => vec![vec![spirv::Capability::Geometry, spirv::Capability::Tessellation]],
                s::BuiltIn::PrimitiveId => vec![vec![
                    spirv::Capability::Geometry,
                    spirv::Capability::Tessellation,
                    spirv::Capability::RayTracingNV,
                    spirv::Capability::RayTracingKHR,
                    spirv::Capability::MeshShadingNV,
                    spirv::Capability::MeshShadingEXT,
                ]],
                s::BuiltIn::WorkDim
                | s::BuiltIn::GlobalSize
                | s::BuiltIn::EnqueuedWorkgroupSize
                | s::BuiltIn::GlobalOffset
                | s::BuiltIn::GlobalLinearId
                | s::BuiltIn::SubgroupMaxSize
                | s::BuiltIn::NumEnqueuedSubgroups => vec![vec![spirv::Capability::Kernel]],
                s::BuiltIn::NumSubgroups | s::BuiltIn::SubgroupId => {
                    vec![vec![spirv::Capability::Kernel, spirv::Capability::GroupNonUniform]]
                }
                s::BuiltIn::SubgroupSize | s::BuiltIn::SubgroupLocalInvocationId => vec![vec![
                    spirv::Capability::Kernel,
                    spirv::Capability::GroupNonUniform,
                    spirv::Capability::SubgroupBallotKHR,
                ]],
                s::BuiltIn::PrimitivePointIndicesEXT
                | s::BuiltIn::PrimitiveLineIndicesEXT
                | s::BuiltIn::PrimitiveTriangleIndicesEXT
                | s::BuiltIn::CullPrimitiveEXT => vec![vec![spirv::Capability::MeshShadingEXT]],
                s::BuiltIn::TaskCountNV
                | s::BuiltIn::PrimitiveCountNV
                | s::BuiltIn::PrimitiveIndicesNV
                | s::BuiltIn::ClipDistancePerViewNV
                | s::BuiltIn::CullDistancePerViewNV
                | s::BuiltIn::LayerPerViewNV
                | s::BuiltIn::MeshViewCountNV
                | s::BuiltIn::MeshViewIndicesNV => vec![vec![spirv::Capability::MeshShadingNV]],
                s::BuiltIn::ViewIndex => vec![vec![spirv::Capability::MultiView]],
                s::BuiltIn::ViewportIndex => vec![vec![
                    spirv::Capability::MultiViewport,
                    spirv::Capability::ShaderViewportIndex,
                    spirv::Capability::ShaderViewportIndexLayerEXT,
                    spirv::Capability::MeshShadingNV,
                    spirv::Capability::MeshShadingEXT,
                ]],
                s::BuiltIn::PositionPerViewNV | s::BuiltIn::ViewportMaskPerViewNV => vec![vec![
                    spirv::Capability::PerViewAttributesNV,
                    spirv::Capability::MeshShadingNV,
                ]],
                s::BuiltIn::CullMaskKHR => vec![vec![spirv::Capability::RayCullMaskKHR]],
                s::BuiltIn::ClusterIDNV => vec![vec![spirv::Capability::RayTracingClusterAccelerationStructureNV]],
                s::BuiltIn::HitMicroTriangleVertexPositionsNV
                | s::BuiltIn::HitMicroTriangleVertexBarycentricsNV
                | s::BuiltIn::HitKindFrontFacingMicroTriangleNV
                | s::BuiltIn::HitKindBackFacingMicroTriangleNV => {
                    vec![vec![spirv::Capability::RayTracingDisplacementMicromapNV]]
                }
                s::BuiltIn::RayGeometryIndexKHR => vec![vec![spirv::Capability::RayTracingKHR]],
                s::BuiltIn::HitIsLSSNV | s::BuiltIn::HitLSSPositionsNV | s::BuiltIn::HitLSSRadiiNV => {
                    vec![vec![spirv::Capability::RayTracingLinearSweptSpheresGeometryNV]]
                }
                s::BuiltIn::CurrentRayTimeNV => vec![vec![spirv::Capability::RayTracingMotionBlurNV]],
                s::BuiltIn::HitTNV => vec![vec![spirv::Capability::RayTracingNV]],
                s::BuiltIn::LaunchIdKHR
                | s::BuiltIn::LaunchSizeKHR
                | s::BuiltIn::WorldRayOriginKHR
                | s::BuiltIn::WorldRayDirectionKHR
                | s::BuiltIn::ObjectRayOriginKHR
                | s::BuiltIn::ObjectRayDirectionKHR
                | s::BuiltIn::RayTminKHR
                | s::BuiltIn::RayTmaxKHR
                | s::BuiltIn::InstanceCustomIndexKHR
                | s::BuiltIn::ObjectToWorldKHR
                | s::BuiltIn::WorldToObjectKHR
                | s::BuiltIn::HitKindKHR
                | s::BuiltIn::IncomingRayFlagsKHR => {
                    vec![vec![spirv::Capability::RayTracingNV, spirv::Capability::RayTracingKHR]]
                }
                s::BuiltIn::HitTriangleVertexPositionsKHR => vec![vec![spirv::Capability::RayTracingPositionFetchKHR]],
                s::BuiltIn::HitIsSphereNV | s::BuiltIn::HitSpherePositionNV | s::BuiltIn::HitSphereRadiusNV => {
                    vec![vec![spirv::Capability::RayTracingSpheresGeometryNV]]
                }
                s::BuiltIn::SampleId | s::BuiltIn::SamplePosition => vec![vec![spirv::Capability::SampleRateShading]],
                s::BuiltIn::Position
                | s::BuiltIn::PointSize
                | s::BuiltIn::VertexId
                | s::BuiltIn::InstanceId
                | s::BuiltIn::FragCoord
                | s::BuiltIn::PointCoord
                | s::BuiltIn::FrontFacing
                | s::BuiltIn::SampleMask
                | s::BuiltIn::FragDepth
                | s::BuiltIn::HelperInvocation
                | s::BuiltIn::VertexIndex
                | s::BuiltIn::InstanceIndex => vec![vec![spirv::Capability::Shader]],
                s::BuiltIn::RemainingRecursionLevelsAMDX | s::BuiltIn::ShaderIndexAMDX => {
                    vec![vec![spirv::Capability::ShaderEnqueueAMDX]]
                }
                s::BuiltIn::WarpsPerSMNV | s::BuiltIn::SMCountNV | s::BuiltIn::WarpIDNV | s::BuiltIn::SMIDNV => {
                    vec![vec![spirv::Capability::ShaderSMBuiltinsNV]]
                }
                s::BuiltIn::SecondaryPositionNV | s::BuiltIn::SecondaryViewportMaskNV => {
                    vec![vec![spirv::Capability::ShaderStereoViewNV]]
                }
                s::BuiltIn::ViewportMaskNV => vec![vec![
                    spirv::Capability::ShaderViewportMaskNV,
                    spirv::Capability::MeshShadingNV,
                ]],
                s::BuiltIn::FragStencilRefEXT => vec![vec![spirv::Capability::StencilExportEXT]],
                s::BuiltIn::SubgroupEqMask
                | s::BuiltIn::SubgroupGeMask
                | s::BuiltIn::SubgroupGtMask
                | s::BuiltIn::SubgroupLeMask
                | s::BuiltIn::SubgroupLtMask => vec![vec![
                    spirv::Capability::SubgroupBallotKHR,
                    spirv::Capability::GroupNonUniformBallot,
                ]],
                s::BuiltIn::TessLevelOuter
                | s::BuiltIn::TessLevelInner
                | s::BuiltIn::TessCoord
                | s::BuiltIn::PatchVertices => vec![vec![spirv::Capability::Tessellation]],
                s::BuiltIn::TileOffsetQCOM | s::BuiltIn::TileDimensionQCOM | s::BuiltIn::TileApronSizeQCOM => {
                    vec![vec![spirv::Capability::TileShadingQCOM]]
                }
            },
            Self::Scope(v) => match v {
                s::Scope::CrossDevice
                | s::Scope::Device
                | s::Scope::Workgroup
                | s::Scope::Subgroup
                | s::Scope::Invocation => vec![vec![]],
                s::Scope::ShaderCallKHR => vec![vec![spirv::Capability::RayTracingKHR]],
                s::Scope::QueueFamily => vec![vec![spirv::Capability::VulkanMemoryModel]],
            },
            Self::GroupOperation(v) => match v {
                s::GroupOperation::ClusteredReduce => vec![vec![spirv::Capability::GroupNonUniformClustered]],
                s::GroupOperation::PartitionedReduceEXT
                | s::GroupOperation::PartitionedInclusiveScanEXT
                | s::GroupOperation::PartitionedExclusiveScanEXT => {
                    vec![vec![spirv::Capability::GroupNonUniformPartitionedEXT]]
                }
                s::GroupOperation::Reduce | s::GroupOperation::InclusiveScan | s::GroupOperation::ExclusiveScan => {
                    vec![vec![
                        spirv::Capability::Kernel,
                        spirv::Capability::GroupNonUniformArithmetic,
                        spirv::Capability::GroupNonUniformBallot,
                    ]]
                }
            },
            Self::KernelEnqueueFlags(v) => match v {
                s::KernelEnqueueFlags::NoWait
                | s::KernelEnqueueFlags::WaitKernel
                | s::KernelEnqueueFlags::WaitWorkGroup => vec![vec![spirv::Capability::Kernel]],
            },
            Self::Capability(v) => match v {
                s::Capability::Matrix
                | s::Capability::Addresses
                | s::Capability::Linkage
                | s::Capability::Kernel
                | s::Capability::Float16
                | s::Capability::Float64
                | s::Capability::Int64
                | s::Capability::Groups
                | s::Capability::Int16
                | s::Capability::Int8
                | s::Capability::Sampled1D
                | s::Capability::SampledBuffer
                | s::Capability::GroupNonUniform
                | s::Capability::ShaderLayer
                | s::Capability::ShaderViewportIndex
                | s::Capability::UniformDecoration
                | s::Capability::CoreBuiltinsARM
                | s::Capability::TileImageColorReadAccessEXT
                | s::Capability::TileImageDepthReadAccessEXT
                | s::Capability::TileImageStencilReadAccessEXT
                | s::Capability::TensorsARM
                | s::Capability::StorageTensorArrayDynamicIndexingARM
                | s::Capability::StorageTensorArrayNonUniformIndexingARM
                | s::Capability::GraphARM
                | s::Capability::CooperativeMatrixLayoutsARM
                | s::Capability::Float8EXT
                | s::Capability::SubgroupBallotKHR
                | s::Capability::SubgroupVoteKHR
                | s::Capability::StorageBuffer16BitAccess
                | s::Capability::StoragePushConstant16
                | s::Capability::StorageInputOutput16
                | s::Capability::DeviceGroup
                | s::Capability::SampleMaskPostDepthCoverage
                | s::Capability::StorageBuffer8BitAccess
                | s::Capability::StoragePushConstant8
                | s::Capability::DenormPreserve
                | s::Capability::DenormFlushToZero
                | s::Capability::SignedZeroInfNanPreserve
                | s::Capability::RoundingModeRTE
                | s::Capability::RoundingModeRTZ
                | s::Capability::UntypedPointersKHR
                | s::Capability::TextureSampleWeightedQCOM
                | s::Capability::TextureBoxFilterQCOM
                | s::Capability::TextureBlockMatchQCOM
                | s::Capability::TextureBlockMatch2QCOM
                | s::Capability::ShaderClockKHR
                | s::Capability::QuadControlKHR
                | s::Capability::Int4TypeINTEL
                | s::Capability::BFloat16TypeKHR
                | s::Capability::AbortKHR
                | s::Capability::ConstantDataKHR
                | s::Capability::PoisonFreezeKHR
                | s::Capability::ImageFootprintNV
                | s::Capability::FragmentBarycentricKHR
                | s::Capability::GroupNonUniformPartitionedEXT
                | s::Capability::VulkanMemoryModel
                | s::Capability::VulkanMemoryModelDeviceScope
                | s::Capability::BindlessTextureNV
                | s::Capability::CooperativeVectorNV
                | s::Capability::AtomicFloat16VectorNV
                | s::Capability::RawAccessChainsNV
                | s::Capability::RayTracingSpheresGeometryNV
                | s::Capability::RayTracingLinearSweptSpheresGeometryNV
                | s::Capability::LongVectorEXT
                | s::Capability::Shader64BitIndexingEXT
                | s::Capability::CooperativeMatrixReductionsNV
                | s::Capability::CooperativeMatrixConversionsNV
                | s::Capability::CooperativeMatrixPerElementOperationsNV
                | s::Capability::CooperativeMatrixTensorAddressingNV
                | s::Capability::CooperativeMatrixBlockLoadsNV
                | s::Capability::CooperativeVectorTrainingNV
                | s::Capability::TensorAddressingNV
                | s::Capability::SubgroupShuffleINTEL
                | s::Capability::SubgroupBufferBlockIOINTEL
                | s::Capability::SubgroupImageBlockIOINTEL
                | s::Capability::SubgroupImageMediaBlockIOINTEL
                | s::Capability::RoundToInfinityINTEL
                | s::Capability::FloatingPointModeINTEL
                | s::Capability::IntegerFunctions2INTEL
                | s::Capability::FunctionPointersINTEL
                | s::Capability::IndirectReferencesINTEL
                | s::Capability::AsmINTEL
                | s::Capability::AtomicFloat32MinMaxEXT
                | s::Capability::AtomicFloat64MinMaxEXT
                | s::Capability::AtomicFloat16MinMaxEXT
                | s::Capability::VectorAnyINTEL
                | s::Capability::ExpectAssumeKHR
                | s::Capability::SubgroupAvcMotionEstimationINTEL
                | s::Capability::SubgroupAvcMotionEstimationIntraINTEL
                | s::Capability::SubgroupAvcMotionEstimationChromaINTEL
                | s::Capability::VariableLengthArrayINTEL
                | s::Capability::FunctionFloatControlINTEL
                | s::Capability::FPGAMemoryAttributesALTERA
                | s::Capability::ArbitraryPrecisionIntegersALTERA
                | s::Capability::ArbitraryPrecisionFloatingPointALTERA
                | s::Capability::UnstructuredLoopControlsINTEL
                | s::Capability::FPGALoopControlsALTERA
                | s::Capability::KernelAttributesINTEL
                | s::Capability::FPGAKernelAttributesINTEL
                | s::Capability::FPGAMemoryAccessesALTERA
                | s::Capability::FPGAClusterAttributesALTERA
                | s::Capability::LoopFuseALTERA
                | s::Capability::FPGADSPControlALTERA
                | s::Capability::MemoryAccessAliasingINTEL
                | s::Capability::FPGAInvocationPipeliningAttributesALTERA
                | s::Capability::FPGABufferLocationALTERA
                | s::Capability::ArbitraryPrecisionFixedPointALTERA
                | s::Capability::USMStorageClassesALTERA
                | s::Capability::RuntimeAlignedAttributeALTERA
                | s::Capability::IOPipesALTERA
                | s::Capability::BlockingPipesALTERA
                | s::Capability::FPGARegALTERA
                | s::Capability::DotProductInputAll
                | s::Capability::DotProductInput4x8BitPacked
                | s::Capability::DotProduct
                | s::Capability::RayCullMaskKHR
                | s::Capability::CooperativeMatrixKHR
                | s::Capability::ReplicatedCompositesEXT
                | s::Capability::BitInstructions
                | s::Capability::FloatControls2
                | s::Capability::FMAKHR
                | s::Capability::AtomicFloat32AddEXT
                | s::Capability::AtomicFloat64AddEXT
                | s::Capability::LongCompositesINTEL
                | s::Capability::OptNoneEXT
                | s::Capability::AtomicFloat16AddEXT
                | s::Capability::DebugInfoModuleINTEL
                | s::Capability::BFloat16ConversionINTEL
                | s::Capability::SplitBarrierINTEL
                | s::Capability::ArithmeticFenceEXT
                | s::Capability::TaskSequenceALTERA
                | s::Capability::FPMaxErrorINTEL
                | s::Capability::FPGALatencyControlALTERA
                | s::Capability::FPGAArgumentInterfacesALTERA
                | s::Capability::GlobalVariableHostAccessINTEL
                | s::Capability::GlobalVariableFPGADecorationsALTERA
                | s::Capability::SubgroupBufferPrefetchINTEL
                | s::Capability::Subgroup2DBlockIOINTEL
                | s::Capability::SubgroupMatrixMultiplyAccumulateINTEL
                | s::Capability::TernaryBitwiseFunctionINTEL
                | s::Capability::SpecConditionalINTEL
                | s::Capability::GroupUniformArithmeticKHR
                | s::Capability::TensorFloat32RoundingINTEL
                | s::Capability::MaskedGatherScatterINTEL
                | s::Capability::CacheControlsINTEL
                | s::Capability::RegisterLimitsINTEL
                | s::Capability::BindlessImagesINTEL => vec![vec![]],
                s::Capability::GenericPointer => vec![vec![spirv::Capability::Addresses]],
                s::Capability::AtomicStorageOps => vec![vec![spirv::Capability::AtomicStorage]],
                s::Capability::BFloat16DotProductKHR | s::Capability::DotProductBFloat16AccVALVE => {
                    vec![vec![spirv::Capability::BFloat16TypeKHR]]
                }
                s::Capability::BFloat16CooperativeMatrixKHR => vec![vec![
                    spirv::Capability::BFloat16TypeKHR,
                    spirv::Capability::CooperativeMatrixKHR,
                ]],
                s::Capability::CooperativeMatrixConversionQCOM => vec![vec![spirv::Capability::CooperativeMatrixKHR]],
                s::Capability::SubgroupDispatch => vec![vec![spirv::Capability::DeviceEnqueue]],
                s::Capability::FPGAClusterAttributesV2ALTERA => {
                    vec![vec![spirv::Capability::FPGAClusterAttributesALTERA]]
                }
                s::Capability::FPGAKernelAttributesv2INTEL => vec![vec![spirv::Capability::FPGAKernelAttributesINTEL]],
                s::Capability::DotProductFloat16AccFloat32VALVE | s::Capability::DotProductFloat16AccFloat16VALVE => {
                    vec![vec![spirv::Capability::Float16]]
                }
                s::Capability::DotProductFloat8AccFloat32VALVE => vec![vec![spirv::Capability::Float8EXT]],
                s::Capability::Float8CooperativeMatrixEXT => vec![vec![
                    spirv::Capability::Float8EXT,
                    spirv::Capability::CooperativeMatrixKHR,
                ]],
                s::Capability::GeometryPointSize
                | s::Capability::GeometryStreams
                | s::Capability::MultiViewport
                | s::Capability::GeometryShaderPassthroughNV => vec![vec![spirv::Capability::Geometry]],
                s::Capability::GroupNonUniformVote
                | s::Capability::GroupNonUniformArithmetic
                | s::Capability::GroupNonUniformBallot
                | s::Capability::GroupNonUniformShuffle
                | s::Capability::GroupNonUniformShuffleRelative
                | s::Capability::GroupNonUniformClustered
                | s::Capability::GroupNonUniformQuad
                | s::Capability::GroupNonUniformRotateKHR => vec![vec![spirv::Capability::GroupNonUniform]],
                s::Capability::ImageReadWrite | s::Capability::ImageMipmap => vec![vec![spirv::Capability::ImageBasic]],
                s::Capability::StorageTexelBufferArrayDynamicIndexing => vec![vec![spirv::Capability::ImageBuffer]],
                s::Capability::StorageTexelBufferArrayNonUniformIndexing => vec![vec![
                    spirv::Capability::ImageBuffer,
                    spirv::Capability::ShaderNonUniform,
                ]],
                s::Capability::InputAttachmentArrayDynamicIndexing => vec![vec![spirv::Capability::InputAttachment]],
                s::Capability::InputAttachmentArrayNonUniformIndexing => vec![vec![
                    spirv::Capability::InputAttachment,
                    spirv::Capability::ShaderNonUniform,
                ]],
                s::Capability::Int4CooperativeMatrixINTEL => vec![vec![
                    spirv::Capability::Int4TypeINTEL,
                    spirv::Capability::CooperativeMatrixKHR,
                ]],
                s::Capability::Int64Atomics => vec![vec![spirv::Capability::Int64]],
                s::Capability::DotProductInput4x8Bit => vec![vec![spirv::Capability::Int8]],
                s::Capability::Vector16
                | s::Capability::Float16Buffer
                | s::Capability::ImageBasic
                | s::Capability::Pipes
                | s::Capability::DeviceEnqueue
                | s::Capability::LiteralSampler
                | s::Capability::NamedBarrier
                | s::Capability::FPFastMathModeINTEL => vec![vec![spirv::Capability::Kernel]],
                s::Capability::Shader => vec![vec![spirv::Capability::Matrix]],
                s::Capability::PerViewAttributesNV => vec![vec![spirv::Capability::MultiView]],
                s::Capability::ShaderViewportIndexLayerEXT => vec![vec![spirv::Capability::MultiViewport]],
                s::Capability::PipeStorage => vec![vec![spirv::Capability::Pipes]],
                s::Capability::RayTraversalPrimitiveCullingKHR => {
                    vec![vec![spirv::Capability::RayQueryKHR, spirv::Capability::RayTracingKHR]]
                }
                s::Capability::ShaderInvocationReorderNV
                | s::Capability::ShaderInvocationReorderEXT
                | s::Capability::RayTracingDisplacementMicromapNV
                | s::Capability::RayTracingClusterAccelerationStructureNV => {
                    vec![vec![spirv::Capability::RayTracingKHR]]
                }
                s::Capability::SampleMaskOverrideCoverageNV => vec![vec![spirv::Capability::SampleRateShading]],
                s::Capability::Image1D => vec![vec![spirv::Capability::Sampled1D]],
                s::Capability::ImageBuffer | s::Capability::UniformTexelBufferArrayDynamicIndexing => {
                    vec![vec![spirv::Capability::SampledBuffer]]
                }
                s::Capability::UniformTexelBufferArrayNonUniformIndexing => vec![vec![
                    spirv::Capability::SampledBuffer,
                    spirv::Capability::ShaderNonUniform,
                ]],
                s::Capability::ImageCubeArray => vec![vec![spirv::Capability::SampledCubeArray]],
                s::Capability::ImageRect => vec![vec![spirv::Capability::SampledRect]],
                s::Capability::Geometry
                | s::Capability::Tessellation
                | s::Capability::AtomicStorage
                | s::Capability::ImageGatherExtended
                | s::Capability::StorageImageMultisample
                | s::Capability::UniformBufferArrayDynamicIndexing
                | s::Capability::SampledImageArrayDynamicIndexing
                | s::Capability::StorageBufferArrayDynamicIndexing
                | s::Capability::StorageImageArrayDynamicIndexing
                | s::Capability::ClipDistance
                | s::Capability::CullDistance
                | s::Capability::SampleRateShading
                | s::Capability::SampledRect
                | s::Capability::InputAttachment
                | s::Capability::SparseResidency
                | s::Capability::MinLod
                | s::Capability::SampledCubeArray
                | s::Capability::ImageMSArray
                | s::Capability::StorageImageExtendedFormats
                | s::Capability::ImageQuery
                | s::Capability::DerivativeControl
                | s::Capability::InterpolationFunction
                | s::Capability::TransformFeedback
                | s::Capability::StorageImageReadWithoutFormat
                | s::Capability::StorageImageWriteWithoutFormat
                | s::Capability::FragmentShadingRateKHR
                | s::Capability::DrawParameters
                | s::Capability::WorkgroupMemoryExplicitLayoutKHR
                | s::Capability::MultiView
                | s::Capability::VariablePointersStorageBuffer
                | s::Capability::RayQueryProvisionalKHR
                | s::Capability::RayQueryKHR
                | s::Capability::RayTracingKHR
                | s::Capability::TileShadingQCOM
                | s::Capability::Float16ImageAMD
                | s::Capability::ImageGatherBiasLodAMD
                | s::Capability::FragmentMaskAMD
                | s::Capability::StencilExportEXT
                | s::Capability::ImageReadWriteLodAMD
                | s::Capability::Int64ImageEXT
                | s::Capability::ShaderEnqueueAMDX
                | s::Capability::FragmentFullyCoveredEXT
                | s::Capability::MeshShadingNV
                | s::Capability::MeshShadingEXT
                | s::Capability::ComputeDerivativeGroupQuadsKHR
                | s::Capability::FragmentDensityEXT
                | s::Capability::ShaderNonUniform
                | s::Capability::RuntimeDescriptorArray
                | s::Capability::RayTracingPositionFetchKHR
                | s::Capability::RayTracingNV
                | s::Capability::RayTracingMotionBlurNV
                | s::Capability::PhysicalStorageBufferAddresses
                | s::Capability::ComputeDerivativeGroupLinearKHR
                | s::Capability::RayTracingProvisionalKHR
                | s::Capability::CooperativeMatrixNV
                | s::Capability::FragmentShaderSampleInterlockEXT
                | s::Capability::FragmentShaderShadingRateInterlockEXT
                | s::Capability::ShaderSMBuiltinsNV
                | s::Capability::FragmentShaderPixelInterlockEXT
                | s::Capability::DemoteToHelperInvocation
                | s::Capability::DisplacementMicromapNV
                | s::Capability::RayTracingOpacityMicromapEXT
                | s::Capability::RayQueryPositionFetchKHR
                | s::Capability::PushConstantBanksNV => vec![vec![spirv::Capability::Shader]],
                s::Capability::UniformBufferArrayNonUniformIndexing
                | s::Capability::SampledImageArrayNonUniformIndexing
                | s::Capability::StorageBufferArrayNonUniformIndexing
                | s::Capability::StorageImageArrayNonUniformIndexing => vec![vec![spirv::Capability::ShaderNonUniform]],
                s::Capability::ShaderViewportMaskNV => vec![vec![spirv::Capability::ShaderViewportIndexLayerEXT]],
                s::Capability::ShaderStereoViewNV => vec![vec![spirv::Capability::ShaderViewportMaskNV]],
                s::Capability::FunctionVariantsINTEL => vec![vec![spirv::Capability::SpecConditionalINTEL]],
                s::Capability::UniformAndStorageBuffer16BitAccess => {
                    vec![vec![spirv::Capability::StorageBuffer16BitAccess]]
                }
                s::Capability::UniformAndStorageBuffer8BitAccess => {
                    vec![vec![spirv::Capability::StorageBuffer8BitAccess]]
                }
                s::Capability::Subgroup2DBlockTransformINTEL | s::Capability::Subgroup2DBlockTransposeINTEL => {
                    vec![vec![spirv::Capability::Subgroup2DBlockIOINTEL]]
                }
                s::Capability::TessellationPointSize => vec![vec![spirv::Capability::Tessellation]],
                s::Capability::DescriptorHeapEXT => vec![vec![spirv::Capability::UntypedPointersKHR]],
                s::Capability::UntypedVariableLengthArrayINTEL => vec![vec![
                    spirv::Capability::VariableLengthArrayINTEL,
                    spirv::Capability::UntypedPointersKHR,
                ]],
                s::Capability::VariablePointers => vec![vec![spirv::Capability::VariablePointersStorageBuffer]],
                s::Capability::VectorComputeINTEL => vec![vec![spirv::Capability::VectorAnyINTEL]],
                s::Capability::WorkgroupMemoryExplicitLayout8BitAccessKHR
                | s::Capability::WorkgroupMemoryExplicitLayout16BitAccessKHR => {
                    vec![vec![spirv::Capability::WorkgroupMemoryExplicitLayoutKHR]]
                }
            },
            Self::RayQueryIntersection(v) => match v {
                s::RayQueryIntersection::RayQueryCandidateIntersectionKHR
                | s::RayQueryIntersection::RayQueryCommittedIntersectionKHR => {
                    vec![vec![spirv::Capability::RayQueryKHR]]
                }
            },
            Self::RayQueryCommittedIntersectionType(v) => match v {
                s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionNoneKHR
                | s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionTriangleKHR
                | s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionGeneratedKHR => {
                    vec![vec![spirv::Capability::RayQueryKHR]]
                }
            },
            Self::RayQueryCandidateIntersectionType(v) => match v {
                s::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionTriangleKHR
                | s::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionAABBKHR => {
                    vec![vec![spirv::Capability::RayQueryKHR]]
                }
            },
            Self::PackedVectorFormat(v) => match v {
                s::PackedVectorFormat::PackedVectorFormat4x8Bit => vec![vec![]],
            },
            Self::CooperativeMatrixLayout(v) => match v {
                s::CooperativeMatrixLayout::RowMajorKHR | s::CooperativeMatrixLayout::ColumnMajorKHR => vec![vec![]],
                s::CooperativeMatrixLayout::RowBlockedInterleavedARM
                | s::CooperativeMatrixLayout::ColumnBlockedInterleavedARM => {
                    vec![vec![spirv::Capability::CooperativeMatrixLayoutsARM]]
                }
            },
            Self::CooperativeMatrixUse(v) => match v {
                s::CooperativeMatrixUse::MatrixAKHR
                | s::CooperativeMatrixUse::MatrixBKHR
                | s::CooperativeMatrixUse::MatrixAccumulatorKHR => vec![vec![]],
            },
            Self::TensorClampMode(v) => match v {
                s::TensorClampMode::Undefined
                | s::TensorClampMode::Constant
                | s::TensorClampMode::ClampToEdge
                | s::TensorClampMode::Repeat
                | s::TensorClampMode::RepeatMirrored => vec![vec![]],
            },
            Self::TensorAddressingOperands(v) => {
                let mut result = vec![];
                if v.intersects(s::TensorAddressingOperands::DECODE_FUNC) {
                    result.push(vec![spirv::Capability::CooperativeMatrixBlockLoadsNV])
                };
                if v.intersects(s::TensorAddressingOperands::TENSOR_VIEW) {
                    result.push(vec![spirv::Capability::CooperativeMatrixTensorAddressingNV])
                };
                result
            }
            Self::InitializationModeQualifier(v) => match v {
                s::InitializationModeQualifier::InitOnDeviceReprogramALTERA
                | s::InitializationModeQualifier::InitOnDeviceResetALTERA => {
                    vec![vec![spirv::Capability::GlobalVariableFPGADecorationsALTERA]]
                }
            },
            Self::LoadCacheControl(v) => match v {
                s::LoadCacheControl::UncachedINTEL
                | s::LoadCacheControl::CachedINTEL
                | s::LoadCacheControl::StreamingINTEL
                | s::LoadCacheControl::InvalidateAfterReadINTEL
                | s::LoadCacheControl::ConstCachedINTEL => vec![vec![spirv::Capability::CacheControlsINTEL]],
            },
            Self::StoreCacheControl(v) => match v {
                s::StoreCacheControl::UncachedINTEL
                | s::StoreCacheControl::WriteThroughINTEL
                | s::StoreCacheControl::WriteBackINTEL
                | s::StoreCacheControl::StreamingINTEL => vec![vec![spirv::Capability::CacheControlsINTEL]],
            },
            Self::NamedMaximumNumberOfRegisters(v) => match v {
                s::NamedMaximumNumberOfRegisters::AutoINTEL => vec![vec![spirv::Capability::RegisterLimitsINTEL]],
            },
            Self::FPEncoding(v) => match v {
                s::FPEncoding::BFloat16KHR => vec![vec![spirv::Capability::BFloat16TypeKHR]],
                s::FPEncoding::Float8E4M3EXT | s::FPEncoding::Float8E5M2EXT => vec![vec![spirv::Capability::Float8EXT]],
            },
            Self::CooperativeVectorMatrixLayout(v) => match v {
                s::CooperativeVectorMatrixLayout::RowMajorNV
                | s::CooperativeVectorMatrixLayout::ColumnMajorNV
                | s::CooperativeVectorMatrixLayout::InferencingOptimalNV
                | s::CooperativeVectorMatrixLayout::TrainingOptimalNV => vec![vec![]],
            },
            Self::ComponentType(v) => match v {
                s::ComponentType::Float16NV
                | s::ComponentType::Float32NV
                | s::ComponentType::Float64NV
                | s::ComponentType::SignedInt8NV
                | s::ComponentType::SignedInt16NV
                | s::ComponentType::SignedInt32NV
                | s::ComponentType::SignedInt64NV
                | s::ComponentType::UnsignedInt8NV
                | s::ComponentType::UnsignedInt16NV
                | s::ComponentType::UnsignedInt32NV
                | s::ComponentType::UnsignedInt64NV
                | s::ComponentType::SignedInt8PackedNV
                | s::ComponentType::UnsignedInt8PackedNV
                | s::ComponentType::FloatE4M3NV
                | s::ComponentType::FloatE5M2NV => vec![vec![]],
            },
            Self::TensorOperands(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::TensorOperands::NONE_ARM
                        | s::TensorOperands::NONTEMPORAL_ARM
                        | s::TensorOperands::OUT_OF_BOUNDS_VALUE_ARM
                        | s::TensorOperands::MAKE_ELEMENT_AVAILABLE_ARM
                        | s::TensorOperands::MAKE_ELEMENT_VISIBLE_ARM
                        | s::TensorOperands::NON_PRIVATE_ELEMENT_ARM,
                ) {
                    result.push(vec![spirv::Capability::TensorsARM])
                };
                result
            }
            _ => vec![],
        }
    }
    pub fn required_extensions(&self) -> Vec<Vec<&'static str>> {
        use spirv as s;
        match self {
            Self::ImageOperands(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::ImageOperands::MAKE_TEXEL_AVAILABLE
                        | s::ImageOperands::MAKE_TEXEL_VISIBLE
                        | s::ImageOperands::NON_PRIVATE_TEXEL
                        | s::ImageOperands::VOLATILE_TEXEL,
                ) {
                    result.push(vec!["SPV_KHR_vulkan_memory_model"])
                };
                result
            }
            Self::MemorySemantics(v) => {
                let mut result = vec![];
                if v.intersects(
                    s::MemorySemantics::OUTPUT_MEMORY
                        | s::MemorySemantics::MAKE_AVAILABLE
                        | s::MemorySemantics::MAKE_VISIBLE
                        | s::MemorySemantics::VOLATILE,
                ) {
                    result.push(vec!["SPV_KHR_vulkan_memory_model"])
                };
                result
            }
            Self::MemoryAccess(v) => {
                let mut result = vec![];
                if v.intersects(s::MemoryAccess::ALIAS_SCOPE_INTEL_MASK | s::MemoryAccess::NO_ALIAS_INTEL_MASK) {
                    result.push(vec!["SPV_INTEL_memory_access_aliasing"])
                };
                if v.intersects(
                    s::MemoryAccess::MAKE_POINTER_AVAILABLE
                        | s::MemoryAccess::MAKE_POINTER_VISIBLE
                        | s::MemoryAccess::NON_PRIVATE_POINTER,
                ) {
                    result.push(vec!["SPV_KHR_vulkan_memory_model"])
                };
                result
            }
            Self::SourceLanguage(v) => match v {
                s::SourceLanguage::Unknown
                | s::SourceLanguage::ESSL
                | s::SourceLanguage::GLSL
                | s::SourceLanguage::OpenCL_C
                | s::SourceLanguage::OpenCL_CPP
                | s::SourceLanguage::HLSL
                | s::SourceLanguage::CPP_for_OpenCL
                | s::SourceLanguage::SYCL
                | s::SourceLanguage::HERO_C
                | s::SourceLanguage::NZSL
                | s::SourceLanguage::WGSL
                | s::SourceLanguage::Slang
                | s::SourceLanguage::Zig
                | s::SourceLanguage::Rust => vec![vec![]],
            },
            Self::ExecutionModel(v) => match v {
                s::ExecutionModel::Vertex
                | s::ExecutionModel::TessellationControl
                | s::ExecutionModel::TessellationEvaluation
                | s::ExecutionModel::Geometry
                | s::ExecutionModel::Fragment
                | s::ExecutionModel::GLCompute
                | s::ExecutionModel::Kernel
                | s::ExecutionModel::TaskNV
                | s::ExecutionModel::MeshNV
                | s::ExecutionModel::RayGenerationKHR
                | s::ExecutionModel::IntersectionKHR
                | s::ExecutionModel::AnyHitKHR
                | s::ExecutionModel::ClosestHitKHR
                | s::ExecutionModel::MissKHR
                | s::ExecutionModel::CallableKHR
                | s::ExecutionModel::TaskEXT
                | s::ExecutionModel::MeshEXT => vec![vec![]],
            },
            Self::AddressingModel(v) => match v {
                s::AddressingModel::Logical | s::AddressingModel::Physical32 | s::AddressingModel::Physical64 => {
                    vec![vec![]]
                }
                s::AddressingModel::PhysicalStorageBuffer64 => vec![vec![
                    "SPV_EXT_physical_storage_buffer",
                    "SPV_KHR_physical_storage_buffer",
                ]],
            },
            Self::MemoryModel(v) => match v {
                s::MemoryModel::Simple | s::MemoryModel::GLSL450 | s::MemoryModel::OpenCL => vec![vec![]],
                s::MemoryModel::Vulkan => vec![vec!["SPV_KHR_vulkan_memory_model"]],
            },
            Self::ExecutionMode(v) => match v {
                s::ExecutionMode::Invocations
                | s::ExecutionMode::SpacingEqual
                | s::ExecutionMode::SpacingFractionalEven
                | s::ExecutionMode::SpacingFractionalOdd
                | s::ExecutionMode::VertexOrderCw
                | s::ExecutionMode::VertexOrderCcw
                | s::ExecutionMode::PixelCenterInteger
                | s::ExecutionMode::OriginUpperLeft
                | s::ExecutionMode::OriginLowerLeft
                | s::ExecutionMode::EarlyFragmentTests
                | s::ExecutionMode::PointMode
                | s::ExecutionMode::Xfb
                | s::ExecutionMode::DepthReplacing
                | s::ExecutionMode::DepthGreater
                | s::ExecutionMode::DepthLess
                | s::ExecutionMode::DepthUnchanged
                | s::ExecutionMode::LocalSize
                | s::ExecutionMode::LocalSizeHint
                | s::ExecutionMode::InputPoints
                | s::ExecutionMode::InputLines
                | s::ExecutionMode::InputLinesAdjacency
                | s::ExecutionMode::Triangles
                | s::ExecutionMode::InputTrianglesAdjacency
                | s::ExecutionMode::Quads
                | s::ExecutionMode::Isolines
                | s::ExecutionMode::OutputVertices
                | s::ExecutionMode::OutputPoints
                | s::ExecutionMode::OutputLineStrip
                | s::ExecutionMode::OutputTriangleStrip
                | s::ExecutionMode::VecTypeHint
                | s::ExecutionMode::ContractionOff
                | s::ExecutionMode::Initializer
                | s::ExecutionMode::Finalizer
                | s::ExecutionMode::SubgroupSize
                | s::ExecutionMode::SubgroupsPerWorkgroup
                | s::ExecutionMode::SubgroupsPerWorkgroupId
                | s::ExecutionMode::LocalSizeId
                | s::ExecutionMode::LocalSizeHintId
                | s::ExecutionMode::NonCoherentColorAttachmentReadEXT
                | s::ExecutionMode::NonCoherentDepthAttachmentReadEXT
                | s::ExecutionMode::NonCoherentStencilAttachmentReadEXT
                | s::ExecutionMode::NonCoherentTileAttachmentReadQCOM
                | s::ExecutionMode::TileShadingRateQCOM
                | s::ExecutionMode::CoalescingAMDX
                | s::ExecutionMode::IsApiEntryAMDX
                | s::ExecutionMode::MaxNodeRecursionAMDX
                | s::ExecutionMode::StaticNumWorkgroupsAMDX
                | s::ExecutionMode::ShaderIndexAMDX
                | s::ExecutionMode::MaxNumWorkgroupsAMDX
                | s::ExecutionMode::QuadDerivativesKHR
                | s::ExecutionMode::RequireFullQuadsKHR
                | s::ExecutionMode::SharesInputWithAMDX
                | s::ExecutionMode::ArithmeticPoisonKHR
                | s::ExecutionMode::Shader64BitIndexingEXT
                | s::ExecutionMode::SharedLocalMemorySizeINTEL
                | s::ExecutionMode::RoundingModeRTPINTEL
                | s::ExecutionMode::RoundingModeRTNINTEL
                | s::ExecutionMode::FloatingPointModeALTINTEL
                | s::ExecutionMode::FloatingPointModeIEEEINTEL
                | s::ExecutionMode::SchedulerTargetFmaxMhzINTEL
                | s::ExecutionMode::FPFastMathDefault
                | s::ExecutionMode::StreamingInterfaceINTEL
                | s::ExecutionMode::RegisterMapInterfaceINTEL
                | s::ExecutionMode::NamedBarrierCountINTEL
                | s::ExecutionMode::MaximumRegistersINTEL
                | s::ExecutionMode::MaximumRegistersIdINTEL
                | s::ExecutionMode::NamedMaximumRegistersINTEL => vec![vec![]],
                s::ExecutionMode::EarlyAndLateFragmentTestsAMD => {
                    vec![vec!["SPV_AMD_shader_early_and_late_fragment_tests"]]
                }
                s::ExecutionMode::StencilRefUnchangedFrontAMD
                | s::ExecutionMode::StencilRefGreaterFrontAMD
                | s::ExecutionMode::StencilRefLessFrontAMD
                | s::ExecutionMode::StencilRefUnchangedBackAMD
                | s::ExecutionMode::StencilRefGreaterBackAMD
                | s::ExecutionMode::StencilRefLessBackAMD => vec![vec![
                    "SPV_AMD_shader_early_and_late_fragment_tests",
                    "SPV_EXT_shader_stencil_export",
                ]],
                s::ExecutionMode::PixelInterlockOrderedEXT
                | s::ExecutionMode::PixelInterlockUnorderedEXT
                | s::ExecutionMode::SampleInterlockOrderedEXT
                | s::ExecutionMode::SampleInterlockUnorderedEXT
                | s::ExecutionMode::ShadingRateInterlockOrderedEXT
                | s::ExecutionMode::ShadingRateInterlockUnorderedEXT => vec![vec!["SPV_EXT_fragment_shader_interlock"]],
                s::ExecutionMode::StencilRefReplacingEXT => vec![vec!["SPV_EXT_shader_stencil_export"]],
                s::ExecutionMode::MaxWorkgroupSizeINTEL
                | s::ExecutionMode::MaxWorkDimINTEL
                | s::ExecutionMode::NoGlobalOffsetINTEL
                | s::ExecutionMode::NumSIMDWorkitemsINTEL => vec![vec!["SPV_INTEL_kernel_attributes"]],
                s::ExecutionMode::DenormPreserve
                | s::ExecutionMode::DenormFlushToZero
                | s::ExecutionMode::SignedZeroInfNanPreserve
                | s::ExecutionMode::RoundingModeRTE
                | s::ExecutionMode::RoundingModeRTZ => vec![vec!["SPV_KHR_float_controls"]],
                s::ExecutionMode::MaximallyReconvergesKHR => vec![vec!["SPV_KHR_maximal_reconvergence"]],
                s::ExecutionMode::PostDepthCoverage => vec![vec!["SPV_KHR_post_depth_coverage"]],
                s::ExecutionMode::SubgroupUniformControlFlowKHR => vec![vec!["SPV_KHR_subgroup_uniform_control_flow"]],
                s::ExecutionMode::DerivativeGroupQuadsKHR | s::ExecutionMode::DerivativeGroupLinearKHR => vec![vec![
                    "SPV_NV_compute_shader_derivatives",
                    "SPV_KHR_compute_shader_derivatives",
                ]],
                s::ExecutionMode::OutputLinesEXT
                | s::ExecutionMode::OutputPrimitivesEXT
                | s::ExecutionMode::OutputTrianglesEXT => vec![vec!["SPV_NV_mesh_shader", "SPV_EXT_mesh_shader"]],
            },
            Self::StorageClass(v) => match v {
                s::StorageClass::UniformConstant
                | s::StorageClass::Input
                | s::StorageClass::Uniform
                | s::StorageClass::Output
                | s::StorageClass::Workgroup
                | s::StorageClass::CrossWorkgroup
                | s::StorageClass::Private
                | s::StorageClass::Function
                | s::StorageClass::Generic
                | s::StorageClass::PushConstant
                | s::StorageClass::AtomicCounter
                | s::StorageClass::Image
                | s::StorageClass::TileImageEXT
                | s::StorageClass::TileAttachmentQCOM
                | s::StorageClass::NodePayloadAMDX
                | s::StorageClass::HitObjectAttributeNV
                | s::StorageClass::HitObjectAttributeEXT
                | s::StorageClass::DeviceOnlyALTERA
                | s::StorageClass::HostOnlyALTERA => vec![vec![]],
                s::StorageClass::TaskPayloadWorkgroupEXT => vec![vec!["SPV_EXT_mesh_shader"]],
                s::StorageClass::PhysicalStorageBuffer => vec![vec![
                    "SPV_EXT_physical_storage_buffer",
                    "SPV_KHR_physical_storage_buffer",
                ]],
                s::StorageClass::CodeSectionINTEL => vec![vec!["SPV_INTEL_function_pointers"]],
                s::StorageClass::StorageBuffer => vec![vec![
                    "SPV_KHR_storage_buffer_storage_class",
                    "SPV_KHR_variable_pointers",
                ]],
                s::StorageClass::CallableDataKHR
                | s::StorageClass::IncomingCallableDataKHR
                | s::StorageClass::RayPayloadKHR
                | s::StorageClass::HitAttributeKHR
                | s::StorageClass::IncomingRayPayloadKHR
                | s::StorageClass::ShaderRecordBufferKHR => vec![vec!["SPV_NV_ray_tracing", "SPV_KHR_ray_tracing"]],
            },
            Self::Dim(v) => match v {
                s::Dim::Dim1D
                | s::Dim::Dim2D
                | s::Dim::Dim3D
                | s::Dim::DimCube
                | s::Dim::DimRect
                | s::Dim::DimBuffer
                | s::Dim::DimSubpassData
                | s::Dim::DimTileImageDataEXT => vec![vec![]],
            },
            Self::SamplerAddressingMode(v) => match v {
                s::SamplerAddressingMode::None
                | s::SamplerAddressingMode::ClampToEdge
                | s::SamplerAddressingMode::Clamp
                | s::SamplerAddressingMode::Repeat
                | s::SamplerAddressingMode::RepeatMirrored => vec![vec![]],
            },
            Self::SamplerFilterMode(v) => match v {
                s::SamplerFilterMode::Nearest | s::SamplerFilterMode::Linear => vec![vec![]],
            },
            Self::ImageFormat(v) => match v {
                s::ImageFormat::Unknown
                | s::ImageFormat::Rgba32f
                | s::ImageFormat::Rgba16f
                | s::ImageFormat::R32f
                | s::ImageFormat::Rgba8
                | s::ImageFormat::Rgba8Snorm
                | s::ImageFormat::Rg32f
                | s::ImageFormat::Rg16f
                | s::ImageFormat::R11fG11fB10f
                | s::ImageFormat::R16f
                | s::ImageFormat::Rgba16
                | s::ImageFormat::Rgb10A2
                | s::ImageFormat::Rg16
                | s::ImageFormat::Rg8
                | s::ImageFormat::R16
                | s::ImageFormat::R8
                | s::ImageFormat::Rgba16Snorm
                | s::ImageFormat::Rg16Snorm
                | s::ImageFormat::Rg8Snorm
                | s::ImageFormat::R16Snorm
                | s::ImageFormat::R8Snorm
                | s::ImageFormat::Rgba32i
                | s::ImageFormat::Rgba16i
                | s::ImageFormat::Rgba8i
                | s::ImageFormat::R32i
                | s::ImageFormat::Rg32i
                | s::ImageFormat::Rg16i
                | s::ImageFormat::Rg8i
                | s::ImageFormat::R16i
                | s::ImageFormat::R8i
                | s::ImageFormat::Rgba32ui
                | s::ImageFormat::Rgba16ui
                | s::ImageFormat::Rgba8ui
                | s::ImageFormat::R32ui
                | s::ImageFormat::Rgb10a2ui
                | s::ImageFormat::Rg32ui
                | s::ImageFormat::Rg16ui
                | s::ImageFormat::Rg8ui
                | s::ImageFormat::R16ui
                | s::ImageFormat::R8ui
                | s::ImageFormat::R64ui
                | s::ImageFormat::R64i => vec![vec![]],
            },
            Self::ImageChannelOrder(v) => match v {
                s::ImageChannelOrder::R
                | s::ImageChannelOrder::A
                | s::ImageChannelOrder::RG
                | s::ImageChannelOrder::RA
                | s::ImageChannelOrder::RGB
                | s::ImageChannelOrder::RGBA
                | s::ImageChannelOrder::BGRA
                | s::ImageChannelOrder::ARGB
                | s::ImageChannelOrder::Intensity
                | s::ImageChannelOrder::Luminance
                | s::ImageChannelOrder::Rx
                | s::ImageChannelOrder::RGx
                | s::ImageChannelOrder::RGBx
                | s::ImageChannelOrder::Depth
                | s::ImageChannelOrder::DepthStencil
                | s::ImageChannelOrder::sRGB
                | s::ImageChannelOrder::sRGBx
                | s::ImageChannelOrder::sRGBA
                | s::ImageChannelOrder::sBGRA
                | s::ImageChannelOrder::ABGR => vec![vec![]],
            },
            Self::ImageChannelDataType(v) => match v {
                s::ImageChannelDataType::SnormInt8
                | s::ImageChannelDataType::SnormInt16
                | s::ImageChannelDataType::UnormInt8
                | s::ImageChannelDataType::UnormInt16
                | s::ImageChannelDataType::UnormShort565
                | s::ImageChannelDataType::UnormShort555
                | s::ImageChannelDataType::UnormInt101010
                | s::ImageChannelDataType::SignedInt8
                | s::ImageChannelDataType::SignedInt16
                | s::ImageChannelDataType::SignedInt32
                | s::ImageChannelDataType::UnsignedInt8
                | s::ImageChannelDataType::UnsignedInt16
                | s::ImageChannelDataType::UnsignedInt32
                | s::ImageChannelDataType::HalfFloat
                | s::ImageChannelDataType::Float
                | s::ImageChannelDataType::UnormInt24
                | s::ImageChannelDataType::UnormInt101010_2
                | s::ImageChannelDataType::UnormInt10X6EXT
                | s::ImageChannelDataType::UnsignedIntRaw10EXT
                | s::ImageChannelDataType::UnsignedIntRaw12EXT
                | s::ImageChannelDataType::UnormInt2_101010EXT
                | s::ImageChannelDataType::UnsignedInt10X6EXT
                | s::ImageChannelDataType::UnsignedInt12X4EXT
                | s::ImageChannelDataType::UnsignedInt14X2EXT
                | s::ImageChannelDataType::UnormInt12X4EXT
                | s::ImageChannelDataType::UnormInt14X2EXT => vec![vec![]],
            },
            Self::FPRoundingMode(v) => match v {
                s::FPRoundingMode::RTE | s::FPRoundingMode::RTZ | s::FPRoundingMode::RTP | s::FPRoundingMode::RTN => {
                    vec![vec![]]
                }
            },
            Self::FPDenormMode(v) => match v {
                s::FPDenormMode::Preserve | s::FPDenormMode::FlushToZero => vec![vec![]],
            },
            Self::QuantizationModes(v) => match v {
                s::QuantizationModes::TRN
                | s::QuantizationModes::TRN_ZERO
                | s::QuantizationModes::RND
                | s::QuantizationModes::RND_ZERO
                | s::QuantizationModes::RND_INF
                | s::QuantizationModes::RND_MIN_INF
                | s::QuantizationModes::RND_CONV
                | s::QuantizationModes::RND_CONV_ODD => vec![vec![]],
            },
            Self::FPOperationMode(v) => match v {
                s::FPOperationMode::IEEE | s::FPOperationMode::ALT => vec![vec![]],
            },
            Self::OverflowModes(v) => match v {
                s::OverflowModes::WRAP
                | s::OverflowModes::SAT
                | s::OverflowModes::SAT_ZERO
                | s::OverflowModes::SAT_SYM => vec![vec![]],
            },
            Self::LinkageType(v) => match v {
                s::LinkageType::Export | s::LinkageType::Import => vec![vec![]],
                s::LinkageType::LinkOnceODR => vec![vec!["SPV_KHR_linkonce_odr"]],
            },
            Self::AccessQualifier(v) => match v {
                s::AccessQualifier::ReadOnly | s::AccessQualifier::WriteOnly | s::AccessQualifier::ReadWrite => {
                    vec![vec![]]
                }
            },
            Self::HostAccessQualifier(v) => match v {
                s::HostAccessQualifier::NoneINTEL
                | s::HostAccessQualifier::ReadINTEL
                | s::HostAccessQualifier::WriteINTEL
                | s::HostAccessQualifier::ReadWriteINTEL => vec![vec![]],
            },
            Self::FunctionParameterAttribute(v) => match v {
                s::FunctionParameterAttribute::Zext
                | s::FunctionParameterAttribute::Sext
                | s::FunctionParameterAttribute::ByVal
                | s::FunctionParameterAttribute::Sret
                | s::FunctionParameterAttribute::NoAlias
                | s::FunctionParameterAttribute::NoCapture
                | s::FunctionParameterAttribute::NoWrite
                | s::FunctionParameterAttribute::NoReadWrite
                | s::FunctionParameterAttribute::RuntimeAlignedALTERA => vec![vec![]],
            },
            Self::Decoration(v) => match v {
                s::Decoration::RelaxedPrecision
                | s::Decoration::SpecId
                | s::Decoration::Block
                | s::Decoration::BufferBlock
                | s::Decoration::RowMajor
                | s::Decoration::ColMajor
                | s::Decoration::ArrayStride
                | s::Decoration::MatrixStride
                | s::Decoration::GLSLShared
                | s::Decoration::GLSLPacked
                | s::Decoration::CPacked
                | s::Decoration::BuiltIn
                | s::Decoration::NoPerspective
                | s::Decoration::Flat
                | s::Decoration::Patch
                | s::Decoration::Centroid
                | s::Decoration::Sample
                | s::Decoration::Invariant
                | s::Decoration::Restrict
                | s::Decoration::Aliased
                | s::Decoration::Volatile
                | s::Decoration::Constant
                | s::Decoration::Coherent
                | s::Decoration::NonWritable
                | s::Decoration::NonReadable
                | s::Decoration::Uniform
                | s::Decoration::UniformId
                | s::Decoration::SaturatedConversion
                | s::Decoration::Stream
                | s::Decoration::Location
                | s::Decoration::Component
                | s::Decoration::Index
                | s::Decoration::Binding
                | s::Decoration::DescriptorSet
                | s::Decoration::Offset
                | s::Decoration::XfbBuffer
                | s::Decoration::XfbStride
                | s::Decoration::FuncParamAttr
                | s::Decoration::FPRoundingMode
                | s::Decoration::FPFastMathMode
                | s::Decoration::LinkageAttributes
                | s::Decoration::NoContraction
                | s::Decoration::InputAttachmentIndex
                | s::Decoration::Alignment
                | s::Decoration::MaxByteOffset
                | s::Decoration::AlignmentId
                | s::Decoration::MaxByteOffsetId
                | s::Decoration::SaturatedToLargestFloat8NormalConversionEXT
                | s::Decoration::NodeSharesPayloadLimitsWithAMDX
                | s::Decoration::NodeMaxPayloadsAMDX
                | s::Decoration::TrackFinishWritingAMDX
                | s::Decoration::PayloadNodeNameAMDX
                | s::Decoration::PayloadNodeBaseIndexAMDX
                | s::Decoration::PayloadNodeSparseArrayAMDX
                | s::Decoration::PayloadNodeArraySizeAMDX
                | s::Decoration::PayloadDispatchIndirectAMDX
                | s::Decoration::ArrayStrideIdEXT
                | s::Decoration::OffsetIdEXT
                | s::Decoration::UTFEncodedKHR
                | s::Decoration::ViewportRelativeNV
                | s::Decoration::MemberOffsetNV
                | s::Decoration::HitObjectShaderRecordBufferNV
                | s::Decoration::HitObjectShaderRecordBufferEXT
                | s::Decoration::BankNV
                | s::Decoration::BindlessSamplerNV
                | s::Decoration::BindlessImageNV
                | s::Decoration::BoundSamplerNV
                | s::Decoration::BoundImageNV
                | s::Decoration::SIMTCallINTEL
                | s::Decoration::ClobberINTEL
                | s::Decoration::SideEffectsINTEL
                | s::Decoration::VectorComputeVariableINTEL
                | s::Decoration::FuncParamIOKindINTEL
                | s::Decoration::VectorComputeFunctionINTEL
                | s::Decoration::StackCallINTEL
                | s::Decoration::GlobalVariableOffsetINTEL
                | s::Decoration::FunctionRoundingModeINTEL
                | s::Decoration::FunctionDenormModeINTEL
                | s::Decoration::RegisterALTERA
                | s::Decoration::MemoryALTERA
                | s::Decoration::NumbanksALTERA
                | s::Decoration::BankwidthALTERA
                | s::Decoration::MaxPrivateCopiesALTERA
                | s::Decoration::SinglepumpALTERA
                | s::Decoration::DoublepumpALTERA
                | s::Decoration::MaxReplicatesALTERA
                | s::Decoration::SimpleDualPortALTERA
                | s::Decoration::MergeALTERA
                | s::Decoration::BankBitsALTERA
                | s::Decoration::ForcePow2DepthALTERA
                | s::Decoration::StridesizeALTERA
                | s::Decoration::WordsizeALTERA
                | s::Decoration::TrueDualPortALTERA
                | s::Decoration::BurstCoalesceALTERA
                | s::Decoration::CacheSizeALTERA
                | s::Decoration::DontStaticallyCoalesceALTERA
                | s::Decoration::PrefetchALTERA
                | s::Decoration::StallEnableALTERA
                | s::Decoration::FuseLoopsInFunctionALTERA
                | s::Decoration::MathOpDSPModeALTERA
                | s::Decoration::AliasScopeINTEL
                | s::Decoration::NoAliasINTEL
                | s::Decoration::InitiationIntervalALTERA
                | s::Decoration::MaxConcurrencyALTERA
                | s::Decoration::PipelineEnableALTERA
                | s::Decoration::BufferLocationALTERA
                | s::Decoration::IOPipeStorageALTERA
                | s::Decoration::FunctionFloatingPointModeINTEL
                | s::Decoration::SingleElementVectorINTEL
                | s::Decoration::VectorComputeCallableFunctionINTEL
                | s::Decoration::MediaBlockIOINTEL
                | s::Decoration::StallFreeALTERA
                | s::Decoration::FPMaxErrorDecorationINTEL
                | s::Decoration::LatencyControlLabelALTERA
                | s::Decoration::LatencyControlConstraintALTERA
                | s::Decoration::ConduitKernelArgumentALTERA
                | s::Decoration::RegisterMapKernelArgumentALTERA
                | s::Decoration::MMHostInterfaceAddressWidthALTERA
                | s::Decoration::MMHostInterfaceDataWidthALTERA
                | s::Decoration::MMHostInterfaceLatencyALTERA
                | s::Decoration::MMHostInterfaceReadWriteModeALTERA
                | s::Decoration::MMHostInterfaceMaxBurstALTERA
                | s::Decoration::MMHostInterfaceWaitRequestALTERA
                | s::Decoration::StableKernelArgumentALTERA
                | s::Decoration::HostAccessINTEL
                | s::Decoration::InitModeALTERA
                | s::Decoration::ImplementInRegisterMapALTERA
                | s::Decoration::ConditionalINTEL
                | s::Decoration::CacheControlLoadINTEL
                | s::Decoration::CacheControlStoreINTEL => vec![vec![]],
                s::Decoration::ExplicitInterpAMD => vec![vec!["SPV_AMD_shader_explicit_vertex_parameter"]],
                s::Decoration::NonUniform => vec![vec!["SPV_EXT_descriptor_indexing"]],
                s::Decoration::RestrictPointer | s::Decoration::AliasedPointer => vec![vec![
                    "SPV_EXT_physical_storage_buffer",
                    "SPV_KHR_physical_storage_buffer",
                ]],
                s::Decoration::CounterBuffer | s::Decoration::UserSemantic => {
                    vec![vec!["SPV_GOOGLE_hlsl_functionality1"]]
                }
                s::Decoration::UserTypeGOOGLE => vec![vec!["SPV_GOOGLE_user_type"]],
                s::Decoration::ReferencedIndirectlyINTEL => vec![vec!["SPV_INTEL_function_pointers"]],
                s::Decoration::NoSignedWrap | s::Decoration::NoUnsignedWrap => {
                    vec![vec!["SPV_KHR_no_integer_wrap_decoration"]]
                }
                s::Decoration::PerVertexKHR => vec![vec![
                    "SPV_NV_fragment_shader_barycentric",
                    "SPV_KHR_fragment_shader_barycentric",
                ]],
                s::Decoration::PassthroughNV => vec![vec!["SPV_NV_geometry_shader_passthrough"]],
                s::Decoration::PerViewNV | s::Decoration::PerTaskNV => vec![vec!["SPV_NV_mesh_shader"]],
                s::Decoration::PerPrimitiveEXT => vec![vec!["SPV_NV_mesh_shader", "SPV_EXT_mesh_shader"]],
                s::Decoration::OverrideCoverageNV => vec![vec!["SPV_NV_sample_mask_override_coverage"]],
                s::Decoration::SecondaryViewportRelativeNV => vec![vec!["SPV_NV_stereo_view_rendering"]],
                s::Decoration::WeightTextureQCOM | s::Decoration::BlockMatchTextureQCOM => {
                    vec![vec!["SPV_QCOM_image_processing"]]
                }
                s::Decoration::BlockMatchSamplerQCOM => vec![vec!["SPV_QCOM_image_processing2"]],
            },
            Self::BuiltIn(v) => match v {
                s::BuiltIn::Position
                | s::BuiltIn::PointSize
                | s::BuiltIn::ClipDistance
                | s::BuiltIn::CullDistance
                | s::BuiltIn::VertexId
                | s::BuiltIn::InstanceId
                | s::BuiltIn::PrimitiveId
                | s::BuiltIn::InvocationId
                | s::BuiltIn::Layer
                | s::BuiltIn::ViewportIndex
                | s::BuiltIn::TessLevelOuter
                | s::BuiltIn::TessLevelInner
                | s::BuiltIn::TessCoord
                | s::BuiltIn::PatchVertices
                | s::BuiltIn::FragCoord
                | s::BuiltIn::PointCoord
                | s::BuiltIn::FrontFacing
                | s::BuiltIn::SampleId
                | s::BuiltIn::SamplePosition
                | s::BuiltIn::SampleMask
                | s::BuiltIn::FragDepth
                | s::BuiltIn::HelperInvocation
                | s::BuiltIn::NumWorkgroups
                | s::BuiltIn::WorkgroupSize
                | s::BuiltIn::WorkgroupId
                | s::BuiltIn::LocalInvocationId
                | s::BuiltIn::GlobalInvocationId
                | s::BuiltIn::LocalInvocationIndex
                | s::BuiltIn::WorkDim
                | s::BuiltIn::GlobalSize
                | s::BuiltIn::EnqueuedWorkgroupSize
                | s::BuiltIn::GlobalOffset
                | s::BuiltIn::GlobalLinearId
                | s::BuiltIn::SubgroupSize
                | s::BuiltIn::SubgroupMaxSize
                | s::BuiltIn::NumSubgroups
                | s::BuiltIn::NumEnqueuedSubgroups
                | s::BuiltIn::SubgroupId
                | s::BuiltIn::SubgroupLocalInvocationId
                | s::BuiltIn::VertexIndex
                | s::BuiltIn::InstanceIndex
                | s::BuiltIn::CoreIDARM
                | s::BuiltIn::CoreCountARM
                | s::BuiltIn::CoreMaxIDARM
                | s::BuiltIn::WarpIDARM
                | s::BuiltIn::WarpMaxIDARM
                | s::BuiltIn::TileOffsetQCOM
                | s::BuiltIn::TileDimensionQCOM
                | s::BuiltIn::TileApronSizeQCOM
                | s::BuiltIn::RemainingRecursionLevelsAMDX
                | s::BuiltIn::ShaderIndexAMDX
                | s::BuiltIn::SamplerHeapEXT
                | s::BuiltIn::ResourceHeapEXT
                | s::BuiltIn::HitTriangleVertexPositionsKHR
                | s::BuiltIn::HitMicroTriangleVertexPositionsNV
                | s::BuiltIn::HitMicroTriangleVertexBarycentricsNV
                | s::BuiltIn::HitKindFrontFacingMicroTriangleNV
                | s::BuiltIn::HitKindBackFacingMicroTriangleNV => vec![vec![]],
                s::BuiltIn::BaryCoordNoPerspAMD
                | s::BuiltIn::BaryCoordNoPerspCentroidAMD
                | s::BuiltIn::BaryCoordNoPerspSampleAMD
                | s::BuiltIn::BaryCoordSmoothAMD
                | s::BuiltIn::BaryCoordSmoothCentroidAMD
                | s::BuiltIn::BaryCoordSmoothSampleAMD
                | s::BuiltIn::BaryCoordPullModelAMD => vec![vec!["SPV_AMD_shader_explicit_vertex_parameter"]],
                s::BuiltIn::FullyCoveredEXT => vec![vec!["SPV_EXT_fragment_fully_covered"]],
                s::BuiltIn::FragSizeEXT | s::BuiltIn::FragInvocationCountEXT => {
                    vec![vec!["SPV_EXT_fragment_invocation_density", "SPV_NV_shading_rate"]]
                }
                s::BuiltIn::PrimitivePointIndicesEXT
                | s::BuiltIn::PrimitiveLineIndicesEXT
                | s::BuiltIn::PrimitiveTriangleIndicesEXT
                | s::BuiltIn::CullPrimitiveEXT => vec![vec!["SPV_EXT_mesh_shader"]],
                s::BuiltIn::FragStencilRefEXT => vec![vec!["SPV_EXT_shader_stencil_export"]],
                s::BuiltIn::DeviceIndex => vec![vec!["SPV_KHR_device_group"]],
                s::BuiltIn::PrimitiveShadingRateKHR | s::BuiltIn::ShadingRateKHR => {
                    vec![vec!["SPV_KHR_fragment_shading_rate"]]
                }
                s::BuiltIn::ViewIndex => vec![vec!["SPV_KHR_multiview"]],
                s::BuiltIn::CullMaskKHR => vec![vec!["SPV_KHR_ray_cull_mask"]],
                s::BuiltIn::RayGeometryIndexKHR => vec![vec!["SPV_KHR_ray_tracing"]],
                s::BuiltIn::SubgroupEqMask
                | s::BuiltIn::SubgroupGeMask
                | s::BuiltIn::SubgroupGtMask
                | s::BuiltIn::SubgroupLeMask
                | s::BuiltIn::SubgroupLtMask => vec![vec!["SPV_KHR_shader_ballot"]],
                s::BuiltIn::BaseVertex | s::BuiltIn::BaseInstance => vec![vec!["SPV_KHR_shader_draw_parameters"]],
                s::BuiltIn::DrawIndex => vec![vec![
                    "SPV_KHR_shader_draw_parameters",
                    "SPV_NV_mesh_shader",
                    "SPV_EXT_mesh_shader",
                ]],
                s::BuiltIn::PositionPerViewNV | s::BuiltIn::ViewportMaskPerViewNV => {
                    vec![vec!["SPV_NVX_multiview_per_view_attributes", "SPV_NV_mesh_shader"]]
                }
                s::BuiltIn::ClusterIDNV => vec![vec!["SPV_NV_cluster_acceleration_structure"]],
                s::BuiltIn::BaryCoordKHR | s::BuiltIn::BaryCoordNoPerspKHR => vec![vec![
                    "SPV_NV_fragment_shader_barycentric",
                    "SPV_KHR_fragment_shader_barycentric",
                ]],
                s::BuiltIn::HitIsSphereNV
                | s::BuiltIn::HitIsLSSNV
                | s::BuiltIn::HitSpherePositionNV
                | s::BuiltIn::HitLSSPositionsNV
                | s::BuiltIn::HitSphereRadiusNV
                | s::BuiltIn::HitLSSRadiiNV => vec![vec!["SPV_NV_linear_swept_spheres"]],
                s::BuiltIn::TaskCountNV
                | s::BuiltIn::PrimitiveCountNV
                | s::BuiltIn::PrimitiveIndicesNV
                | s::BuiltIn::ClipDistancePerViewNV
                | s::BuiltIn::CullDistancePerViewNV
                | s::BuiltIn::LayerPerViewNV
                | s::BuiltIn::MeshViewCountNV
                | s::BuiltIn::MeshViewIndicesNV => vec![vec!["SPV_NV_mesh_shader"]],
                s::BuiltIn::HitTNV => vec![vec!["SPV_NV_ray_tracing"]],
                s::BuiltIn::LaunchIdKHR
                | s::BuiltIn::LaunchSizeKHR
                | s::BuiltIn::WorldRayOriginKHR
                | s::BuiltIn::WorldRayDirectionKHR
                | s::BuiltIn::ObjectRayOriginKHR
                | s::BuiltIn::ObjectRayDirectionKHR
                | s::BuiltIn::RayTminKHR
                | s::BuiltIn::RayTmaxKHR
                | s::BuiltIn::InstanceCustomIndexKHR
                | s::BuiltIn::ObjectToWorldKHR
                | s::BuiltIn::WorldToObjectKHR
                | s::BuiltIn::HitKindKHR
                | s::BuiltIn::IncomingRayFlagsKHR => vec![vec!["SPV_NV_ray_tracing", "SPV_KHR_ray_tracing"]],
                s::BuiltIn::CurrentRayTimeNV => vec![vec!["SPV_NV_ray_tracing_motion_blur"]],
                s::BuiltIn::WarpsPerSMNV | s::BuiltIn::SMCountNV | s::BuiltIn::WarpIDNV | s::BuiltIn::SMIDNV => {
                    vec![vec!["SPV_NV_shader_sm_builtins"]]
                }
                s::BuiltIn::SecondaryPositionNV | s::BuiltIn::SecondaryViewportMaskNV => {
                    vec![vec!["SPV_NV_stereo_view_rendering"]]
                }
                s::BuiltIn::ViewportMaskNV => vec![vec!["SPV_NV_viewport_array2", "SPV_NV_mesh_shader"]],
            },
            Self::Scope(v) => match v {
                s::Scope::CrossDevice
                | s::Scope::Device
                | s::Scope::Workgroup
                | s::Scope::Subgroup
                | s::Scope::Invocation
                | s::Scope::QueueFamily
                | s::Scope::ShaderCallKHR => vec![vec![]],
            },
            Self::GroupOperation(v) => match v {
                s::GroupOperation::Reduce
                | s::GroupOperation::InclusiveScan
                | s::GroupOperation::ExclusiveScan
                | s::GroupOperation::ClusteredReduce
                | s::GroupOperation::PartitionedReduceEXT
                | s::GroupOperation::PartitionedInclusiveScanEXT
                | s::GroupOperation::PartitionedExclusiveScanEXT => vec![vec![]],
            },
            Self::KernelEnqueueFlags(v) => match v {
                s::KernelEnqueueFlags::NoWait
                | s::KernelEnqueueFlags::WaitKernel
                | s::KernelEnqueueFlags::WaitWorkGroup => vec![vec![]],
            },
            Self::Capability(v) => match v {
                s::Capability::Matrix
                | s::Capability::Shader
                | s::Capability::Geometry
                | s::Capability::Tessellation
                | s::Capability::Addresses
                | s::Capability::Linkage
                | s::Capability::Kernel
                | s::Capability::Vector16
                | s::Capability::Float16Buffer
                | s::Capability::Float16
                | s::Capability::Float64
                | s::Capability::Int64
                | s::Capability::Int64Atomics
                | s::Capability::ImageBasic
                | s::Capability::ImageReadWrite
                | s::Capability::ImageMipmap
                | s::Capability::Pipes
                | s::Capability::DeviceEnqueue
                | s::Capability::LiteralSampler
                | s::Capability::AtomicStorage
                | s::Capability::Int16
                | s::Capability::TessellationPointSize
                | s::Capability::GeometryPointSize
                | s::Capability::ImageGatherExtended
                | s::Capability::StorageImageMultisample
                | s::Capability::UniformBufferArrayDynamicIndexing
                | s::Capability::SampledImageArrayDynamicIndexing
                | s::Capability::StorageBufferArrayDynamicIndexing
                | s::Capability::StorageImageArrayDynamicIndexing
                | s::Capability::ClipDistance
                | s::Capability::CullDistance
                | s::Capability::ImageCubeArray
                | s::Capability::SampleRateShading
                | s::Capability::ImageRect
                | s::Capability::SampledRect
                | s::Capability::GenericPointer
                | s::Capability::Int8
                | s::Capability::InputAttachment
                | s::Capability::SparseResidency
                | s::Capability::MinLod
                | s::Capability::Sampled1D
                | s::Capability::Image1D
                | s::Capability::SampledCubeArray
                | s::Capability::SampledBuffer
                | s::Capability::ImageBuffer
                | s::Capability::ImageMSArray
                | s::Capability::StorageImageExtendedFormats
                | s::Capability::ImageQuery
                | s::Capability::DerivativeControl
                | s::Capability::InterpolationFunction
                | s::Capability::TransformFeedback
                | s::Capability::GeometryStreams
                | s::Capability::StorageImageReadWithoutFormat
                | s::Capability::StorageImageWriteWithoutFormat
                | s::Capability::MultiViewport
                | s::Capability::SubgroupDispatch
                | s::Capability::NamedBarrier
                | s::Capability::PipeStorage
                | s::Capability::GroupNonUniform
                | s::Capability::GroupNonUniformVote
                | s::Capability::GroupNonUniformArithmetic
                | s::Capability::GroupNonUniformBallot
                | s::Capability::GroupNonUniformShuffle
                | s::Capability::GroupNonUniformShuffleRelative
                | s::Capability::GroupNonUniformClustered
                | s::Capability::GroupNonUniformQuad
                | s::Capability::ShaderLayer
                | s::Capability::ShaderViewportIndex
                | s::Capability::UniformDecoration => vec![vec![]],
                s::Capability::ArbitraryPrecisionFixedPointALTERA => vec![vec![
                    "SPV_ALTERA_arbitrary_precision_fixed_point",
                    "SPV_INTEL_arbitrary_precision_fixed_point",
                ]],
                s::Capability::ArbitraryPrecisionFloatingPointALTERA => vec![vec![
                    "SPV_ALTERA_arbitrary_precision_floating_point",
                    "SPV_INTEL_arbitrary_precision_floating_point",
                ]],
                s::Capability::ArbitraryPrecisionIntegersALTERA => vec![vec![
                    "SPV_ALTERA_arbitrary_precision_integers",
                    "SPV_INTEL_arbitrary_precision_integers",
                ]],
                s::Capability::BlockingPipesALTERA => {
                    vec![vec!["SPV_ALTERA_blocking_pipes", "SPV_INTEL_blocking_pipes"]]
                }
                s::Capability::FPGAArgumentInterfacesALTERA => vec![vec![
                    "SPV_ALTERA_fpga_argument_interfaces",
                    "SPV_INTEL_fpga_argument_interfaces",
                ]],
                s::Capability::FPGABufferLocationALTERA => vec![vec![
                    "SPV_ALTERA_fpga_buffer_location",
                    "SPV_INTEL_fpga_buffer_location",
                ]],
                s::Capability::FPGAClusterAttributesALTERA | s::Capability::FPGAClusterAttributesV2ALTERA => {
                    vec![vec![
                        "SPV_ALTERA_fpga_cluster_attributes",
                        "SPV_INTEL_fpga_cluster_attributes",
                    ]]
                }
                s::Capability::FPGADSPControlALTERA => {
                    vec![vec!["SPV_ALTERA_fpga_dsp_control", "SPV_INTEL_fpga_dsp_control"]]
                }
                s::Capability::FPGAInvocationPipeliningAttributesALTERA => vec![vec![
                    "SPV_ALTERA_fpga_invocation_pipelining_attributes",
                    "SPV_INTEL_fpga_invocation_pipelining_attributes",
                ]],
                s::Capability::FPGALatencyControlALTERA => vec![vec![
                    "SPV_ALTERA_fpga_latency_control",
                    "SPV_INTEL_fpga_latency_control",
                ]],
                s::Capability::FPGALoopControlsALTERA => {
                    vec![vec!["SPV_ALTERA_fpga_loop_controls", "SPV_INTEL_fpga_loop_controls"]]
                }
                s::Capability::FPGAMemoryAccessesALTERA => vec![vec![
                    "SPV_ALTERA_fpga_memory_accesses",
                    "SPV_INTEL_fpga_memory_accesses",
                ]],
                s::Capability::FPGAMemoryAttributesALTERA => vec![vec![
                    "SPV_ALTERA_fpga_memory_attributes",
                    "SPV_INTEL_fpga_memory_attributes",
                ]],
                s::Capability::FPGARegALTERA => vec![vec!["SPV_ALTERA_fpga_reg", "SPV_INTEL_fpga_reg"]],
                s::Capability::GlobalVariableFPGADecorationsALTERA => vec![vec![
                    "SPV_ALTERA_global_variable_fpga_decorations",
                    "SPV_INTEL_global_variable_fpga_decorations",
                ]],
                s::Capability::IOPipesALTERA => vec![vec!["SPV_ALTERA_io_pipes", "SPV_INTEL_io_pipes"]],
                s::Capability::LoopFuseALTERA => vec![vec!["SPV_ALTERA_loop_fuse", "SPV_INTEL_loop_fuse"]],
                s::Capability::RuntimeAlignedAttributeALTERA => {
                    vec![vec!["SPV_ALTERA_runtime_aligned", "SPV_INTEL_runtime_aligned"]]
                }
                s::Capability::TaskSequenceALTERA => vec![vec!["SPV_ALTERA_task_sequence", "SPV_INTEL_task_sequence"]],
                s::Capability::USMStorageClassesALTERA => {
                    vec![vec!["SPV_ALTERA_usm_storage_classes", "SPV_INTEL_usm_storage_classes"]]
                }
                s::Capability::ShaderEnqueueAMDX => vec![vec!["SPV_AMDX_shader_enqueue"]],
                s::Capability::Float16ImageAMD => vec![vec!["SPV_AMD_gpu_shader_half_float_fetch"]],
                s::Capability::Groups => vec![vec!["SPV_AMD_shader_ballot"]],
                s::Capability::FragmentMaskAMD => vec![vec!["SPV_AMD_shader_fragment_mask"]],
                s::Capability::ImageReadWriteLodAMD => vec![vec!["SPV_AMD_shader_image_load_store_lod"]],
                s::Capability::ImageGatherBiasLodAMD => vec![vec!["SPV_AMD_texture_gather_bias_lod"]],
                s::Capability::CooperativeMatrixLayoutsARM => vec![vec!["SPV_ARM_cooperative_matrix_layouts"]],
                s::Capability::CoreBuiltinsARM => vec![vec!["SPV_ARM_core_builtins"]],
                s::Capability::GraphARM => vec![vec!["SPV_ARM_graph"]],
                s::Capability::TensorsARM
                | s::Capability::StorageTensorArrayDynamicIndexingARM
                | s::Capability::StorageTensorArrayNonUniformIndexingARM => vec![vec!["SPV_ARM_tensors"]],
                s::Capability::ArithmeticFenceEXT => vec![vec!["SPV_EXT_arithmetic_fence"]],
                s::Capability::DemoteToHelperInvocation => vec![vec!["SPV_EXT_demote_to_helper_invocation"]],
                s::Capability::DescriptorHeapEXT => vec![vec!["SPV_EXT_descriptor_heap"]],
                s::Capability::ShaderNonUniform
                | s::Capability::RuntimeDescriptorArray
                | s::Capability::InputAttachmentArrayDynamicIndexing
                | s::Capability::UniformTexelBufferArrayDynamicIndexing
                | s::Capability::StorageTexelBufferArrayDynamicIndexing
                | s::Capability::UniformBufferArrayNonUniformIndexing
                | s::Capability::SampledImageArrayNonUniformIndexing
                | s::Capability::StorageBufferArrayNonUniformIndexing
                | s::Capability::StorageImageArrayNonUniformIndexing
                | s::Capability::InputAttachmentArrayNonUniformIndexing
                | s::Capability::UniformTexelBufferArrayNonUniformIndexing
                | s::Capability::StorageTexelBufferArrayNonUniformIndexing => vec![vec!["SPV_EXT_descriptor_indexing"]],
                s::Capability::Float8EXT | s::Capability::Float8CooperativeMatrixEXT => vec![vec!["SPV_EXT_float8"]],
                s::Capability::FragmentFullyCoveredEXT => vec![vec!["SPV_EXT_fragment_fully_covered"]],
                s::Capability::FragmentDensityEXT => {
                    vec![vec!["SPV_EXT_fragment_invocation_density", "SPV_NV_shading_rate"]]
                }
                s::Capability::FragmentShaderSampleInterlockEXT
                | s::Capability::FragmentShaderShadingRateInterlockEXT
                | s::Capability::FragmentShaderPixelInterlockEXT => vec![vec!["SPV_EXT_fragment_shader_interlock"]],
                s::Capability::LongVectorEXT => vec![vec!["SPV_EXT_long_vector"]],
                s::Capability::MeshShadingEXT => vec![vec!["SPV_EXT_mesh_shader"]],
                s::Capability::RayTracingOpacityMicromapEXT => vec![vec!["SPV_EXT_opacity_micromap"]],
                s::Capability::OptNoneEXT => vec![vec!["SPV_EXT_optnone", "SPV_INTEL_optnone"]],
                s::Capability::PhysicalStorageBufferAddresses => vec![vec![
                    "SPV_EXT_physical_storage_buffer",
                    "SPV_KHR_physical_storage_buffer",
                ]],
                s::Capability::ReplicatedCompositesEXT => vec![vec!["SPV_EXT_replicated_composites"]],
                s::Capability::Shader64BitIndexingEXT => vec![vec!["SPV_EXT_shader_64bit_indexing"]],
                s::Capability::AtomicFloat16AddEXT => vec![vec!["SPV_EXT_shader_atomic_float16_add"]],
                s::Capability::AtomicFloat32AddEXT | s::Capability::AtomicFloat64AddEXT => {
                    vec![vec!["SPV_EXT_shader_atomic_float_add"]]
                }
                s::Capability::AtomicFloat32MinMaxEXT
                | s::Capability::AtomicFloat64MinMaxEXT
                | s::Capability::AtomicFloat16MinMaxEXT => vec![vec!["SPV_EXT_shader_atomic_float_min_max"]],
                s::Capability::Int64ImageEXT => vec![vec!["SPV_EXT_shader_image_int64"]],
                s::Capability::ShaderInvocationReorderEXT => vec![vec!["SPV_EXT_shader_invocation_reorder"]],
                s::Capability::StencilExportEXT => vec![vec!["SPV_EXT_shader_stencil_export"]],
                s::Capability::TileImageColorReadAccessEXT
                | s::Capability::TileImageDepthReadAccessEXT
                | s::Capability::TileImageStencilReadAccessEXT => vec![vec!["SPV_EXT_shader_tile_image"]],
                s::Capability::ShaderViewportIndexLayerEXT => {
                    vec![vec!["SPV_EXT_shader_viewport_index_layer", "SPV_NV_viewport_array2"]]
                }
                s::Capability::Subgroup2DBlockIOINTEL
                | s::Capability::Subgroup2DBlockTransformINTEL
                | s::Capability::Subgroup2DBlockTransposeINTEL => vec![vec!["SPV_INTEL_2d_block_io"]],
                s::Capability::BFloat16ConversionINTEL => vec![vec!["SPV_INTEL_bfloat16_conversion"]],
                s::Capability::BindlessImagesINTEL => vec![vec!["SPV_INTEL_bindless_images"]],
                s::Capability::CacheControlsINTEL => vec![vec!["SPV_INTEL_cache_controls"]],
                s::Capability::DebugInfoModuleINTEL => vec![vec!["SPV_INTEL_debug_module"]],
                s::Capability::SubgroupAvcMotionEstimationINTEL
                | s::Capability::SubgroupAvcMotionEstimationIntraINTEL
                | s::Capability::SubgroupAvcMotionEstimationChromaINTEL => {
                    vec![vec!["SPV_INTEL_device_side_avc_motion_estimation"]]
                }
                s::Capability::RoundToInfinityINTEL
                | s::Capability::FloatingPointModeINTEL
                | s::Capability::FunctionFloatControlINTEL => vec![vec!["SPV_INTEL_float_controls2"]],
                s::Capability::FPFastMathModeINTEL => vec![vec!["SPV_INTEL_fp_fast_math_mode"]],
                s::Capability::FPMaxErrorINTEL => vec![vec!["SPV_INTEL_fp_max_error"]],
                s::Capability::FunctionPointersINTEL | s::Capability::IndirectReferencesINTEL => {
                    vec![vec!["SPV_INTEL_function_pointers"]]
                }
                s::Capability::SpecConditionalINTEL | s::Capability::FunctionVariantsINTEL => {
                    vec![vec!["SPV_INTEL_function_variants"]]
                }
                s::Capability::GlobalVariableHostAccessINTEL => vec![vec!["SPV_INTEL_global_variable_host_access"]],
                s::Capability::AsmINTEL => vec![vec!["SPV_INTEL_inline_assembly"]],
                s::Capability::Int4TypeINTEL | s::Capability::Int4CooperativeMatrixINTEL => {
                    vec![vec!["SPV_INTEL_int4"]]
                }
                s::Capability::KernelAttributesINTEL
                | s::Capability::FPGAKernelAttributesINTEL
                | s::Capability::FPGAKernelAttributesv2INTEL => vec![vec!["SPV_INTEL_kernel_attributes"]],
                s::Capability::LongCompositesINTEL => vec![vec!["SPV_INTEL_long_composites"]],
                s::Capability::MaskedGatherScatterINTEL => vec![vec!["SPV_INTEL_masked_gather_scatter"]],
                s::Capability::RegisterLimitsINTEL => vec![vec!["SPV_INTEL_maximum_registers"]],
                s::Capability::SubgroupImageMediaBlockIOINTEL => vec![vec!["SPV_INTEL_media_block_io"]],
                s::Capability::MemoryAccessAliasingINTEL => vec![vec!["SPV_INTEL_memory_access_aliasing"]],
                s::Capability::IntegerFunctions2INTEL => vec![vec!["SPV_INTEL_shader_integer_functions2"]],
                s::Capability::SplitBarrierINTEL => vec![vec!["SPV_INTEL_split_barrier"]],
                s::Capability::SubgroupBufferPrefetchINTEL => vec![vec!["SPV_INTEL_subgroup_buffer_prefetch"]],
                s::Capability::SubgroupMatrixMultiplyAccumulateINTEL => {
                    vec![vec!["SPV_INTEL_subgroup_matrix_multiply_accumulate"]]
                }
                s::Capability::SubgroupShuffleINTEL
                | s::Capability::SubgroupBufferBlockIOINTEL
                | s::Capability::SubgroupImageBlockIOINTEL => vec![vec!["SPV_INTEL_subgroups"]],
                s::Capability::TensorFloat32RoundingINTEL => vec![vec!["SPV_INTEL_tensor_float32_conversion"]],
                s::Capability::TernaryBitwiseFunctionINTEL => vec![vec!["SPV_INTEL_ternary_bitwise_function"]],
                s::Capability::UnstructuredLoopControlsINTEL => vec![vec!["SPV_INTEL_unstructured_loop_controls"]],
                s::Capability::VariableLengthArrayINTEL | s::Capability::UntypedVariableLengthArrayINTEL => {
                    vec![vec!["SPV_INTEL_variable_length_array"]]
                }
                s::Capability::VectorComputeINTEL | s::Capability::VectorAnyINTEL => {
                    vec![vec!["SPV_INTEL_vector_compute"]]
                }
                s::Capability::StorageBuffer16BitAccess
                | s::Capability::UniformAndStorageBuffer16BitAccess
                | s::Capability::StoragePushConstant16
                | s::Capability::StorageInputOutput16 => vec![vec!["SPV_KHR_16bit_storage"]],
                s::Capability::StorageBuffer8BitAccess
                | s::Capability::UniformAndStorageBuffer8BitAccess
                | s::Capability::StoragePushConstant8 => vec![vec!["SPV_KHR_8bit_storage"]],
                s::Capability::AbortKHR => vec![vec!["SPV_KHR_abort"]],
                s::Capability::BFloat16TypeKHR
                | s::Capability::BFloat16DotProductKHR
                | s::Capability::BFloat16CooperativeMatrixKHR => vec![vec!["SPV_KHR_bfloat16"]],
                s::Capability::BitInstructions => vec![vec!["SPV_KHR_bit_instructions"]],
                s::Capability::ConstantDataKHR => vec![vec!["SPV_KHR_constant_data"]],
                s::Capability::CooperativeMatrixKHR => vec![vec!["SPV_KHR_cooperative_matrix"]],
                s::Capability::DeviceGroup => vec![vec!["SPV_KHR_device_group"]],
                s::Capability::ExpectAssumeKHR => vec![vec!["SPV_KHR_expect_assume"]],
                s::Capability::DenormPreserve
                | s::Capability::DenormFlushToZero
                | s::Capability::SignedZeroInfNanPreserve
                | s::Capability::RoundingModeRTE
                | s::Capability::RoundingModeRTZ => vec![vec!["SPV_KHR_float_controls"]],
                s::Capability::FloatControls2 => vec![vec!["SPV_KHR_float_controls2"]],
                s::Capability::FMAKHR => vec![vec!["SPV_KHR_fma"]],
                s::Capability::FragmentShadingRateKHR => vec![vec!["SPV_KHR_fragment_shading_rate"]],
                s::Capability::DotProductInputAll
                | s::Capability::DotProductInput4x8Bit
                | s::Capability::DotProductInput4x8BitPacked
                | s::Capability::DotProduct => vec![vec!["SPV_KHR_integer_dot_product"]],
                s::Capability::MultiView => vec![vec!["SPV_KHR_multiview"]],
                s::Capability::PoisonFreezeKHR => vec![vec!["SPV_KHR_poison_freeze"]],
                s::Capability::SampleMaskPostDepthCoverage => vec![vec!["SPV_KHR_post_depth_coverage"]],
                s::Capability::QuadControlKHR => vec![vec!["SPV_KHR_quad_control"]],
                s::Capability::RayCullMaskKHR => vec![vec!["SPV_KHR_ray_cull_mask"]],
                s::Capability::RayQueryProvisionalKHR | s::Capability::RayQueryKHR => vec![vec!["SPV_KHR_ray_query"]],
                s::Capability::RayTraversalPrimitiveCullingKHR => {
                    vec![vec!["SPV_KHR_ray_query", "SPV_KHR_ray_tracing"]]
                }
                s::Capability::RayTracingKHR | s::Capability::RayTracingProvisionalKHR => {
                    vec![vec!["SPV_KHR_ray_tracing"]]
                }
                s::Capability::RayTracingPositionFetchKHR | s::Capability::RayQueryPositionFetchKHR => {
                    vec![vec!["SPV_KHR_ray_tracing_position_fetch"]]
                }
                s::Capability::AtomicStorageOps => vec![vec!["SPV_KHR_shader_atomic_counter_ops"]],
                s::Capability::SubgroupBallotKHR => vec![vec!["SPV_KHR_shader_ballot"]],
                s::Capability::ShaderClockKHR => vec![vec!["SPV_KHR_shader_clock"]],
                s::Capability::DrawParameters => vec![vec!["SPV_KHR_shader_draw_parameters"]],
                s::Capability::GroupNonUniformRotateKHR => vec![vec!["SPV_KHR_subgroup_rotate"]],
                s::Capability::SubgroupVoteKHR => vec![vec!["SPV_KHR_subgroup_vote"]],
                s::Capability::GroupUniformArithmeticKHR => vec![vec!["SPV_KHR_uniform_group_instructions"]],
                s::Capability::UntypedPointersKHR => vec![vec!["SPV_KHR_untyped_pointers"]],
                s::Capability::VariablePointersStorageBuffer | s::Capability::VariablePointers => {
                    vec![vec!["SPV_KHR_variable_pointers"]]
                }
                s::Capability::VulkanMemoryModel | s::Capability::VulkanMemoryModelDeviceScope => {
                    vec![vec!["SPV_KHR_vulkan_memory_model"]]
                }
                s::Capability::WorkgroupMemoryExplicitLayoutKHR
                | s::Capability::WorkgroupMemoryExplicitLayout8BitAccessKHR
                | s::Capability::WorkgroupMemoryExplicitLayout16BitAccessKHR => {
                    vec![vec!["SPV_KHR_workgroup_memory_explicit_layout"]]
                }
                s::Capability::PerViewAttributesNV => vec![vec!["SPV_NVX_multiview_per_view_attributes"]],
                s::Capability::BindlessTextureNV => vec![vec!["SPV_NV_bindless_texture"]],
                s::Capability::RayTracingClusterAccelerationStructureNV => {
                    vec![vec!["SPV_NV_cluster_acceleration_structure"]]
                }
                s::Capability::ComputeDerivativeGroupQuadsKHR | s::Capability::ComputeDerivativeGroupLinearKHR => {
                    vec![vec![
                        "SPV_NV_compute_shader_derivatives",
                        "SPV_KHR_compute_shader_derivatives",
                    ]]
                }
                s::Capability::CooperativeMatrixNV => vec![vec!["SPV_NV_cooperative_matrix"]],
                s::Capability::CooperativeMatrixReductionsNV
                | s::Capability::CooperativeMatrixConversionsNV
                | s::Capability::CooperativeMatrixPerElementOperationsNV
                | s::Capability::CooperativeMatrixTensorAddressingNV
                | s::Capability::CooperativeMatrixBlockLoadsNV => vec![vec!["SPV_NV_cooperative_matrix2"]],
                s::Capability::CooperativeVectorNV | s::Capability::CooperativeVectorTrainingNV => {
                    vec![vec!["SPV_NV_cooperative_vector"]]
                }
                s::Capability::DisplacementMicromapNV | s::Capability::RayTracingDisplacementMicromapNV => {
                    vec![vec!["SPV_NV_displacement_micromap"]]
                }
                s::Capability::FragmentBarycentricKHR => vec![vec![
                    "SPV_NV_fragment_shader_barycentric",
                    "SPV_KHR_fragment_shader_barycentric",
                ]],
                s::Capability::GeometryShaderPassthroughNV => vec![vec!["SPV_NV_geometry_shader_passthrough"]],
                s::Capability::RayTracingSpheresGeometryNV | s::Capability::RayTracingLinearSweptSpheresGeometryNV => {
                    vec![vec!["SPV_NV_linear_swept_spheres"]]
                }
                s::Capability::MeshShadingNV => vec![vec!["SPV_NV_mesh_shader"]],
                s::Capability::PushConstantBanksNV => vec![vec!["SPV_NV_push_constant_bank"]],
                s::Capability::RawAccessChainsNV => vec![vec!["SPV_NV_raw_access_chains"]],
                s::Capability::RayTracingNV => vec![vec!["SPV_NV_ray_tracing"]],
                s::Capability::RayTracingMotionBlurNV => vec![vec!["SPV_NV_ray_tracing_motion_blur"]],
                s::Capability::SampleMaskOverrideCoverageNV => vec![vec!["SPV_NV_sample_mask_override_coverage"]],
                s::Capability::AtomicFloat16VectorNV => vec![vec!["SPV_NV_shader_atomic_fp16_vector"]],
                s::Capability::ImageFootprintNV => vec![vec!["SPV_NV_shader_image_footprint"]],
                s::Capability::ShaderInvocationReorderNV => vec![vec!["SPV_NV_shader_invocation_reorder"]],
                s::Capability::ShaderSMBuiltinsNV => vec![vec!["SPV_NV_shader_sm_builtins"]],
                s::Capability::GroupNonUniformPartitionedEXT => vec![vec![
                    "SPV_NV_shader_subgroup_partitioned",
                    "SPV_EXT_shader_subgroup_partitioned",
                ]],
                s::Capability::ShaderStereoViewNV => vec![vec!["SPV_NV_stereo_view_rendering"]],
                s::Capability::TensorAddressingNV => vec![vec!["SPV_NV_tensor_addressing"]],
                s::Capability::ShaderViewportMaskNV => vec![vec!["SPV_NV_viewport_array2"]],
                s::Capability::CooperativeMatrixConversionQCOM => vec![vec!["SPV_QCOM_cooperative_matrix_conversion"]],
                s::Capability::TextureSampleWeightedQCOM
                | s::Capability::TextureBoxFilterQCOM
                | s::Capability::TextureBlockMatchQCOM => vec![vec!["SPV_QCOM_image_processing"]],
                s::Capability::TextureBlockMatch2QCOM => vec![vec!["SPV_QCOM_image_processing2"]],
                s::Capability::TileShadingQCOM => vec![vec!["SPV_QCOM_tile_shading"]],
                s::Capability::DotProductFloat16AccFloat32VALVE
                | s::Capability::DotProductFloat16AccFloat16VALVE
                | s::Capability::DotProductBFloat16AccVALVE
                | s::Capability::DotProductFloat8AccFloat32VALVE => vec![vec!["SPV_VALVE_mixed_float_dot_product"]],
            },
            Self::RayQueryIntersection(v) => match v {
                s::RayQueryIntersection::RayQueryCandidateIntersectionKHR
                | s::RayQueryIntersection::RayQueryCommittedIntersectionKHR => vec![vec![]],
            },
            Self::RayQueryCommittedIntersectionType(v) => match v {
                s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionNoneKHR
                | s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionTriangleKHR
                | s::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionGeneratedKHR => vec![vec![]],
            },
            Self::RayQueryCandidateIntersectionType(v) => match v {
                s::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionTriangleKHR
                | s::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionAABBKHR => vec![vec![]],
            },
            Self::PackedVectorFormat(v) => match v {
                s::PackedVectorFormat::PackedVectorFormat4x8Bit => vec![vec!["SPV_KHR_integer_dot_product"]],
            },
            Self::CooperativeMatrixLayout(v) => match v {
                s::CooperativeMatrixLayout::RowMajorKHR
                | s::CooperativeMatrixLayout::ColumnMajorKHR
                | s::CooperativeMatrixLayout::RowBlockedInterleavedARM
                | s::CooperativeMatrixLayout::ColumnBlockedInterleavedARM => vec![vec![]],
            },
            Self::CooperativeMatrixUse(v) => match v {
                s::CooperativeMatrixUse::MatrixAKHR
                | s::CooperativeMatrixUse::MatrixBKHR
                | s::CooperativeMatrixUse::MatrixAccumulatorKHR => vec![vec![]],
            },
            Self::TensorClampMode(v) => match v {
                s::TensorClampMode::Undefined
                | s::TensorClampMode::Constant
                | s::TensorClampMode::ClampToEdge
                | s::TensorClampMode::Repeat
                | s::TensorClampMode::RepeatMirrored => vec![vec![]],
            },
            Self::InitializationModeQualifier(v) => match v {
                s::InitializationModeQualifier::InitOnDeviceReprogramALTERA
                | s::InitializationModeQualifier::InitOnDeviceResetALTERA => vec![vec![]],
            },
            Self::LoadCacheControl(v) => match v {
                s::LoadCacheControl::UncachedINTEL
                | s::LoadCacheControl::CachedINTEL
                | s::LoadCacheControl::StreamingINTEL
                | s::LoadCacheControl::InvalidateAfterReadINTEL
                | s::LoadCacheControl::ConstCachedINTEL => vec![vec![]],
            },
            Self::StoreCacheControl(v) => match v {
                s::StoreCacheControl::UncachedINTEL
                | s::StoreCacheControl::WriteThroughINTEL
                | s::StoreCacheControl::WriteBackINTEL
                | s::StoreCacheControl::StreamingINTEL => vec![vec![]],
            },
            Self::NamedMaximumNumberOfRegisters(v) => match v {
                s::NamedMaximumNumberOfRegisters::AutoINTEL => vec![vec![]],
            },
            Self::FPEncoding(v) => match v {
                s::FPEncoding::BFloat16KHR | s::FPEncoding::Float8E4M3EXT | s::FPEncoding::Float8E5M2EXT => {
                    vec![vec![]]
                }
            },
            Self::CooperativeVectorMatrixLayout(v) => match v {
                s::CooperativeVectorMatrixLayout::RowMajorNV
                | s::CooperativeVectorMatrixLayout::ColumnMajorNV
                | s::CooperativeVectorMatrixLayout::InferencingOptimalNV
                | s::CooperativeVectorMatrixLayout::TrainingOptimalNV => vec![vec![]],
            },
            Self::ComponentType(v) => match v {
                s::ComponentType::Float16NV
                | s::ComponentType::Float32NV
                | s::ComponentType::Float64NV
                | s::ComponentType::SignedInt8NV
                | s::ComponentType::SignedInt16NV
                | s::ComponentType::SignedInt32NV
                | s::ComponentType::SignedInt64NV
                | s::ComponentType::UnsignedInt8NV
                | s::ComponentType::UnsignedInt16NV
                | s::ComponentType::UnsignedInt32NV
                | s::ComponentType::UnsignedInt64NV
                | s::ComponentType::SignedInt8PackedNV
                | s::ComponentType::UnsignedInt8PackedNV
                | s::ComponentType::FloatE4M3NV
                | s::ComponentType::FloatE5M2NV => vec![vec![]],
            },
            _ => vec![],
        }
    }
    pub fn additional_operands(&self) -> Vec<crate::grammar::LogicalOperand> {
        use spirv as s;
        match self {
            Self::ImageOperands(v) => {
                let mut result = vec![];
                result.extend(
                    [
                        s::ImageOperands::BIAS,
                        s::ImageOperands::LOD,
                        s::ImageOperands::CONST_OFFSET,
                        s::ImageOperands::OFFSET,
                        s::ImageOperands::CONST_OFFSETS,
                        s::ImageOperands::SAMPLE,
                        s::ImageOperands::MIN_LOD,
                        s::ImageOperands::OFFSETS,
                    ]
                    .iter()
                    .filter(|arg| v.contains(**arg))
                    .flat_map(|_| {
                        [crate::grammar::LogicalOperand {
                            kind: crate::grammar::OperandKind::IdRef,
                            quantifier: crate::grammar::OperandQuantifier::One,
                        }]
                        .iter()
                        .cloned()
                    }),
                );
                result.extend(
                    [s::ImageOperands::GRAD]
                        .iter()
                        .filter(|arg| v.contains(**arg))
                        .flat_map(|_| {
                            [
                                crate::grammar::LogicalOperand {
                                    kind: crate::grammar::OperandKind::IdRef,
                                    quantifier: crate::grammar::OperandQuantifier::One,
                                },
                                crate::grammar::LogicalOperand {
                                    kind: crate::grammar::OperandKind::IdRef,
                                    quantifier: crate::grammar::OperandQuantifier::One,
                                },
                            ]
                            .iter()
                            .cloned()
                        }),
                );
                result.extend(
                    [
                        s::ImageOperands::MAKE_TEXEL_AVAILABLE,
                        s::ImageOperands::MAKE_TEXEL_VISIBLE,
                    ]
                    .iter()
                    .filter(|arg| v.contains(**arg))
                    .flat_map(|_| {
                        [crate::grammar::LogicalOperand {
                            kind: crate::grammar::OperandKind::IdScope,
                            quantifier: crate::grammar::OperandQuantifier::One,
                        }]
                        .iter()
                        .cloned()
                    }),
                );
                result
            }
            Self::LoopControl(v) => {
                let mut result = vec![];
                result.extend(
                    [
                        s::LoopControl::DEPENDENCY_LENGTH,
                        s::LoopControl::MIN_ITERATIONS,
                        s::LoopControl::MAX_ITERATIONS,
                        s::LoopControl::ITERATION_MULTIPLE,
                        s::LoopControl::PEEL_COUNT,
                        s::LoopControl::PARTIAL_COUNT,
                        s::LoopControl::INITIATION_INTERVAL_ALTERA,
                        s::LoopControl::MAX_CONCURRENCY_ALTERA,
                        s::LoopControl::DEPENDENCY_ARRAY_ALTERA,
                        s::LoopControl::PIPELINE_ENABLE_ALTERA,
                        s::LoopControl::LOOP_COALESCE_ALTERA,
                        s::LoopControl::MAX_INTERLEAVING_ALTERA,
                        s::LoopControl::SPECULATED_ITERATIONS_ALTERA,
                        s::LoopControl::LOOP_COUNT_ALTERA,
                        s::LoopControl::MAX_REINVOCATION_DELAY_ALTERA,
                    ]
                    .iter()
                    .filter(|arg| v.contains(**arg))
                    .flat_map(|_| {
                        [crate::grammar::LogicalOperand {
                            kind: crate::grammar::OperandKind::LiteralInteger,
                            quantifier: crate::grammar::OperandQuantifier::One,
                        }]
                        .iter()
                        .cloned()
                    }),
                );
                result
            }
            Self::MemoryAccess(v) => {
                let mut result = vec![];
                result.extend(
                    [
                        s::MemoryAccess::ALIAS_SCOPE_INTEL_MASK,
                        s::MemoryAccess::NO_ALIAS_INTEL_MASK,
                    ]
                    .iter()
                    .filter(|arg| v.contains(**arg))
                    .flat_map(|_| {
                        [crate::grammar::LogicalOperand {
                            kind: crate::grammar::OperandKind::IdRef,
                            quantifier: crate::grammar::OperandQuantifier::One,
                        }]
                        .iter()
                        .cloned()
                    }),
                );
                result.extend(
                    [
                        s::MemoryAccess::MAKE_POINTER_AVAILABLE,
                        s::MemoryAccess::MAKE_POINTER_VISIBLE,
                    ]
                    .iter()
                    .filter(|arg| v.contains(**arg))
                    .flat_map(|_| {
                        [crate::grammar::LogicalOperand {
                            kind: crate::grammar::OperandKind::IdScope,
                            quantifier: crate::grammar::OperandQuantifier::One,
                        }]
                        .iter()
                        .cloned()
                    }),
                );
                result.extend(
                    [s::MemoryAccess::ALIGNED]
                        .iter()
                        .filter(|arg| v.contains(**arg))
                        .flat_map(|_| {
                            [crate::grammar::LogicalOperand {
                                kind: crate::grammar::OperandKind::LiteralInteger,
                                quantifier: crate::grammar::OperandQuantifier::One,
                            }]
                            .iter()
                            .cloned()
                        }),
                );
                result
            }
            Self::ExecutionMode(v) => match v {
                s::ExecutionMode::IsApiEntryAMDX => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::SharesInputWithAMDX => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::ExecutionMode::MaximumRegistersIdINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::MaxNodeRecursionAMDX => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::ShaderIndexAMDX => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::SubgroupsPerWorkgroupId => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::FPFastMathDefault => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::ExecutionMode::LocalSizeId
                | s::ExecutionMode::StaticNumWorkgroupsAMDX
                | s::ExecutionMode::MaxNumWorkgroupsAMDX => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::ExecutionMode::LocalSizeHintId => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::IdRef,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::ExecutionMode::NamedBarrierCountINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::Invocations => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::MaximumRegistersINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::OutputPrimitivesEXT => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::SharedLocalMemorySizeINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::StreamingInterfaceINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::SubgroupSize => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::SubgroupsPerWorkgroup => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::DenormPreserve
                | s::ExecutionMode::DenormFlushToZero
                | s::ExecutionMode::SignedZeroInfNanPreserve
                | s::ExecutionMode::RoundingModeRTE
                | s::ExecutionMode::RoundingModeRTZ
                | s::ExecutionMode::RoundingModeRTPINTEL
                | s::ExecutionMode::RoundingModeRTNINTEL
                | s::ExecutionMode::FloatingPointModeALTINTEL
                | s::ExecutionMode::FloatingPointModeIEEEINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::VecTypeHint => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::OutputVertices => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::RegisterMapInterfaceINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::MaxWorkDimINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::MaxWorkgroupSizeINTEL => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::ExecutionMode::SchedulerTargetFmaxMhzINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::NumSIMDWorkitemsINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::ExecutionMode::TileShadingRateQCOM => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::ExecutionMode::LocalSize | s::ExecutionMode::LocalSizeHint => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::ExecutionMode::NamedMaximumRegistersINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::NamedMaximumNumberOfRegisters,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                _ => vec![],
            },
            Self::Decoration(v) => match v {
                s::Decoration::MMHostInterfaceReadWriteModeALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::AccessQualifier,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::BuiltIn => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::BuiltIn,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::FPFastMathMode => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::FPFastMathMode,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::FPRoundingMode => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::FPRoundingMode,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::FuncParamAttr => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::FunctionParameterAttribute,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::HostAccessINTEL => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::HostAccessQualifier,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralString,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::AliasScopeINTEL | s::Decoration::NoAliasINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::AlignmentId => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::PayloadNodeArraySizeAMDX => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::ArrayStrideIdEXT => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::PayloadNodeBaseIndexAMDX => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::OffsetIdEXT => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::ConditionalINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::CounterBuffer => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MaxByteOffsetId => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::NodeMaxPayloadsAMDX => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::PayloadNodeNameAMDX => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::NodeSharesPayloadLimitsWithAMDX => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdRef,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::UniformId => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::IdScope,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::InitModeALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::InitializationModeQualifier,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::FPMaxErrorDecorationINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralFloat,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MMHostInterfaceAddressWidthALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Alignment => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::ArrayStride => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::InputAttachmentIndex => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::BankNV => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::BankBitsALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::ZeroOrMore,
                }],
                s::Decoration::BankwidthALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::NumbanksALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Binding => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::BufferLocationALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Offset => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::CacheControlLoadINTEL => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LoadCacheControl,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::CacheControlStoreINTEL => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::StoreCacheControl,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::CacheSizeALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Component => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::InitiationIntervalALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MMHostInterfaceDataWidthALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::DescriptorSet => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::PipelineEnableALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::ForcePow2DepthALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::IOPipeStorageALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Index => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MaxConcurrencyALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::FuncParamIOKindINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MMHostInterfaceLatencyALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::LatencyControlLabelALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Location => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MatrixStride => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MaxByteOffset => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MMHostInterfaceMaxBurstALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MaxPrivateCopiesALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MaxReplicatesALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MathOpDSPModeALTERA => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::SIMTCallINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::SecondaryViewportRelativeNV | s::Decoration::GlobalVariableOffsetINTEL => {
                    vec![crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    }]
                }
                s::Decoration::PrefetchALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::LatencyControlConstraintALTERA => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::SpecId => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::Stream => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::StridesizeALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::FunctionDenormModeINTEL => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::FPDenormMode,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::FunctionFloatingPointModeINTEL => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::FPOperationMode,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::FunctionRoundingModeINTEL => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralInteger,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::FPRoundingMode,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::ImplementInRegisterMapALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MMHostInterfaceWaitRequestALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::WordsizeALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::XfbBuffer => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::XfbStride => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MemberOffsetNV => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralInteger,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MemoryALTERA => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralString,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::MergeALTERA => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralString,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralString,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::LinkageAttributes => vec![
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LiteralString,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                    crate::grammar::LogicalOperand {
                        kind: crate::grammar::OperandKind::LinkageType,
                        quantifier: crate::grammar::OperandQuantifier::One,
                    },
                ],
                s::Decoration::ClobberINTEL => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralString,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::UserSemantic => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralString,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                s::Decoration::UserTypeGOOGLE => vec![crate::grammar::LogicalOperand {
                    kind: crate::grammar::OperandKind::LiteralString,
                    quantifier: crate::grammar::OperandQuantifier::One,
                }],
                _ => vec![],
            },
            Self::TensorAddressingOperands(v) => {
                let mut result = vec![];
                result.extend(
                    [
                        s::TensorAddressingOperands::TENSOR_VIEW,
                        s::TensorAddressingOperands::DECODE_FUNC,
                    ]
                    .iter()
                    .filter(|arg| v.contains(**arg))
                    .flat_map(|_| {
                        [crate::grammar::LogicalOperand {
                            kind: crate::grammar::OperandKind::IdRef,
                            quantifier: crate::grammar::OperandQuantifier::One,
                        }]
                        .iter()
                        .cloned()
                    }),
                );
                result
            }
            Self::TensorOperands(v) => {
                let mut result = vec![];
                result.extend(
                    [
                        s::TensorOperands::OUT_OF_BOUNDS_VALUE_ARM,
                        s::TensorOperands::MAKE_ELEMENT_AVAILABLE_ARM,
                        s::TensorOperands::MAKE_ELEMENT_VISIBLE_ARM,
                    ]
                    .iter()
                    .filter(|arg| v.contains(**arg))
                    .flat_map(|_| {
                        [crate::grammar::LogicalOperand {
                            kind: crate::grammar::OperandKind::IdRef,
                            quantifier: crate::grammar::OperandQuantifier::One,
                        }]
                        .iter()
                        .cloned()
                    }),
                );
                result
            }
            _ => vec![],
        }
    }
}
