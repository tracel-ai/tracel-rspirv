// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use crate::prelude::*;
use tracel_rspirv::spirv::*;
#[pliron_attr(name = "spirv.image_operands", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ImageOperandsAttr(pub ImageOperands);
impl ImageOperandsAttr {
    pub fn new(value: ImageOperands) -> Self {
        Self(value)
    }
}
impl From<ImageOperands> for ImageOperandsAttr {
    fn from(value: ImageOperands) -> Self {
        Self(value)
    }
}
impl From<ImageOperandsAttr> for ImageOperands {
    fn from(value: ImageOperandsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fp_fast_math_mode", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FPFastMathModeAttr(pub FPFastMathMode);
impl FPFastMathModeAttr {
    pub fn new(value: FPFastMathMode) -> Self {
        Self(value)
    }
}
impl From<FPFastMathMode> for FPFastMathModeAttr {
    fn from(value: FPFastMathMode) -> Self {
        Self(value)
    }
}
impl From<FPFastMathModeAttr> for FPFastMathMode {
    fn from(value: FPFastMathModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.selection_control", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct SelectionControlAttr(pub SelectionControl);
impl SelectionControlAttr {
    pub fn new(value: SelectionControl) -> Self {
        Self(value)
    }
}
impl From<SelectionControl> for SelectionControlAttr {
    fn from(value: SelectionControl) -> Self {
        Self(value)
    }
}
impl From<SelectionControlAttr> for SelectionControl {
    fn from(value: SelectionControlAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.loop_control", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct LoopControlAttr(pub LoopControl);
impl LoopControlAttr {
    pub fn new(value: LoopControl) -> Self {
        Self(value)
    }
}
impl From<LoopControl> for LoopControlAttr {
    fn from(value: LoopControl) -> Self {
        Self(value)
    }
}
impl From<LoopControlAttr> for LoopControl {
    fn from(value: LoopControlAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.function_control", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FunctionControlAttr(pub FunctionControl);
impl FunctionControlAttr {
    pub fn new(value: FunctionControl) -> Self {
        Self(value)
    }
}
impl From<FunctionControl> for FunctionControlAttr {
    fn from(value: FunctionControl) -> Self {
        Self(value)
    }
}
impl From<FunctionControlAttr> for FunctionControl {
    fn from(value: FunctionControlAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.memory_semantics", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct MemorySemanticsAttr(pub MemorySemantics);
impl MemorySemanticsAttr {
    pub fn new(value: MemorySemantics) -> Self {
        Self(value)
    }
}
impl From<MemorySemantics> for MemorySemanticsAttr {
    fn from(value: MemorySemantics) -> Self {
        Self(value)
    }
}
impl From<MemorySemanticsAttr> for MemorySemantics {
    fn from(value: MemorySemanticsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.memory_access", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct MemoryAccessAttr(pub MemoryAccess);
impl MemoryAccessAttr {
    pub fn new(value: MemoryAccess) -> Self {
        Self(value)
    }
}
impl From<MemoryAccess> for MemoryAccessAttr {
    fn from(value: MemoryAccess) -> Self {
        Self(value)
    }
}
impl From<MemoryAccessAttr> for MemoryAccess {
    fn from(value: MemoryAccessAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.kernel_profiling_info", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct KernelProfilingInfoAttr(pub KernelProfilingInfo);
impl KernelProfilingInfoAttr {
    pub fn new(value: KernelProfilingInfo) -> Self {
        Self(value)
    }
}
impl From<KernelProfilingInfo> for KernelProfilingInfoAttr {
    fn from(value: KernelProfilingInfo) -> Self {
        Self(value)
    }
}
impl From<KernelProfilingInfoAttr> for KernelProfilingInfo {
    fn from(value: KernelProfilingInfoAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.ray_flags", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct RayFlagsAttr(pub RayFlags);
impl RayFlagsAttr {
    pub fn new(value: RayFlags) -> Self {
        Self(value)
    }
}
impl From<RayFlags> for RayFlagsAttr {
    fn from(value: RayFlags) -> Self {
        Self(value)
    }
}
impl From<RayFlagsAttr> for RayFlags {
    fn from(value: RayFlagsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fragment_shading_rate", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FragmentShadingRateAttr(pub FragmentShadingRate);
impl FragmentShadingRateAttr {
    pub fn new(value: FragmentShadingRate) -> Self {
        Self(value)
    }
}
impl From<FragmentShadingRate> for FragmentShadingRateAttr {
    fn from(value: FragmentShadingRate) -> Self {
        Self(value)
    }
}
impl From<FragmentShadingRateAttr> for FragmentShadingRate {
    fn from(value: FragmentShadingRateAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.raw_access_chain_operands", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct RawAccessChainOperandsAttr(pub RawAccessChainOperands);
impl RawAccessChainOperandsAttr {
    pub fn new(value: RawAccessChainOperands) -> Self {
        Self(value)
    }
}
impl From<RawAccessChainOperands> for RawAccessChainOperandsAttr {
    fn from(value: RawAccessChainOperands) -> Self {
        Self(value)
    }
}
impl From<RawAccessChainOperandsAttr> for RawAccessChainOperands {
    fn from(value: RawAccessChainOperandsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.source_language", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct SourceLanguageAttr(pub SourceLanguage);
impl SourceLanguageAttr {
    pub fn new(value: SourceLanguage) -> Self {
        Self(value)
    }
}
impl From<SourceLanguage> for SourceLanguageAttr {
    fn from(value: SourceLanguage) -> Self {
        Self(value)
    }
}
impl From<SourceLanguageAttr> for SourceLanguage {
    fn from(value: SourceLanguageAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.execution_model", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ExecutionModelAttr(pub ExecutionModel);
impl ExecutionModelAttr {
    pub fn new(value: ExecutionModel) -> Self {
        Self(value)
    }
}
impl From<ExecutionModel> for ExecutionModelAttr {
    fn from(value: ExecutionModel) -> Self {
        Self(value)
    }
}
impl From<ExecutionModelAttr> for ExecutionModel {
    fn from(value: ExecutionModelAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.addressing_model", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct AddressingModelAttr(pub AddressingModel);
impl AddressingModelAttr {
    pub fn new(value: AddressingModel) -> Self {
        Self(value)
    }
}
impl From<AddressingModel> for AddressingModelAttr {
    fn from(value: AddressingModel) -> Self {
        Self(value)
    }
}
impl From<AddressingModelAttr> for AddressingModel {
    fn from(value: AddressingModelAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.memory_model", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct MemoryModelAttr(pub MemoryModel);
impl MemoryModelAttr {
    pub fn new(value: MemoryModel) -> Self {
        Self(value)
    }
}
impl From<MemoryModel> for MemoryModelAttr {
    fn from(value: MemoryModel) -> Self {
        Self(value)
    }
}
impl From<MemoryModelAttr> for MemoryModel {
    fn from(value: MemoryModelAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.execution_mode", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ExecutionModeAttr(pub ExecutionMode);
impl ExecutionModeAttr {
    pub fn new(value: ExecutionMode) -> Self {
        Self(value)
    }
}
impl From<ExecutionMode> for ExecutionModeAttr {
    fn from(value: ExecutionMode) -> Self {
        Self(value)
    }
}
impl From<ExecutionModeAttr> for ExecutionMode {
    fn from(value: ExecutionModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.storage_class", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct StorageClassAttr(pub StorageClass);
impl StorageClassAttr {
    pub fn new(value: StorageClass) -> Self {
        Self(value)
    }
}
impl From<StorageClass> for StorageClassAttr {
    fn from(value: StorageClass) -> Self {
        Self(value)
    }
}
impl From<StorageClassAttr> for StorageClass {
    fn from(value: StorageClassAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.dim", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct DimAttr(pub Dim);
impl DimAttr {
    pub fn new(value: Dim) -> Self {
        Self(value)
    }
}
impl From<Dim> for DimAttr {
    fn from(value: Dim) -> Self {
        Self(value)
    }
}
impl From<DimAttr> for Dim {
    fn from(value: DimAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.sampler_addressing_mode", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct SamplerAddressingModeAttr(pub SamplerAddressingMode);
impl SamplerAddressingModeAttr {
    pub fn new(value: SamplerAddressingMode) -> Self {
        Self(value)
    }
}
impl From<SamplerAddressingMode> for SamplerAddressingModeAttr {
    fn from(value: SamplerAddressingMode) -> Self {
        Self(value)
    }
}
impl From<SamplerAddressingModeAttr> for SamplerAddressingMode {
    fn from(value: SamplerAddressingModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.sampler_filter_mode", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct SamplerFilterModeAttr(pub SamplerFilterMode);
impl SamplerFilterModeAttr {
    pub fn new(value: SamplerFilterMode) -> Self {
        Self(value)
    }
}
impl From<SamplerFilterMode> for SamplerFilterModeAttr {
    fn from(value: SamplerFilterMode) -> Self {
        Self(value)
    }
}
impl From<SamplerFilterModeAttr> for SamplerFilterMode {
    fn from(value: SamplerFilterModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.image_format", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ImageFormatAttr(pub ImageFormat);
impl ImageFormatAttr {
    pub fn new(value: ImageFormat) -> Self {
        Self(value)
    }
}
impl From<ImageFormat> for ImageFormatAttr {
    fn from(value: ImageFormat) -> Self {
        Self(value)
    }
}
impl From<ImageFormatAttr> for ImageFormat {
    fn from(value: ImageFormatAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.image_channel_order", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ImageChannelOrderAttr(pub ImageChannelOrder);
impl ImageChannelOrderAttr {
    pub fn new(value: ImageChannelOrder) -> Self {
        Self(value)
    }
}
impl From<ImageChannelOrder> for ImageChannelOrderAttr {
    fn from(value: ImageChannelOrder) -> Self {
        Self(value)
    }
}
impl From<ImageChannelOrderAttr> for ImageChannelOrder {
    fn from(value: ImageChannelOrderAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.image_channel_data_type", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ImageChannelDataTypeAttr(pub ImageChannelDataType);
impl ImageChannelDataTypeAttr {
    pub fn new(value: ImageChannelDataType) -> Self {
        Self(value)
    }
}
impl From<ImageChannelDataType> for ImageChannelDataTypeAttr {
    fn from(value: ImageChannelDataType) -> Self {
        Self(value)
    }
}
impl From<ImageChannelDataTypeAttr> for ImageChannelDataType {
    fn from(value: ImageChannelDataTypeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fp_rounding_mode", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FPRoundingModeAttr(pub FPRoundingMode);
impl FPRoundingModeAttr {
    pub fn new(value: FPRoundingMode) -> Self {
        Self(value)
    }
}
impl From<FPRoundingMode> for FPRoundingModeAttr {
    fn from(value: FPRoundingMode) -> Self {
        Self(value)
    }
}
impl From<FPRoundingModeAttr> for FPRoundingMode {
    fn from(value: FPRoundingModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fp_denorm_mode", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FPDenormModeAttr(pub FPDenormMode);
impl FPDenormModeAttr {
    pub fn new(value: FPDenormMode) -> Self {
        Self(value)
    }
}
impl From<FPDenormMode> for FPDenormModeAttr {
    fn from(value: FPDenormMode) -> Self {
        Self(value)
    }
}
impl From<FPDenormModeAttr> for FPDenormMode {
    fn from(value: FPDenormModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.quantization_modes", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct QuantizationModesAttr(pub QuantizationModes);
impl QuantizationModesAttr {
    pub fn new(value: QuantizationModes) -> Self {
        Self(value)
    }
}
impl From<QuantizationModes> for QuantizationModesAttr {
    fn from(value: QuantizationModes) -> Self {
        Self(value)
    }
}
impl From<QuantizationModesAttr> for QuantizationModes {
    fn from(value: QuantizationModesAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fp_operation_mode", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FPOperationModeAttr(pub FPOperationMode);
impl FPOperationModeAttr {
    pub fn new(value: FPOperationMode) -> Self {
        Self(value)
    }
}
impl From<FPOperationMode> for FPOperationModeAttr {
    fn from(value: FPOperationMode) -> Self {
        Self(value)
    }
}
impl From<FPOperationModeAttr> for FPOperationMode {
    fn from(value: FPOperationModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.overflow_modes", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct OverflowModesAttr(pub OverflowModes);
impl OverflowModesAttr {
    pub fn new(value: OverflowModes) -> Self {
        Self(value)
    }
}
impl From<OverflowModes> for OverflowModesAttr {
    fn from(value: OverflowModes) -> Self {
        Self(value)
    }
}
impl From<OverflowModesAttr> for OverflowModes {
    fn from(value: OverflowModesAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.linkage_type", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct LinkageTypeAttr(pub LinkageType);
impl LinkageTypeAttr {
    pub fn new(value: LinkageType) -> Self {
        Self(value)
    }
}
impl From<LinkageType> for LinkageTypeAttr {
    fn from(value: LinkageType) -> Self {
        Self(value)
    }
}
impl From<LinkageTypeAttr> for LinkageType {
    fn from(value: LinkageTypeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.access_qualifier", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct AccessQualifierAttr(pub AccessQualifier);
impl AccessQualifierAttr {
    pub fn new(value: AccessQualifier) -> Self {
        Self(value)
    }
}
impl From<AccessQualifier> for AccessQualifierAttr {
    fn from(value: AccessQualifier) -> Self {
        Self(value)
    }
}
impl From<AccessQualifierAttr> for AccessQualifier {
    fn from(value: AccessQualifierAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.host_access_qualifier", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct HostAccessQualifierAttr(pub HostAccessQualifier);
impl HostAccessQualifierAttr {
    pub fn new(value: HostAccessQualifier) -> Self {
        Self(value)
    }
}
impl From<HostAccessQualifier> for HostAccessQualifierAttr {
    fn from(value: HostAccessQualifier) -> Self {
        Self(value)
    }
}
impl From<HostAccessQualifierAttr> for HostAccessQualifier {
    fn from(value: HostAccessQualifierAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.function_parameter_attribute", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FunctionParameterAttributeAttr(pub FunctionParameterAttribute);
impl FunctionParameterAttributeAttr {
    pub fn new(value: FunctionParameterAttribute) -> Self {
        Self(value)
    }
}
impl From<FunctionParameterAttribute> for FunctionParameterAttributeAttr {
    fn from(value: FunctionParameterAttribute) -> Self {
        Self(value)
    }
}
impl From<FunctionParameterAttributeAttr> for FunctionParameterAttribute {
    fn from(value: FunctionParameterAttributeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.built_in", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct BuiltInAttr(pub BuiltIn);
impl BuiltInAttr {
    pub fn new(value: BuiltIn) -> Self {
        Self(value)
    }
}
impl From<BuiltIn> for BuiltInAttr {
    fn from(value: BuiltIn) -> Self {
        Self(value)
    }
}
impl From<BuiltInAttr> for BuiltIn {
    fn from(value: BuiltInAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.scope", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ScopeAttr(pub Scope);
impl ScopeAttr {
    pub fn new(value: Scope) -> Self {
        Self(value)
    }
}
impl From<Scope> for ScopeAttr {
    fn from(value: Scope) -> Self {
        Self(value)
    }
}
impl From<ScopeAttr> for Scope {
    fn from(value: ScopeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.group_operation", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct GroupOperationAttr(pub GroupOperation);
impl GroupOperationAttr {
    pub fn new(value: GroupOperation) -> Self {
        Self(value)
    }
}
impl From<GroupOperation> for GroupOperationAttr {
    fn from(value: GroupOperation) -> Self {
        Self(value)
    }
}
impl From<GroupOperationAttr> for GroupOperation {
    fn from(value: GroupOperationAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.kernel_enqueue_flags", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct KernelEnqueueFlagsAttr(pub KernelEnqueueFlags);
impl KernelEnqueueFlagsAttr {
    pub fn new(value: KernelEnqueueFlags) -> Self {
        Self(value)
    }
}
impl From<KernelEnqueueFlags> for KernelEnqueueFlagsAttr {
    fn from(value: KernelEnqueueFlags) -> Self {
        Self(value)
    }
}
impl From<KernelEnqueueFlagsAttr> for KernelEnqueueFlags {
    fn from(value: KernelEnqueueFlagsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.capability", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CapabilityAttr(pub Capability);
impl CapabilityAttr {
    pub fn new(value: Capability) -> Self {
        Self(value)
    }
}
impl From<Capability> for CapabilityAttr {
    fn from(value: Capability) -> Self {
        Self(value)
    }
}
impl From<CapabilityAttr> for Capability {
    fn from(value: CapabilityAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.ray_query_intersection", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct RayQueryIntersectionAttr(pub RayQueryIntersection);
impl RayQueryIntersectionAttr {
    pub fn new(value: RayQueryIntersection) -> Self {
        Self(value)
    }
}
impl From<RayQueryIntersection> for RayQueryIntersectionAttr {
    fn from(value: RayQueryIntersection) -> Self {
        Self(value)
    }
}
impl From<RayQueryIntersectionAttr> for RayQueryIntersection {
    fn from(value: RayQueryIntersectionAttr) -> Self {
        value.0
    }
}
#[pliron_attr(
    name = "spirv.ray_query_committed_intersection_type",
    format = "$0",
    verifier = "succ"
)]
#[derive(PartialEq, Clone, Debug)]
pub struct RayQueryCommittedIntersectionTypeAttr(pub RayQueryCommittedIntersectionType);
impl RayQueryCommittedIntersectionTypeAttr {
    pub fn new(value: RayQueryCommittedIntersectionType) -> Self {
        Self(value)
    }
}
impl From<RayQueryCommittedIntersectionType> for RayQueryCommittedIntersectionTypeAttr {
    fn from(value: RayQueryCommittedIntersectionType) -> Self {
        Self(value)
    }
}
impl From<RayQueryCommittedIntersectionTypeAttr> for RayQueryCommittedIntersectionType {
    fn from(value: RayQueryCommittedIntersectionTypeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(
    name = "spirv.ray_query_candidate_intersection_type",
    format = "$0",
    verifier = "succ"
)]
#[derive(PartialEq, Clone, Debug)]
pub struct RayQueryCandidateIntersectionTypeAttr(pub RayQueryCandidateIntersectionType);
impl RayQueryCandidateIntersectionTypeAttr {
    pub fn new(value: RayQueryCandidateIntersectionType) -> Self {
        Self(value)
    }
}
impl From<RayQueryCandidateIntersectionType> for RayQueryCandidateIntersectionTypeAttr {
    fn from(value: RayQueryCandidateIntersectionType) -> Self {
        Self(value)
    }
}
impl From<RayQueryCandidateIntersectionTypeAttr> for RayQueryCandidateIntersectionType {
    fn from(value: RayQueryCandidateIntersectionTypeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.packed_vector_format", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct PackedVectorFormatAttr(pub PackedVectorFormat);
impl PackedVectorFormatAttr {
    pub fn new(value: PackedVectorFormat) -> Self {
        Self(value)
    }
}
impl From<PackedVectorFormat> for PackedVectorFormatAttr {
    fn from(value: PackedVectorFormat) -> Self {
        Self(value)
    }
}
impl From<PackedVectorFormatAttr> for PackedVectorFormat {
    fn from(value: PackedVectorFormatAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.cooperative_matrix_operands", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CooperativeMatrixOperandsAttr(pub CooperativeMatrixOperands);
impl CooperativeMatrixOperandsAttr {
    pub fn new(value: CooperativeMatrixOperands) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixOperands> for CooperativeMatrixOperandsAttr {
    fn from(value: CooperativeMatrixOperands) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixOperandsAttr> for CooperativeMatrixOperands {
    fn from(value: CooperativeMatrixOperandsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.cooperative_matrix_layout", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CooperativeMatrixLayoutAttr(pub CooperativeMatrixLayout);
impl CooperativeMatrixLayoutAttr {
    pub fn new(value: CooperativeMatrixLayout) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixLayout> for CooperativeMatrixLayoutAttr {
    fn from(value: CooperativeMatrixLayout) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixLayoutAttr> for CooperativeMatrixLayout {
    fn from(value: CooperativeMatrixLayoutAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.cooperative_matrix_use", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CooperativeMatrixUseAttr(pub CooperativeMatrixUse);
impl CooperativeMatrixUseAttr {
    pub fn new(value: CooperativeMatrixUse) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixUse> for CooperativeMatrixUseAttr {
    fn from(value: CooperativeMatrixUse) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixUseAttr> for CooperativeMatrixUse {
    fn from(value: CooperativeMatrixUseAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.cooperative_matrix_reduce", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CooperativeMatrixReduceAttr(pub CooperativeMatrixReduce);
impl CooperativeMatrixReduceAttr {
    pub fn new(value: CooperativeMatrixReduce) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixReduce> for CooperativeMatrixReduceAttr {
    fn from(value: CooperativeMatrixReduce) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixReduceAttr> for CooperativeMatrixReduce {
    fn from(value: CooperativeMatrixReduceAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.tensor_clamp_mode", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct TensorClampModeAttr(pub TensorClampMode);
impl TensorClampModeAttr {
    pub fn new(value: TensorClampMode) -> Self {
        Self(value)
    }
}
impl From<TensorClampMode> for TensorClampModeAttr {
    fn from(value: TensorClampMode) -> Self {
        Self(value)
    }
}
impl From<TensorClampModeAttr> for TensorClampMode {
    fn from(value: TensorClampModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.tensor_addressing_operands", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct TensorAddressingOperandsAttr(pub TensorAddressingOperands);
impl TensorAddressingOperandsAttr {
    pub fn new(value: TensorAddressingOperands) -> Self {
        Self(value)
    }
}
impl From<TensorAddressingOperands> for TensorAddressingOperandsAttr {
    fn from(value: TensorAddressingOperands) -> Self {
        Self(value)
    }
}
impl From<TensorAddressingOperandsAttr> for TensorAddressingOperands {
    fn from(value: TensorAddressingOperandsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.initialization_mode_qualifier", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct InitializationModeQualifierAttr(pub InitializationModeQualifier);
impl InitializationModeQualifierAttr {
    pub fn new(value: InitializationModeQualifier) -> Self {
        Self(value)
    }
}
impl From<InitializationModeQualifier> for InitializationModeQualifierAttr {
    fn from(value: InitializationModeQualifier) -> Self {
        Self(value)
    }
}
impl From<InitializationModeQualifierAttr> for InitializationModeQualifier {
    fn from(value: InitializationModeQualifierAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.load_cache_control", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct LoadCacheControlAttr(pub LoadCacheControl);
impl LoadCacheControlAttr {
    pub fn new(value: LoadCacheControl) -> Self {
        Self(value)
    }
}
impl From<LoadCacheControl> for LoadCacheControlAttr {
    fn from(value: LoadCacheControl) -> Self {
        Self(value)
    }
}
impl From<LoadCacheControlAttr> for LoadCacheControl {
    fn from(value: LoadCacheControlAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.store_cache_control", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct StoreCacheControlAttr(pub StoreCacheControl);
impl StoreCacheControlAttr {
    pub fn new(value: StoreCacheControl) -> Self {
        Self(value)
    }
}
impl From<StoreCacheControl> for StoreCacheControlAttr {
    fn from(value: StoreCacheControl) -> Self {
        Self(value)
    }
}
impl From<StoreCacheControlAttr> for StoreCacheControl {
    fn from(value: StoreCacheControlAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.named_maximum_number_of_registers", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct NamedMaximumNumberOfRegistersAttr(pub NamedMaximumNumberOfRegisters);
impl NamedMaximumNumberOfRegistersAttr {
    pub fn new(value: NamedMaximumNumberOfRegisters) -> Self {
        Self(value)
    }
}
impl From<NamedMaximumNumberOfRegisters> for NamedMaximumNumberOfRegistersAttr {
    fn from(value: NamedMaximumNumberOfRegisters) -> Self {
        Self(value)
    }
}
impl From<NamedMaximumNumberOfRegistersAttr> for NamedMaximumNumberOfRegisters {
    fn from(value: NamedMaximumNumberOfRegistersAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.matrix_multiply_accumulate_operands", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct MatrixMultiplyAccumulateOperandsAttr(pub MatrixMultiplyAccumulateOperands);
impl MatrixMultiplyAccumulateOperandsAttr {
    pub fn new(value: MatrixMultiplyAccumulateOperands) -> Self {
        Self(value)
    }
}
impl From<MatrixMultiplyAccumulateOperands> for MatrixMultiplyAccumulateOperandsAttr {
    fn from(value: MatrixMultiplyAccumulateOperands) -> Self {
        Self(value)
    }
}
impl From<MatrixMultiplyAccumulateOperandsAttr> for MatrixMultiplyAccumulateOperands {
    fn from(value: MatrixMultiplyAccumulateOperandsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fp_encoding", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FPEncodingAttr(pub FPEncoding);
impl FPEncodingAttr {
    pub fn new(value: FPEncoding) -> Self {
        Self(value)
    }
}
impl From<FPEncoding> for FPEncodingAttr {
    fn from(value: FPEncoding) -> Self {
        Self(value)
    }
}
impl From<FPEncodingAttr> for FPEncoding {
    fn from(value: FPEncodingAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.cooperative_vector_matrix_layout", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CooperativeVectorMatrixLayoutAttr(pub CooperativeVectorMatrixLayout);
impl CooperativeVectorMatrixLayoutAttr {
    pub fn new(value: CooperativeVectorMatrixLayout) -> Self {
        Self(value)
    }
}
impl From<CooperativeVectorMatrixLayout> for CooperativeVectorMatrixLayoutAttr {
    fn from(value: CooperativeVectorMatrixLayout) -> Self {
        Self(value)
    }
}
impl From<CooperativeVectorMatrixLayoutAttr> for CooperativeVectorMatrixLayout {
    fn from(value: CooperativeVectorMatrixLayoutAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.component_type", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ComponentTypeAttr(pub ComponentType);
impl ComponentTypeAttr {
    pub fn new(value: ComponentType) -> Self {
        Self(value)
    }
}
impl From<ComponentType> for ComponentTypeAttr {
    fn from(value: ComponentType) -> Self {
        Self(value)
    }
}
impl From<ComponentTypeAttr> for ComponentType {
    fn from(value: ComponentTypeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.tensor_operands", format = "$0", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct TensorOperandsAttr(pub TensorOperands);
impl TensorOperandsAttr {
    pub fn new(value: TensorOperands) -> Self {
        Self(value)
    }
}
impl From<TensorOperands> for TensorOperandsAttr {
    fn from(value: TensorOperands) -> Self {
        Self(value)
    }
}
impl From<TensorOperandsAttr> for TensorOperands {
    fn from(value: TensorOperandsAttr) -> Self {
        value.0
    }
}
