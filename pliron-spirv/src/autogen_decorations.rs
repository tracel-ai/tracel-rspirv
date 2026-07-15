// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use crate::{
    prelude::*, decorations::{DecorationInfo, DecorationExt, all_decorations_for_op},
};
use core::cell::Ref;
pub static ATTR_RELAXED_PRECISION: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "relaxed_precision".try_into().unwrap());
pub static ATTR_BLOCK: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "block".try_into().unwrap());
pub static ATTR_BUFFER_BLOCK: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "buffer_block".try_into().unwrap());
pub static ATTR_ROW_MAJOR: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "row_major".try_into().unwrap());
pub static ATTR_COL_MAJOR: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "col_major".try_into().unwrap());
pub static ATTR_ARRAY_STRIDE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "array_stride".try_into().unwrap());
pub static ATTR_MATRIX_STRIDE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "matrix_stride".try_into().unwrap());
pub static ATTR_GLSL_SHARED: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "glsl_shared".try_into().unwrap());
pub static ATTR_GLSL_PACKED: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "glsl_packed".try_into().unwrap());
pub static ATTR_C_PACKED: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "c_packed".try_into().unwrap());
pub static ATTR_BUILT_IN: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "built_in".try_into().unwrap());
pub static ATTR_NO_PERSPECTIVE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "no_perspective".try_into().unwrap());
pub static ATTR_FLAT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "flat".try_into().unwrap());
pub static ATTR_PATCH: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "patch".try_into().unwrap());
pub static ATTR_CENTROID: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "centroid".try_into().unwrap());
pub static ATTR_SAMPLE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "sample".try_into().unwrap());
pub static ATTR_INVARIANT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "invariant".try_into().unwrap());
pub static ATTR_RESTRICT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "restrict".try_into().unwrap());
pub static ATTR_ALIASED: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "aliased".try_into().unwrap());
pub static ATTR_VOLATILE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "volatile".try_into().unwrap());
pub static ATTR_CONSTANT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "constant".try_into().unwrap());
pub static ATTR_COHERENT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "coherent".try_into().unwrap());
pub static ATTR_NON_WRITABLE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "non_writable".try_into().unwrap());
pub static ATTR_NON_READABLE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "non_readable".try_into().unwrap());
pub static ATTR_UNIFORM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "uniform".try_into().unwrap());
pub static ATTR_SATURATED_CONVERSION: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "saturated_conversion".try_into().unwrap()
});
pub static ATTR_STREAM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "stream".try_into().unwrap());
pub static ATTR_LOCATION: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "location".try_into().unwrap());
pub static ATTR_COMPONENT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "component".try_into().unwrap());
pub static ATTR_INDEX: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "index".try_into().unwrap());
pub static ATTR_BINDING: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "binding".try_into().unwrap());
pub static ATTR_DESCRIPTOR_SET: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "descriptor_set".try_into().unwrap());
pub static ATTR_OFFSET: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "offset".try_into().unwrap());
pub static ATTR_XFB_BUFFER: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "xfb_buffer".try_into().unwrap());
pub static ATTR_XFB_STRIDE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "xfb_stride".try_into().unwrap());
pub static ATTR_FUNC_PARAM_ATTR: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "func_param_attr".try_into().unwrap());
pub static ATTR_FP_ROUNDING_MODE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "fp_rounding_mode".try_into().unwrap());
pub static ATTR_FP_FAST_MATH_MODE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "fp_fast_math_mode".try_into().unwrap());
pub static ATTR_NO_CONTRACTION: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "no_contraction".try_into().unwrap());
pub static ATTR_INPUT_ATTACHMENT_INDEX: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "input_attachment_index".try_into().unwrap()
});
pub static ATTR_ALIGNMENT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "alignment".try_into().unwrap());
pub static ATTR_MAX_BYTE_OFFSET: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "max_byte_offset".try_into().unwrap());
pub static ATTR_SATURATED_TO_LARGEST_FLOAT8_NORMAL_CONVERSION_EXT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "saturated_to_largest_float8_normal_conversion_ext".try_into().unwrap()
});
pub static ATTR_NO_SIGNED_WRAP: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "no_signed_wrap".try_into().unwrap());
pub static ATTR_NO_UNSIGNED_WRAP: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "no_unsigned_wrap".try_into().unwrap());
pub static ATTR_WEIGHT_TEXTURE_QCOM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "weight_texture_qcom".try_into().unwrap()
});
pub static ATTR_BLOCK_MATCH_TEXTURE_QCOM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "block_match_texture_qcom".try_into().unwrap()
});
pub static ATTR_BLOCK_MATCH_SAMPLER_QCOM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "block_match_sampler_qcom".try_into().unwrap()
});
pub static ATTR_EXPLICIT_INTERP_AMD: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "explicit_interp_amd".try_into().unwrap()
});
pub static ATTR_TRACK_FINISH_WRITING_AMDX: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "track_finish_writing_amdx".try_into().unwrap()
});
pub static ATTR_PAYLOAD_NODE_SPARSE_ARRAY_AMDX: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "payload_node_sparse_array_amdx".try_into().unwrap()
});
pub static ATTR_PAYLOAD_DISPATCH_INDIRECT_AMDX: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "payload_dispatch_indirect_amdx".try_into().unwrap()
});
pub static ATTR_UTF_ENCODED_KHR: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "utf_encoded_khr".try_into().unwrap());
pub static ATTR_OVERRIDE_COVERAGE_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "override_coverage_nv".try_into().unwrap()
});
pub static ATTR_PASSTHROUGH_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "passthrough_nv".try_into().unwrap());
pub static ATTR_VIEWPORT_RELATIVE_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "viewport_relative_nv".try_into().unwrap()
});
pub static ATTR_SECONDARY_VIEWPORT_RELATIVE_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "secondary_viewport_relative_nv".try_into().unwrap()
});
pub static ATTR_PER_PRIMITIVE_EXT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "per_primitive_ext".try_into().unwrap());
pub static ATTR_PER_VIEW_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "per_view_nv".try_into().unwrap());
pub static ATTR_PER_TASK_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "per_task_nv".try_into().unwrap());
pub static ATTR_PER_VERTEX_KHR: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "per_vertex_khr".try_into().unwrap());
pub static ATTR_NON_UNIFORM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "non_uniform".try_into().unwrap());
pub static ATTR_RESTRICT_POINTER: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "restrict_pointer".try_into().unwrap());
pub static ATTR_ALIASED_POINTER: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "aliased_pointer".try_into().unwrap());
pub static ATTR_MEMBER_OFFSET_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "member_offset_nv".try_into().unwrap());
pub static ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "hit_object_shader_record_buffer_nv".try_into().unwrap()
});
pub static ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_EXT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "hit_object_shader_record_buffer_ext".try_into().unwrap()
});
pub static ATTR_BANK_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "bank_nv".try_into().unwrap());
pub static ATTR_BINDLESS_SAMPLER_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "bindless_sampler_nv".try_into().unwrap()
});
pub static ATTR_BINDLESS_IMAGE_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "bindless_image_nv".try_into().unwrap());
pub static ATTR_BOUND_SAMPLER_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "bound_sampler_nv".try_into().unwrap());
pub static ATTR_BOUND_IMAGE_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "bound_image_nv".try_into().unwrap());
pub static ATTR_SIMT_CALL_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "simt_call_intel".try_into().unwrap());
pub static ATTR_REFERENCED_INDIRECTLY_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "referenced_indirectly_intel".try_into().unwrap()
});
pub static ATTR_CLOBBER_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "clobber_intel".try_into().unwrap());
pub static ATTR_SIDE_EFFECTS_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "side_effects_intel".try_into().unwrap());
pub static ATTR_VECTOR_COMPUTE_VARIABLE_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "vector_compute_variable_intel".try_into().unwrap()
});
pub static ATTR_FUNC_PARAM_IO_KIND_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "func_param_io_kind_intel".try_into().unwrap()
});
pub static ATTR_VECTOR_COMPUTE_FUNCTION_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "vector_compute_function_intel".try_into().unwrap()
});
pub static ATTR_STACK_CALL_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "stack_call_intel".try_into().unwrap());
pub static ATTR_GLOBAL_VARIABLE_OFFSET_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "global_variable_offset_intel".try_into().unwrap()
});
pub static ATTR_USER_SEMANTIC: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "user_semantic".try_into().unwrap());
pub static ATTR_USER_TYPE_GOOGLE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "user_type_google".try_into().unwrap());
pub static ATTR_REGISTER_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "register_altera".try_into().unwrap());
pub static ATTR_MEMORY_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "memory_altera".try_into().unwrap());
pub static ATTR_NUMBANKS_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "numbanks_altera".try_into().unwrap());
pub static ATTR_BANKWIDTH_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "bankwidth_altera".try_into().unwrap());
pub static ATTR_MAX_PRIVATE_COPIES_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "max_private_copies_altera".try_into().unwrap()
});
pub static ATTR_SINGLEPUMP_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "singlepump_altera".try_into().unwrap());
pub static ATTR_DOUBLEPUMP_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "doublepump_altera".try_into().unwrap());
pub static ATTR_MAX_REPLICATES_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "max_replicates_altera".try_into().unwrap()
});
pub static ATTR_SIMPLE_DUAL_PORT_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "simple_dual_port_altera".try_into().unwrap()
});
pub static ATTR_FORCE_POW2_DEPTH_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "force_pow2_depth_altera".try_into().unwrap()
});
pub static ATTR_STRIDESIZE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "stridesize_altera".try_into().unwrap());
pub static ATTR_WORDSIZE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "wordsize_altera".try_into().unwrap());
pub static ATTR_TRUE_DUAL_PORT_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "true_dual_port_altera".try_into().unwrap()
});
pub static ATTR_BURST_COALESCE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "burst_coalesce_altera".try_into().unwrap()
});
pub static ATTR_CACHE_SIZE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "cache_size_altera".try_into().unwrap());
pub static ATTR_DONT_STATICALLY_COALESCE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "dont_statically_coalesce_altera".try_into().unwrap()
});
pub static ATTR_PREFETCH_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "prefetch_altera".try_into().unwrap());
pub static ATTR_STALL_ENABLE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "stall_enable_altera".try_into().unwrap()
});
pub static ATTR_FUSE_LOOPS_IN_FUNCTION_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "fuse_loops_in_function_altera".try_into().unwrap()
});
pub static ATTR_INITIATION_INTERVAL_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "initiation_interval_altera".try_into().unwrap()
});
pub static ATTR_MAX_CONCURRENCY_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "max_concurrency_altera".try_into().unwrap()
});
pub static ATTR_PIPELINE_ENABLE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "pipeline_enable_altera".try_into().unwrap()
});
pub static ATTR_BUFFER_LOCATION_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "buffer_location_altera".try_into().unwrap()
});
pub static ATTR_IO_PIPE_STORAGE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "io_pipe_storage_altera".try_into().unwrap()
});
pub static ATTR_SINGLE_ELEMENT_VECTOR_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "single_element_vector_intel".try_into().unwrap()
});
pub static ATTR_VECTOR_COMPUTE_CALLABLE_FUNCTION_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "vector_compute_callable_function_intel".try_into().unwrap()
});
pub static ATTR_MEDIA_BLOCK_IOINTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "media_block_iointel".try_into().unwrap()
});
pub static ATTR_STALL_FREE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| "stall_free_altera".try_into().unwrap());
pub static ATTR_LATENCY_CONTROL_LABEL_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "latency_control_label_altera".try_into().unwrap()
});
pub static ATTR_CONDUIT_KERNEL_ARGUMENT_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "conduit_kernel_argument_altera".try_into().unwrap()
});
pub static ATTR_REGISTER_MAP_KERNEL_ARGUMENT_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "register_map_kernel_argument_altera".try_into().unwrap()
});
pub static ATTR_MM_HOST_INTERFACE_ADDRESS_WIDTH_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "mm_host_interface_address_width_altera".try_into().unwrap()
});
pub static ATTR_MM_HOST_INTERFACE_DATA_WIDTH_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "mm_host_interface_data_width_altera".try_into().unwrap()
});
pub static ATTR_MM_HOST_INTERFACE_LATENCY_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "mm_host_interface_latency_altera".try_into().unwrap()
});
pub static ATTR_MM_HOST_INTERFACE_MAX_BURST_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "mm_host_interface_max_burst_altera".try_into().unwrap()
});
pub static ATTR_MM_HOST_INTERFACE_WAIT_REQUEST_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "mm_host_interface_wait_request_altera".try_into().unwrap()
});
pub static ATTR_STABLE_KERNEL_ARGUMENT_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "stable_kernel_argument_altera".try_into().unwrap()
});
pub static ATTR_IMPLEMENT_IN_REGISTER_MAP_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "implement_in_register_map_altera".try_into().unwrap()
});
#[op_interface]
pub trait DecoratableOp {
    fn verify(_op: &dyn Op, _ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }
    #[allow(non_snake_case)]
    fn get_decoration_relaxed_precision<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_relaxed_precision(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_block<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_block(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_buffer_block<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_buffer_block(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_row_major<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_row_major(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_col_major<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_col_major(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_array_stride<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_array_stride(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_matrix_stride<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_matrix_stride(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_glsl_shared<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_glsl_shared(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_glsl_packed<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_glsl_packed(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_c_packed<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_c_packed(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_built_in<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, BuiltInAttr>>
    where
        Self: Sized,
    {
        get_decoration_built_in(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_no_perspective<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_no_perspective(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_flat<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_flat(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_patch<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_patch(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_centroid<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_centroid(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_sample<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_sample(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_invariant<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_invariant(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_restrict<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_restrict(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_aliased<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_aliased(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_volatile<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_volatile(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_constant<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_constant(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_coherent<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_coherent(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_non_writable<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_non_writable(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_non_readable<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_non_readable(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_uniform<'a>(&self, ctx: &'a Context) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_uniform(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_saturated_conversion<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_saturated_conversion(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_stream<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_stream(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_location<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_location(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_component<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_component(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_index<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_index(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_binding<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_binding(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_descriptor_set<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_descriptor_set(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_offset<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_offset(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_xfb_buffer<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_xfb_buffer(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_xfb_stride<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_xfb_stride(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_func_param_attr<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, VecAttr>>
    where
        Self: Sized,
    {
        get_decoration_func_param_attr(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_fp_rounding_mode<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, FPRoundingModeAttr>>
    where
        Self: Sized,
    {
        get_decoration_fp_rounding_mode(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_fp_fast_math_mode<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, FPFastMathModeAttr>>
    where
        Self: Sized,
    {
        get_decoration_fp_fast_math_mode(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_no_contraction<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_no_contraction(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_input_attachment_index<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_input_attachment_index(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_alignment<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_alignment(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_max_byte_offset<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_max_byte_offset(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_saturated_to_largest_float8_normal_conversion_ext<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_saturated_to_largest_float8_normal_conversion_ext(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_no_signed_wrap<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_no_signed_wrap(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_no_unsigned_wrap<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_no_unsigned_wrap(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_weight_texture_qcom<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_weight_texture_qcom(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_block_match_texture_qcom<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_block_match_texture_qcom(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_block_match_sampler_qcom<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_block_match_sampler_qcom(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_explicit_interp_amd<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_explicit_interp_amd(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_track_finish_writing_amdx<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_track_finish_writing_amdx(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_payload_node_sparse_array_amdx<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_payload_node_sparse_array_amdx(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_payload_dispatch_indirect_amdx<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_payload_dispatch_indirect_amdx(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_utf_encoded_khr<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_utf_encoded_khr(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_override_coverage_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_override_coverage_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_passthrough_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_passthrough_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_viewport_relative_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_viewport_relative_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_secondary_viewport_relative_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_secondary_viewport_relative_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_per_primitive_ext<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_per_primitive_ext(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_per_view_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_per_view_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_per_task_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_per_task_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_per_vertex_khr<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_per_vertex_khr(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_non_uniform<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_non_uniform(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_restrict_pointer<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_restrict_pointer(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_aliased_pointer<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_aliased_pointer(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_member_offset_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_member_offset_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_hit_object_shader_record_buffer_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_hit_object_shader_record_buffer_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_hit_object_shader_record_buffer_ext<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_hit_object_shader_record_buffer_ext(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_bank_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_bank_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_bindless_sampler_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_bindless_sampler_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_bindless_image_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_bindless_image_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_bound_sampler_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_bound_sampler_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_bound_image_nv<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_bound_image_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_simt_call_intel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_simt_call_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_referenced_indirectly_intel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_referenced_indirectly_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_clobber_intel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, StringAttr>>
    where
        Self: Sized,
    {
        get_decoration_clobber_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_side_effects_intel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_side_effects_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_vector_compute_variable_intel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_vector_compute_variable_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_func_param_io_kind_intel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_func_param_io_kind_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_vector_compute_function_intel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_vector_compute_function_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_stack_call_intel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_stack_call_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_global_variable_offset_intel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_global_variable_offset_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_user_semantic<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, StringAttr>>
    where
        Self: Sized,
    {
        get_decoration_user_semantic(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_user_type_google<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, StringAttr>>
    where
        Self: Sized,
    {
        get_decoration_user_type_google(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_register_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_register_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_memory_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, StringAttr>>
    where
        Self: Sized,
    {
        get_decoration_memory_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_numbanks_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_numbanks_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_bankwidth_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_bankwidth_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_max_private_copies_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_max_private_copies_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_singlepump_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_singlepump_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_doublepump_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_doublepump_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_max_replicates_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_max_replicates_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_simple_dual_port_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_simple_dual_port_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_force_pow2_depth_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_force_pow2_depth_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_stridesize_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_stridesize_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_wordsize_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_wordsize_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_true_dual_port_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_true_dual_port_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_burst_coalesce_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_burst_coalesce_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_cache_size_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_cache_size_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_dont_statically_coalesce_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_dont_statically_coalesce_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_prefetch_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_prefetch_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_stall_enable_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_stall_enable_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_fuse_loops_in_function_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_fuse_loops_in_function_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_initiation_interval_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_initiation_interval_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_max_concurrency_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_max_concurrency_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_pipeline_enable_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_pipeline_enable_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_buffer_location_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_buffer_location_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_io_pipe_storage_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_io_pipe_storage_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_single_element_vector_intel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_single_element_vector_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_vector_compute_callable_function_intel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_vector_compute_callable_function_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_media_block_iointel<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_media_block_iointel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_stall_free_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_stall_free_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_latency_control_label_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_latency_control_label_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_conduit_kernel_argument_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_conduit_kernel_argument_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_register_map_kernel_argument_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_register_map_kernel_argument_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_mm_host_interface_address_width_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_mm_host_interface_address_width_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_mm_host_interface_data_width_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_mm_host_interface_data_width_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_mm_host_interface_latency_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_mm_host_interface_latency_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_mm_host_interface_max_burst_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_mm_host_interface_max_burst_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_mm_host_interface_wait_request_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_mm_host_interface_wait_request_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_stable_kernel_argument_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, UnitAttr>>
    where
        Self: Sized,
    {
        get_decoration_stable_kernel_argument_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn get_decoration_implement_in_register_map_altera<'a>(
        &self,
        ctx: &'a Context,
    ) -> Option<Ref<'a, LiteralIntegerAttr>>
    where
        Self: Sized,
    {
        get_decoration_implement_in_register_map_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn set_decoration_relaxed_precision(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_relaxed_precision(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_block(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_block(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_buffer_block(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_buffer_block(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_row_major(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_row_major(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_col_major(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_col_major(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_array_stride(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_array_stride(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_matrix_stride(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_matrix_stride(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_glsl_shared(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_glsl_shared(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_glsl_packed(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_glsl_packed(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_c_packed(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_c_packed(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_built_in(&self, ctx: &Context, value: BuiltInAttr)
    where
        Self: Sized,
    {
        set_decoration_built_in(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_no_perspective(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_no_perspective(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_flat(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_flat(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_patch(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_patch(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_centroid(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_centroid(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_sample(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_sample(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_invariant(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_invariant(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_restrict(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_restrict(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_aliased(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_aliased(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_volatile(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_volatile(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_constant(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_constant(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_coherent(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_coherent(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_non_writable(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_non_writable(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_non_readable(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_non_readable(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_uniform(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_uniform(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_saturated_conversion(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_saturated_conversion(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_stream(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_stream(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_location(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_location(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_component(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_component(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_index(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_index(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_binding(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_binding(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_descriptor_set(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_descriptor_set(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_offset(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_offset(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_xfb_buffer(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_xfb_buffer(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_xfb_stride(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_xfb_stride(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_func_param_attr(&self, ctx: &Context, value: VecAttr)
    where
        Self: Sized,
    {
        set_decoration_func_param_attr(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_fp_rounding_mode(&self, ctx: &Context, value: FPRoundingModeAttr)
    where
        Self: Sized,
    {
        set_decoration_fp_rounding_mode(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_fp_fast_math_mode(&self, ctx: &Context, value: FPFastMathModeAttr)
    where
        Self: Sized,
    {
        set_decoration_fp_fast_math_mode(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_no_contraction(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_no_contraction(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_input_attachment_index(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_input_attachment_index(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_alignment(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_alignment(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_max_byte_offset(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_max_byte_offset(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_saturated_to_largest_float8_normal_conversion_ext(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_saturated_to_largest_float8_normal_conversion_ext(
            self,
            ctx,
            value,
        );
    }
    #[allow(non_snake_case)]
    fn set_decoration_no_signed_wrap(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_no_signed_wrap(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_no_unsigned_wrap(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_no_unsigned_wrap(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_weight_texture_qcom(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_weight_texture_qcom(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_block_match_texture_qcom(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_block_match_texture_qcom(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_block_match_sampler_qcom(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_block_match_sampler_qcom(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_explicit_interp_amd(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_explicit_interp_amd(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_track_finish_writing_amdx(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_track_finish_writing_amdx(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_payload_node_sparse_array_amdx(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_payload_node_sparse_array_amdx(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_payload_dispatch_indirect_amdx(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_payload_dispatch_indirect_amdx(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_utf_encoded_khr(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_utf_encoded_khr(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_override_coverage_nv(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_override_coverage_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_passthrough_nv(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_passthrough_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_viewport_relative_nv(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_viewport_relative_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_secondary_viewport_relative_nv(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_secondary_viewport_relative_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_per_primitive_ext(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_per_primitive_ext(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_per_view_nv(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_per_view_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_per_task_nv(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_per_task_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_per_vertex_khr(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_per_vertex_khr(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_non_uniform(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_non_uniform(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_restrict_pointer(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_restrict_pointer(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_aliased_pointer(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_aliased_pointer(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_member_offset_nv(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_member_offset_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_hit_object_shader_record_buffer_nv(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_hit_object_shader_record_buffer_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_hit_object_shader_record_buffer_ext(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_hit_object_shader_record_buffer_ext(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_bank_nv(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_bank_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_bindless_sampler_nv(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_bindless_sampler_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_bindless_image_nv(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_bindless_image_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_bound_sampler_nv(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_bound_sampler_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_bound_image_nv(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_bound_image_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_simt_call_intel(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_simt_call_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_referenced_indirectly_intel(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_referenced_indirectly_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_clobber_intel(&self, ctx: &Context, value: StringAttr)
    where
        Self: Sized,
    {
        set_decoration_clobber_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_side_effects_intel(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_side_effects_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_vector_compute_variable_intel(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_vector_compute_variable_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_func_param_io_kind_intel(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_func_param_io_kind_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_vector_compute_function_intel(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_vector_compute_function_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_stack_call_intel(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_stack_call_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_global_variable_offset_intel(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_global_variable_offset_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_user_semantic(&self, ctx: &Context, value: StringAttr)
    where
        Self: Sized,
    {
        set_decoration_user_semantic(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_user_type_google(&self, ctx: &Context, value: StringAttr)
    where
        Self: Sized,
    {
        set_decoration_user_type_google(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_register_altera(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_register_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_memory_altera(&self, ctx: &Context, value: StringAttr)
    where
        Self: Sized,
    {
        set_decoration_memory_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_numbanks_altera(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_numbanks_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_bankwidth_altera(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_bankwidth_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_max_private_copies_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_max_private_copies_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_singlepump_altera(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_singlepump_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_doublepump_altera(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_doublepump_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_max_replicates_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_max_replicates_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_simple_dual_port_altera(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_simple_dual_port_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_force_pow2_depth_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_force_pow2_depth_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_stridesize_altera(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_stridesize_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_wordsize_altera(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_wordsize_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_true_dual_port_altera(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_true_dual_port_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_burst_coalesce_altera(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_burst_coalesce_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_cache_size_altera(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_cache_size_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_dont_statically_coalesce_altera(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_dont_statically_coalesce_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_prefetch_altera(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_prefetch_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_stall_enable_altera(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_stall_enable_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_fuse_loops_in_function_altera(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_fuse_loops_in_function_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_initiation_interval_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_initiation_interval_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_max_concurrency_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_max_concurrency_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_pipeline_enable_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_pipeline_enable_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_buffer_location_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_buffer_location_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_io_pipe_storage_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_io_pipe_storage_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_single_element_vector_intel(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_single_element_vector_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_vector_compute_callable_function_intel(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_vector_compute_callable_function_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_media_block_iointel(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_media_block_iointel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_stall_free_altera(&self, ctx: &Context, value: UnitAttr)
    where
        Self: Sized,
    {
        set_decoration_stall_free_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_latency_control_label_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_latency_control_label_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_conduit_kernel_argument_altera(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_conduit_kernel_argument_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_register_map_kernel_argument_altera(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_register_map_kernel_argument_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_mm_host_interface_address_width_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_mm_host_interface_address_width_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_mm_host_interface_data_width_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_mm_host_interface_data_width_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_mm_host_interface_latency_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_mm_host_interface_latency_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_mm_host_interface_max_burst_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_mm_host_interface_max_burst_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_mm_host_interface_wait_request_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_mm_host_interface_wait_request_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_stable_kernel_argument_altera(
        &self,
        ctx: &Context,
        value: UnitAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_stable_kernel_argument_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_implement_in_register_map_altera(
        &self,
        ctx: &Context,
        value: LiteralIntegerAttr,
    )
    where
        Self: Sized,
    {
        set_decoration_implement_in_register_map_altera(self, ctx, value);
    }
    fn all_decorations(&self, ctx: &Context) -> Vec<(Decoration, Vec<Operand>)>
    where
        Self: Sized,
    {
        all_decorations_for_op(self, ctx)
    }
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_relaxed_precision<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_RELAXED_PRECISION) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_block<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_BLOCK) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_buffer_block<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_BUFFER_BLOCK) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_row_major<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_ROW_MAJOR) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_col_major<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_COL_MAJOR) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_array_stride<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_ARRAY_STRIDE) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_matrix_stride<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_MATRIX_STRIDE) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_glsl_shared<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_GLSL_SHARED) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_glsl_packed<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_GLSL_PACKED) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_c_packed<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_C_PACKED) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_built_in<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, BuiltInAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<BuiltInAttr>(&ATTR_BUILT_IN) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_no_perspective<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_NO_PERSPECTIVE) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_flat<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_FLAT) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_patch<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_PATCH) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_centroid<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_CENTROID) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_sample<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_SAMPLE) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_invariant<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_INVARIANT) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_restrict<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_RESTRICT) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_aliased<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_ALIASED) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_volatile<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_VOLATILE) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_constant<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_CONSTANT) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_coherent<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_COHERENT) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_non_writable<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_NON_WRITABLE) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_non_readable<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_NON_READABLE) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_uniform<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_UNIFORM) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_saturated_conversion<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_SATURATED_CONVERSION) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_stream<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_STREAM) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_location<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_LOCATION) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_component<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_COMPONENT) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_index<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_INDEX) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_binding<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_BINDING) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_descriptor_set<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_DESCRIPTOR_SET) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_offset<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_OFFSET) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_xfb_buffer<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_XFB_BUFFER) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_xfb_stride<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_XFB_STRIDE) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_func_param_attr<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, VecAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<VecAttr>(&ATTR_FUNC_PARAM_ATTR) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_fp_rounding_mode<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, FPRoundingModeAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<FPRoundingModeAttr>(&ATTR_FP_ROUNDING_MODE) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_fp_fast_math_mode<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, FPFastMathModeAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<FPFastMathModeAttr>(&ATTR_FP_FAST_MATH_MODE) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_no_contraction<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_NO_CONTRACTION) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_input_attachment_index<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<LiteralIntegerAttr>(&ATTR_INPUT_ATTACHMENT_INDEX)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_alignment<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_ALIGNMENT) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_max_byte_offset<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_MAX_BYTE_OFFSET) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_saturated_to_largest_float8_normal_conversion_ext<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes
                    .get::<
                        UnitAttr,
                    >(&ATTR_SATURATED_TO_LARGEST_FLOAT8_NORMAL_CONVERSION_EXT)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_no_signed_wrap<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_NO_SIGNED_WRAP) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_no_unsigned_wrap<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_NO_UNSIGNED_WRAP) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_weight_texture_qcom<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_WEIGHT_TEXTURE_QCOM) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_block_match_texture_qcom<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_BLOCK_MATCH_TEXTURE_QCOM) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_block_match_sampler_qcom<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_BLOCK_MATCH_SAMPLER_QCOM) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_explicit_interp_amd<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_EXPLICIT_INTERP_AMD) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_track_finish_writing_amdx<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_TRACK_FINISH_WRITING_AMDX) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_payload_node_sparse_array_amdx<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_PAYLOAD_NODE_SPARSE_ARRAY_AMDX) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_payload_dispatch_indirect_amdx<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_PAYLOAD_DISPATCH_INDIRECT_AMDX) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_utf_encoded_khr<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_UTF_ENCODED_KHR) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_override_coverage_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_OVERRIDE_COVERAGE_NV) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_passthrough_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_PASSTHROUGH_NV) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_viewport_relative_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_VIEWPORT_RELATIVE_NV) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_secondary_viewport_relative_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes
                    .get::<LiteralIntegerAttr>(&ATTR_SECONDARY_VIEWPORT_RELATIVE_NV)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_per_primitive_ext<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_PER_PRIMITIVE_EXT) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_per_view_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_PER_VIEW_NV) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_per_task_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_PER_TASK_NV) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_per_vertex_khr<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_PER_VERTEX_KHR) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_non_uniform<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_NON_UNIFORM) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_restrict_pointer<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_RESTRICT_POINTER) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_aliased_pointer<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_ALIASED_POINTER) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_member_offset_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_MEMBER_OFFSET_NV) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_hit_object_shader_record_buffer_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<UnitAttr>(&ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_NV)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_hit_object_shader_record_buffer_ext<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<UnitAttr>(&ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_EXT)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_bank_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_BANK_NV) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_bindless_sampler_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_BINDLESS_SAMPLER_NV) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_bindless_image_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_BINDLESS_IMAGE_NV) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_bound_sampler_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_BOUND_SAMPLER_NV) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_bound_image_nv<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_BOUND_IMAGE_NV) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_simt_call_intel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_SIMT_CALL_INTEL) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_referenced_indirectly_intel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_REFERENCED_INDIRECTLY_INTEL) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_clobber_intel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, StringAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<StringAttr>(&ATTR_CLOBBER_INTEL) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_side_effects_intel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_SIDE_EFFECTS_INTEL) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_vector_compute_variable_intel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_VECTOR_COMPUTE_VARIABLE_INTEL) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_func_param_io_kind_intel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<LiteralIntegerAttr>(&ATTR_FUNC_PARAM_IO_KIND_INTEL)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_vector_compute_function_intel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_VECTOR_COMPUTE_FUNCTION_INTEL) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_stack_call_intel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_STACK_CALL_INTEL) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_global_variable_offset_intel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes
                    .get::<LiteralIntegerAttr>(&ATTR_GLOBAL_VARIABLE_OFFSET_INTEL)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_user_semantic<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, StringAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<StringAttr>(&ATTR_USER_SEMANTIC) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_user_type_google<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, StringAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<StringAttr>(&ATTR_USER_TYPE_GOOGLE) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_register_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_REGISTER_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_memory_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, StringAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<StringAttr>(&ATTR_MEMORY_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_numbanks_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_NUMBANKS_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_bankwidth_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_BANKWIDTH_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_max_private_copies_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<LiteralIntegerAttr>(&ATTR_MAX_PRIVATE_COPIES_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_singlepump_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_SINGLEPUMP_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_doublepump_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_DOUBLEPUMP_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_max_replicates_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_MAX_REPLICATES_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_simple_dual_port_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_SIMPLE_DUAL_PORT_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_force_pow2_depth_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<LiteralIntegerAttr>(&ATTR_FORCE_POW2_DEPTH_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_stridesize_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_STRIDESIZE_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_wordsize_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_WORDSIZE_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_true_dual_port_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_TRUE_DUAL_PORT_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_burst_coalesce_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_BURST_COALESCE_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_cache_size_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_CACHE_SIZE_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_dont_statically_coalesce_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_DONT_STATICALLY_COALESCE_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_prefetch_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<LiteralIntegerAttr>(&ATTR_PREFETCH_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_stall_enable_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_STALL_ENABLE_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_fuse_loops_in_function_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_FUSE_LOOPS_IN_FUNCTION_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_initiation_interval_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<LiteralIntegerAttr>(&ATTR_INITIATION_INTERVAL_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_max_concurrency_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<LiteralIntegerAttr>(&ATTR_MAX_CONCURRENCY_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_pipeline_enable_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<LiteralIntegerAttr>(&ATTR_PIPELINE_ENABLE_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_buffer_location_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<LiteralIntegerAttr>(&ATTR_BUFFER_LOCATION_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_io_pipe_storage_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<LiteralIntegerAttr>(&ATTR_IO_PIPE_STORAGE_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_single_element_vector_intel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_SINGLE_ELEMENT_VECTOR_INTEL) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_vector_compute_callable_function_intel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes
                    .get::<UnitAttr>(&ATTR_VECTOR_COMPUTE_CALLABLE_FUNCTION_INTEL)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_media_block_iointel<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_MEDIA_BLOCK_IOINTEL) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_stall_free_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_STALL_FREE_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_latency_control_label_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes
                    .get::<LiteralIntegerAttr>(&ATTR_LATENCY_CONTROL_LABEL_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_conduit_kernel_argument_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_CONDUIT_KERNEL_ARGUMENT_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_register_map_kernel_argument_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes.get::<UnitAttr>(&ATTR_REGISTER_MAP_KERNEL_ARGUMENT_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_mm_host_interface_address_width_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes
                    .get::<
                        LiteralIntegerAttr,
                    >(&ATTR_MM_HOST_INTERFACE_ADDRESS_WIDTH_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_mm_host_interface_data_width_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes
                    .get::<LiteralIntegerAttr>(&ATTR_MM_HOST_INTERFACE_DATA_WIDTH_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_mm_host_interface_latency_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes
                    .get::<LiteralIntegerAttr>(&ATTR_MM_HOST_INTERFACE_LATENCY_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_mm_host_interface_max_burst_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes
                    .get::<LiteralIntegerAttr>(&ATTR_MM_HOST_INTERFACE_MAX_BURST_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_mm_host_interface_wait_request_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes
                    .get::<
                        LiteralIntegerAttr,
                    >(&ATTR_MM_HOST_INTERFACE_WAIT_REQUEST_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_stable_kernel_argument_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, UnitAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| { op.attributes.get::<UnitAttr>(&ATTR_STABLE_KERNEL_ARGUMENT_ALTERA) },
        )
        .ok()
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn get_decoration_implement_in_register_map_altera<'a>(
    op: &dyn DecoratableOp,
    ctx: &'a Context,
) -> Option<Ref<'a, LiteralIntegerAttr>> {
    Ref::filter_map(
            op.get_operation().deref(ctx),
            |op| {
                op.attributes
                    .get::<LiteralIntegerAttr>(&ATTR_IMPLEMENT_IN_REGISTER_MAP_ALTERA)
            },
        )
        .ok()
}
#[allow(non_snake_case)]
pub fn set_decoration_relaxed_precision(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_RELAXED_PRECISION.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_block(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_BLOCK.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_buffer_block(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_BUFFER_BLOCK.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_row_major(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_ROW_MAJOR.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_col_major(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_COL_MAJOR.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_array_stride(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_ARRAY_STRIDE.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_matrix_stride(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_MATRIX_STRIDE.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_glsl_shared(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_GLSL_SHARED.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_glsl_packed(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_GLSL_PACKED.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_c_packed(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_C_PACKED.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_built_in(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: BuiltInAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_BUILT_IN.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_no_perspective(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_NO_PERSPECTIVE.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_flat(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_FLAT.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_patch(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_PATCH.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_centroid(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_CENTROID.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_sample(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_SAMPLE.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_invariant(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_INVARIANT.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_restrict(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_RESTRICT.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_aliased(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_ALIASED.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_volatile(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_VOLATILE.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_constant(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_CONSTANT.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_coherent(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_COHERENT.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_non_writable(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_NON_WRITABLE.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_non_readable(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_NON_READABLE.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_uniform(op: &dyn DecoratableOp, ctx: &Context, value: UnitAttr) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_UNIFORM.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_saturated_conversion(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SATURATED_CONVERSION.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_stream(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_STREAM.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_location(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_LOCATION.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_component(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_COMPONENT.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_index(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_INDEX.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_binding(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_BINDING.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_descriptor_set(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_DESCRIPTOR_SET.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_offset(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_OFFSET.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_xfb_buffer(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_XFB_BUFFER.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_xfb_stride(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_XFB_STRIDE.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_func_param_attr(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: VecAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_FUNC_PARAM_ATTR.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_fp_rounding_mode(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: FPRoundingModeAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_FP_ROUNDING_MODE.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_fp_fast_math_mode(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: FPFastMathModeAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_FP_FAST_MATH_MODE.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_no_contraction(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_NO_CONTRACTION.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_input_attachment_index(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_INPUT_ATTACHMENT_INDEX.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_alignment(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_ALIGNMENT.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_max_byte_offset(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MAX_BYTE_OFFSET.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_saturated_to_largest_float8_normal_conversion_ext(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SATURATED_TO_LARGEST_FLOAT8_NORMAL_CONVERSION_EXT.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_no_signed_wrap(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_NO_SIGNED_WRAP.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_no_unsigned_wrap(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_NO_UNSIGNED_WRAP.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_weight_texture_qcom(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_WEIGHT_TEXTURE_QCOM.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_block_match_texture_qcom(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BLOCK_MATCH_TEXTURE_QCOM.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_block_match_sampler_qcom(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BLOCK_MATCH_SAMPLER_QCOM.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_explicit_interp_amd(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_EXPLICIT_INTERP_AMD.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_track_finish_writing_amdx(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_TRACK_FINISH_WRITING_AMDX.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_payload_node_sparse_array_amdx(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PAYLOAD_NODE_SPARSE_ARRAY_AMDX.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_payload_dispatch_indirect_amdx(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PAYLOAD_DISPATCH_INDIRECT_AMDX.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_utf_encoded_khr(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_UTF_ENCODED_KHR.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_override_coverage_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_OVERRIDE_COVERAGE_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_passthrough_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_PASSTHROUGH_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_viewport_relative_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_VIEWPORT_RELATIVE_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_secondary_viewport_relative_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SECONDARY_VIEWPORT_RELATIVE_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_per_primitive_ext(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PER_PRIMITIVE_EXT.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_per_view_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_PER_VIEW_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_per_task_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_PER_TASK_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_per_vertex_khr(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_PER_VERTEX_KHR.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_non_uniform(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_NON_UNIFORM.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_restrict_pointer(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_RESTRICT_POINTER.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_aliased_pointer(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_ALIASED_POINTER.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_member_offset_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MEMBER_OFFSET_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_hit_object_shader_record_buffer_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_hit_object_shader_record_buffer_ext(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_EXT.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_bank_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_BANK_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_bindless_sampler_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BINDLESS_SAMPLER_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_bindless_image_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BINDLESS_IMAGE_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_bound_sampler_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BOUND_SAMPLER_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_bound_image_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_BOUND_IMAGE_NV.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_simt_call_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SIMT_CALL_INTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_referenced_indirectly_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_REFERENCED_INDIRECTLY_INTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_clobber_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: StringAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_CLOBBER_INTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_side_effects_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SIDE_EFFECTS_INTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_vector_compute_variable_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_VECTOR_COMPUTE_VARIABLE_INTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_func_param_io_kind_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_FUNC_PARAM_IO_KIND_INTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_vector_compute_function_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_VECTOR_COMPUTE_FUNCTION_INTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_stack_call_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_STACK_CALL_INTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_global_variable_offset_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_GLOBAL_VARIABLE_OFFSET_INTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_user_semantic(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: StringAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_USER_SEMANTIC.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_user_type_google(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: StringAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_USER_TYPE_GOOGLE.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_register_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_REGISTER_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_memory_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: StringAttr,
) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_MEMORY_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_numbanks_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_NUMBANKS_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_bankwidth_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BANKWIDTH_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_max_private_copies_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MAX_PRIVATE_COPIES_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_singlepump_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SINGLEPUMP_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_doublepump_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_DOUBLEPUMP_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_max_replicates_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MAX_REPLICATES_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_simple_dual_port_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SIMPLE_DUAL_PORT_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_force_pow2_depth_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_FORCE_POW2_DEPTH_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_stridesize_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_STRIDESIZE_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_wordsize_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_WORDSIZE_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_true_dual_port_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_TRUE_DUAL_PORT_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_burst_coalesce_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BURST_COALESCE_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_cache_size_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_CACHE_SIZE_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_dont_statically_coalesce_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_DONT_STATICALLY_COALESCE_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_prefetch_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PREFETCH_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_stall_enable_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_STALL_ENABLE_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_fuse_loops_in_function_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_FUSE_LOOPS_IN_FUNCTION_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_initiation_interval_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_INITIATION_INTERVAL_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_max_concurrency_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MAX_CONCURRENCY_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_pipeline_enable_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PIPELINE_ENABLE_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_buffer_location_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BUFFER_LOCATION_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_io_pipe_storage_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_IO_PIPE_STORAGE_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_single_element_vector_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SINGLE_ELEMENT_VECTOR_INTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_vector_compute_callable_function_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_VECTOR_COMPUTE_CALLABLE_FUNCTION_INTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_media_block_iointel(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MEDIA_BLOCK_IOINTEL.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_stall_free_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_STALL_FREE_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_latency_control_label_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_LATENCY_CONTROL_LABEL_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_conduit_kernel_argument_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_CONDUIT_KERNEL_ARGUMENT_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_register_map_kernel_argument_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_REGISTER_MAP_KERNEL_ARGUMENT_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_mm_host_interface_address_width_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MM_HOST_INTERFACE_ADDRESS_WIDTH_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_mm_host_interface_data_width_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MM_HOST_INTERFACE_DATA_WIDTH_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_mm_host_interface_latency_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MM_HOST_INTERFACE_LATENCY_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_mm_host_interface_max_burst_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MM_HOST_INTERFACE_MAX_BURST_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_mm_host_interface_wait_request_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MM_HOST_INTERFACE_WAIT_REQUEST_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_stable_kernel_argument_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: UnitAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_STABLE_KERNEL_ARGUMENT_ALTERA.clone(), value);
}
#[allow(non_snake_case)]
pub fn set_decoration_implement_in_register_map_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
    value: LiteralIntegerAttr,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_IMPLEMENT_IN_REGISTER_MAP_ALTERA.clone(), value);
}
impl DecorationExt for Decoration {
    fn decoration_key(&self) -> &'static Identifier {
        match self {
            Decoration::RelaxedPrecision => &ATTR_RELAXED_PRECISION,
            Decoration::Block => &ATTR_BLOCK,
            Decoration::BufferBlock => &ATTR_BUFFER_BLOCK,
            Decoration::RowMajor => &ATTR_ROW_MAJOR,
            Decoration::ColMajor => &ATTR_COL_MAJOR,
            Decoration::ArrayStride => &ATTR_ARRAY_STRIDE,
            Decoration::MatrixStride => &ATTR_MATRIX_STRIDE,
            Decoration::GLSLShared => &ATTR_GLSL_SHARED,
            Decoration::GLSLPacked => &ATTR_GLSL_PACKED,
            Decoration::CPacked => &ATTR_C_PACKED,
            Decoration::BuiltIn => &ATTR_BUILT_IN,
            Decoration::NoPerspective => &ATTR_NO_PERSPECTIVE,
            Decoration::Flat => &ATTR_FLAT,
            Decoration::Patch => &ATTR_PATCH,
            Decoration::Centroid => &ATTR_CENTROID,
            Decoration::Sample => &ATTR_SAMPLE,
            Decoration::Invariant => &ATTR_INVARIANT,
            Decoration::Restrict => &ATTR_RESTRICT,
            Decoration::Aliased => &ATTR_ALIASED,
            Decoration::Volatile => &ATTR_VOLATILE,
            Decoration::Constant => &ATTR_CONSTANT,
            Decoration::Coherent => &ATTR_COHERENT,
            Decoration::NonWritable => &ATTR_NON_WRITABLE,
            Decoration::NonReadable => &ATTR_NON_READABLE,
            Decoration::Uniform => &ATTR_UNIFORM,
            Decoration::SaturatedConversion => &ATTR_SATURATED_CONVERSION,
            Decoration::Stream => &ATTR_STREAM,
            Decoration::Location => &ATTR_LOCATION,
            Decoration::Component => &ATTR_COMPONENT,
            Decoration::Index => &ATTR_INDEX,
            Decoration::Binding => &ATTR_BINDING,
            Decoration::DescriptorSet => &ATTR_DESCRIPTOR_SET,
            Decoration::Offset => &ATTR_OFFSET,
            Decoration::XfbBuffer => &ATTR_XFB_BUFFER,
            Decoration::XfbStride => &ATTR_XFB_STRIDE,
            Decoration::FuncParamAttr => &ATTR_FUNC_PARAM_ATTR,
            Decoration::FPRoundingMode => &ATTR_FP_ROUNDING_MODE,
            Decoration::FPFastMathMode => &ATTR_FP_FAST_MATH_MODE,
            Decoration::NoContraction => &ATTR_NO_CONTRACTION,
            Decoration::InputAttachmentIndex => &ATTR_INPUT_ATTACHMENT_INDEX,
            Decoration::Alignment => &ATTR_ALIGNMENT,
            Decoration::MaxByteOffset => &ATTR_MAX_BYTE_OFFSET,
            Decoration::SaturatedToLargestFloat8NormalConversionEXT => {
                &ATTR_SATURATED_TO_LARGEST_FLOAT8_NORMAL_CONVERSION_EXT
            }
            Decoration::NoSignedWrap => &ATTR_NO_SIGNED_WRAP,
            Decoration::NoUnsignedWrap => &ATTR_NO_UNSIGNED_WRAP,
            Decoration::WeightTextureQCOM => &ATTR_WEIGHT_TEXTURE_QCOM,
            Decoration::BlockMatchTextureQCOM => &ATTR_BLOCK_MATCH_TEXTURE_QCOM,
            Decoration::BlockMatchSamplerQCOM => &ATTR_BLOCK_MATCH_SAMPLER_QCOM,
            Decoration::ExplicitInterpAMD => &ATTR_EXPLICIT_INTERP_AMD,
            Decoration::TrackFinishWritingAMDX => &ATTR_TRACK_FINISH_WRITING_AMDX,
            Decoration::PayloadNodeSparseArrayAMDX => {
                &ATTR_PAYLOAD_NODE_SPARSE_ARRAY_AMDX
            }
            Decoration::PayloadDispatchIndirectAMDX => {
                &ATTR_PAYLOAD_DISPATCH_INDIRECT_AMDX
            }
            Decoration::UTFEncodedKHR => &ATTR_UTF_ENCODED_KHR,
            Decoration::OverrideCoverageNV => &ATTR_OVERRIDE_COVERAGE_NV,
            Decoration::PassthroughNV => &ATTR_PASSTHROUGH_NV,
            Decoration::ViewportRelativeNV => &ATTR_VIEWPORT_RELATIVE_NV,
            Decoration::SecondaryViewportRelativeNV => {
                &ATTR_SECONDARY_VIEWPORT_RELATIVE_NV
            }
            Decoration::PerPrimitiveEXT => &ATTR_PER_PRIMITIVE_EXT,
            Decoration::PerViewNV => &ATTR_PER_VIEW_NV,
            Decoration::PerTaskNV => &ATTR_PER_TASK_NV,
            Decoration::PerVertexKHR => &ATTR_PER_VERTEX_KHR,
            Decoration::NonUniform => &ATTR_NON_UNIFORM,
            Decoration::RestrictPointer => &ATTR_RESTRICT_POINTER,
            Decoration::AliasedPointer => &ATTR_ALIASED_POINTER,
            Decoration::MemberOffsetNV => &ATTR_MEMBER_OFFSET_NV,
            Decoration::HitObjectShaderRecordBufferNV => {
                &ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_NV
            }
            Decoration::HitObjectShaderRecordBufferEXT => {
                &ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_EXT
            }
            Decoration::BankNV => &ATTR_BANK_NV,
            Decoration::BindlessSamplerNV => &ATTR_BINDLESS_SAMPLER_NV,
            Decoration::BindlessImageNV => &ATTR_BINDLESS_IMAGE_NV,
            Decoration::BoundSamplerNV => &ATTR_BOUND_SAMPLER_NV,
            Decoration::BoundImageNV => &ATTR_BOUND_IMAGE_NV,
            Decoration::SIMTCallINTEL => &ATTR_SIMT_CALL_INTEL,
            Decoration::ReferencedIndirectlyINTEL => &ATTR_REFERENCED_INDIRECTLY_INTEL,
            Decoration::ClobberINTEL => &ATTR_CLOBBER_INTEL,
            Decoration::SideEffectsINTEL => &ATTR_SIDE_EFFECTS_INTEL,
            Decoration::VectorComputeVariableINTEL => &ATTR_VECTOR_COMPUTE_VARIABLE_INTEL,
            Decoration::FuncParamIOKindINTEL => &ATTR_FUNC_PARAM_IO_KIND_INTEL,
            Decoration::VectorComputeFunctionINTEL => &ATTR_VECTOR_COMPUTE_FUNCTION_INTEL,
            Decoration::StackCallINTEL => &ATTR_STACK_CALL_INTEL,
            Decoration::GlobalVariableOffsetINTEL => &ATTR_GLOBAL_VARIABLE_OFFSET_INTEL,
            Decoration::UserSemantic => &ATTR_USER_SEMANTIC,
            Decoration::UserTypeGOOGLE => &ATTR_USER_TYPE_GOOGLE,
            Decoration::RegisterALTERA => &ATTR_REGISTER_ALTERA,
            Decoration::MemoryALTERA => &ATTR_MEMORY_ALTERA,
            Decoration::NumbanksALTERA => &ATTR_NUMBANKS_ALTERA,
            Decoration::BankwidthALTERA => &ATTR_BANKWIDTH_ALTERA,
            Decoration::MaxPrivateCopiesALTERA => &ATTR_MAX_PRIVATE_COPIES_ALTERA,
            Decoration::SinglepumpALTERA => &ATTR_SINGLEPUMP_ALTERA,
            Decoration::DoublepumpALTERA => &ATTR_DOUBLEPUMP_ALTERA,
            Decoration::MaxReplicatesALTERA => &ATTR_MAX_REPLICATES_ALTERA,
            Decoration::SimpleDualPortALTERA => &ATTR_SIMPLE_DUAL_PORT_ALTERA,
            Decoration::ForcePow2DepthALTERA => &ATTR_FORCE_POW2_DEPTH_ALTERA,
            Decoration::StridesizeALTERA => &ATTR_STRIDESIZE_ALTERA,
            Decoration::WordsizeALTERA => &ATTR_WORDSIZE_ALTERA,
            Decoration::TrueDualPortALTERA => &ATTR_TRUE_DUAL_PORT_ALTERA,
            Decoration::BurstCoalesceALTERA => &ATTR_BURST_COALESCE_ALTERA,
            Decoration::CacheSizeALTERA => &ATTR_CACHE_SIZE_ALTERA,
            Decoration::DontStaticallyCoalesceALTERA => {
                &ATTR_DONT_STATICALLY_COALESCE_ALTERA
            }
            Decoration::PrefetchALTERA => &ATTR_PREFETCH_ALTERA,
            Decoration::StallEnableALTERA => &ATTR_STALL_ENABLE_ALTERA,
            Decoration::FuseLoopsInFunctionALTERA => &ATTR_FUSE_LOOPS_IN_FUNCTION_ALTERA,
            Decoration::InitiationIntervalALTERA => &ATTR_INITIATION_INTERVAL_ALTERA,
            Decoration::MaxConcurrencyALTERA => &ATTR_MAX_CONCURRENCY_ALTERA,
            Decoration::PipelineEnableALTERA => &ATTR_PIPELINE_ENABLE_ALTERA,
            Decoration::BufferLocationALTERA => &ATTR_BUFFER_LOCATION_ALTERA,
            Decoration::IOPipeStorageALTERA => &ATTR_IO_PIPE_STORAGE_ALTERA,
            Decoration::SingleElementVectorINTEL => &ATTR_SINGLE_ELEMENT_VECTOR_INTEL,
            Decoration::VectorComputeCallableFunctionINTEL => {
                &ATTR_VECTOR_COMPUTE_CALLABLE_FUNCTION_INTEL
            }
            Decoration::MediaBlockIOINTEL => &ATTR_MEDIA_BLOCK_IOINTEL,
            Decoration::StallFreeALTERA => &ATTR_STALL_FREE_ALTERA,
            Decoration::LatencyControlLabelALTERA => &ATTR_LATENCY_CONTROL_LABEL_ALTERA,
            Decoration::ConduitKernelArgumentALTERA => {
                &ATTR_CONDUIT_KERNEL_ARGUMENT_ALTERA
            }
            Decoration::RegisterMapKernelArgumentALTERA => {
                &ATTR_REGISTER_MAP_KERNEL_ARGUMENT_ALTERA
            }
            Decoration::MMHostInterfaceAddressWidthALTERA => {
                &ATTR_MM_HOST_INTERFACE_ADDRESS_WIDTH_ALTERA
            }
            Decoration::MMHostInterfaceDataWidthALTERA => {
                &ATTR_MM_HOST_INTERFACE_DATA_WIDTH_ALTERA
            }
            Decoration::MMHostInterfaceLatencyALTERA => {
                &ATTR_MM_HOST_INTERFACE_LATENCY_ALTERA
            }
            Decoration::MMHostInterfaceMaxBurstALTERA => {
                &ATTR_MM_HOST_INTERFACE_MAX_BURST_ALTERA
            }
            Decoration::MMHostInterfaceWaitRequestALTERA => {
                &ATTR_MM_HOST_INTERFACE_WAIT_REQUEST_ALTERA
            }
            Decoration::StableKernelArgumentALTERA => &ATTR_STABLE_KERNEL_ARGUMENT_ALTERA,
            Decoration::ImplementInRegisterMapALTERA => {
                &ATTR_IMPLEMENT_IN_REGISTER_MAP_ALTERA
            }
            _ => unimplemented!("Unsupported decoration"),
        }
    }
}
pub fn decoration_for_key(identifier: &Identifier) -> Option<Decoration> {
    match identifier.as_str() {
        "relaxed_precision" => Some(Decoration::RelaxedPrecision),
        "block" => Some(Decoration::Block),
        "buffer_block" => Some(Decoration::BufferBlock),
        "row_major" => Some(Decoration::RowMajor),
        "col_major" => Some(Decoration::ColMajor),
        "array_stride" => Some(Decoration::ArrayStride),
        "matrix_stride" => Some(Decoration::MatrixStride),
        "glsl_shared" => Some(Decoration::GLSLShared),
        "glsl_packed" => Some(Decoration::GLSLPacked),
        "c_packed" => Some(Decoration::CPacked),
        "built_in" => Some(Decoration::BuiltIn),
        "no_perspective" => Some(Decoration::NoPerspective),
        "flat" => Some(Decoration::Flat),
        "patch" => Some(Decoration::Patch),
        "centroid" => Some(Decoration::Centroid),
        "sample" => Some(Decoration::Sample),
        "invariant" => Some(Decoration::Invariant),
        "restrict" => Some(Decoration::Restrict),
        "aliased" => Some(Decoration::Aliased),
        "volatile" => Some(Decoration::Volatile),
        "constant" => Some(Decoration::Constant),
        "coherent" => Some(Decoration::Coherent),
        "non_writable" => Some(Decoration::NonWritable),
        "non_readable" => Some(Decoration::NonReadable),
        "uniform" => Some(Decoration::Uniform),
        "saturated_conversion" => Some(Decoration::SaturatedConversion),
        "stream" => Some(Decoration::Stream),
        "location" => Some(Decoration::Location),
        "component" => Some(Decoration::Component),
        "index" => Some(Decoration::Index),
        "binding" => Some(Decoration::Binding),
        "descriptor_set" => Some(Decoration::DescriptorSet),
        "offset" => Some(Decoration::Offset),
        "xfb_buffer" => Some(Decoration::XfbBuffer),
        "xfb_stride" => Some(Decoration::XfbStride),
        "func_param_attr" => Some(Decoration::FuncParamAttr),
        "fp_rounding_mode" => Some(Decoration::FPRoundingMode),
        "fp_fast_math_mode" => Some(Decoration::FPFastMathMode),
        "no_contraction" => Some(Decoration::NoContraction),
        "input_attachment_index" => Some(Decoration::InputAttachmentIndex),
        "alignment" => Some(Decoration::Alignment),
        "max_byte_offset" => Some(Decoration::MaxByteOffset),
        "saturated_to_largest_float8_normal_conversion_ext" => {
            Some(Decoration::SaturatedToLargestFloat8NormalConversionEXT)
        }
        "no_signed_wrap" => Some(Decoration::NoSignedWrap),
        "no_unsigned_wrap" => Some(Decoration::NoUnsignedWrap),
        "weight_texture_qcom" => Some(Decoration::WeightTextureQCOM),
        "block_match_texture_qcom" => Some(Decoration::BlockMatchTextureQCOM),
        "block_match_sampler_qcom" => Some(Decoration::BlockMatchSamplerQCOM),
        "explicit_interp_amd" => Some(Decoration::ExplicitInterpAMD),
        "track_finish_writing_amdx" => Some(Decoration::TrackFinishWritingAMDX),
        "payload_node_sparse_array_amdx" => Some(Decoration::PayloadNodeSparseArrayAMDX),
        "payload_dispatch_indirect_amdx" => Some(Decoration::PayloadDispatchIndirectAMDX),
        "utf_encoded_khr" => Some(Decoration::UTFEncodedKHR),
        "override_coverage_nv" => Some(Decoration::OverrideCoverageNV),
        "passthrough_nv" => Some(Decoration::PassthroughNV),
        "viewport_relative_nv" => Some(Decoration::ViewportRelativeNV),
        "secondary_viewport_relative_nv" => Some(Decoration::SecondaryViewportRelativeNV),
        "per_primitive_ext" => Some(Decoration::PerPrimitiveEXT),
        "per_view_nv" => Some(Decoration::PerViewNV),
        "per_task_nv" => Some(Decoration::PerTaskNV),
        "per_vertex_khr" => Some(Decoration::PerVertexKHR),
        "non_uniform" => Some(Decoration::NonUniform),
        "restrict_pointer" => Some(Decoration::RestrictPointer),
        "aliased_pointer" => Some(Decoration::AliasedPointer),
        "member_offset_nv" => Some(Decoration::MemberOffsetNV),
        "hit_object_shader_record_buffer_nv" => {
            Some(Decoration::HitObjectShaderRecordBufferNV)
        }
        "hit_object_shader_record_buffer_ext" => {
            Some(Decoration::HitObjectShaderRecordBufferEXT)
        }
        "bank_nv" => Some(Decoration::BankNV),
        "bindless_sampler_nv" => Some(Decoration::BindlessSamplerNV),
        "bindless_image_nv" => Some(Decoration::BindlessImageNV),
        "bound_sampler_nv" => Some(Decoration::BoundSamplerNV),
        "bound_image_nv" => Some(Decoration::BoundImageNV),
        "simt_call_intel" => Some(Decoration::SIMTCallINTEL),
        "referenced_indirectly_intel" => Some(Decoration::ReferencedIndirectlyINTEL),
        "clobber_intel" => Some(Decoration::ClobberINTEL),
        "side_effects_intel" => Some(Decoration::SideEffectsINTEL),
        "vector_compute_variable_intel" => Some(Decoration::VectorComputeVariableINTEL),
        "func_param_io_kind_intel" => Some(Decoration::FuncParamIOKindINTEL),
        "vector_compute_function_intel" => Some(Decoration::VectorComputeFunctionINTEL),
        "stack_call_intel" => Some(Decoration::StackCallINTEL),
        "global_variable_offset_intel" => Some(Decoration::GlobalVariableOffsetINTEL),
        "user_semantic" => Some(Decoration::UserSemantic),
        "user_type_google" => Some(Decoration::UserTypeGOOGLE),
        "register_altera" => Some(Decoration::RegisterALTERA),
        "memory_altera" => Some(Decoration::MemoryALTERA),
        "numbanks_altera" => Some(Decoration::NumbanksALTERA),
        "bankwidth_altera" => Some(Decoration::BankwidthALTERA),
        "max_private_copies_altera" => Some(Decoration::MaxPrivateCopiesALTERA),
        "singlepump_altera" => Some(Decoration::SinglepumpALTERA),
        "doublepump_altera" => Some(Decoration::DoublepumpALTERA),
        "max_replicates_altera" => Some(Decoration::MaxReplicatesALTERA),
        "simple_dual_port_altera" => Some(Decoration::SimpleDualPortALTERA),
        "force_pow2_depth_altera" => Some(Decoration::ForcePow2DepthALTERA),
        "stridesize_altera" => Some(Decoration::StridesizeALTERA),
        "wordsize_altera" => Some(Decoration::WordsizeALTERA),
        "true_dual_port_altera" => Some(Decoration::TrueDualPortALTERA),
        "burst_coalesce_altera" => Some(Decoration::BurstCoalesceALTERA),
        "cache_size_altera" => Some(Decoration::CacheSizeALTERA),
        "dont_statically_coalesce_altera" => {
            Some(Decoration::DontStaticallyCoalesceALTERA)
        }
        "prefetch_altera" => Some(Decoration::PrefetchALTERA),
        "stall_enable_altera" => Some(Decoration::StallEnableALTERA),
        "fuse_loops_in_function_altera" => Some(Decoration::FuseLoopsInFunctionALTERA),
        "initiation_interval_altera" => Some(Decoration::InitiationIntervalALTERA),
        "max_concurrency_altera" => Some(Decoration::MaxConcurrencyALTERA),
        "pipeline_enable_altera" => Some(Decoration::PipelineEnableALTERA),
        "buffer_location_altera" => Some(Decoration::BufferLocationALTERA),
        "io_pipe_storage_altera" => Some(Decoration::IOPipeStorageALTERA),
        "single_element_vector_intel" => Some(Decoration::SingleElementVectorINTEL),
        "vector_compute_callable_function_intel" => {
            Some(Decoration::VectorComputeCallableFunctionINTEL)
        }
        "media_block_iointel" => Some(Decoration::MediaBlockIOINTEL),
        "stall_free_altera" => Some(Decoration::StallFreeALTERA),
        "latency_control_label_altera" => Some(Decoration::LatencyControlLabelALTERA),
        "conduit_kernel_argument_altera" => Some(Decoration::ConduitKernelArgumentALTERA),
        "register_map_kernel_argument_altera" => {
            Some(Decoration::RegisterMapKernelArgumentALTERA)
        }
        "mm_host_interface_address_width_altera" => {
            Some(Decoration::MMHostInterfaceAddressWidthALTERA)
        }
        "mm_host_interface_data_width_altera" => {
            Some(Decoration::MMHostInterfaceDataWidthALTERA)
        }
        "mm_host_interface_latency_altera" => {
            Some(Decoration::MMHostInterfaceLatencyALTERA)
        }
        "mm_host_interface_max_burst_altera" => {
            Some(Decoration::MMHostInterfaceMaxBurstALTERA)
        }
        "mm_host_interface_wait_request_altera" => {
            Some(Decoration::MMHostInterfaceWaitRequestALTERA)
        }
        "stable_kernel_argument_altera" => Some(Decoration::StableKernelArgumentALTERA),
        "implement_in_register_map_altera" => {
            Some(Decoration::ImplementInRegisterMapALTERA)
        }
        _ => None,
    }
}
impl DecorationInfo {
    pub fn as_operands(&self) -> Vec<Operand> {
        match self.decoration {
            Decoration::RelaxedPrecision
            | Decoration::Block
            | Decoration::BufferBlock
            | Decoration::RowMajor
            | Decoration::ColMajor
            | Decoration::GLSLShared
            | Decoration::GLSLPacked
            | Decoration::CPacked
            | Decoration::NoPerspective
            | Decoration::Flat
            | Decoration::Patch
            | Decoration::Centroid
            | Decoration::Sample
            | Decoration::Invariant
            | Decoration::Restrict
            | Decoration::Aliased
            | Decoration::Volatile
            | Decoration::Constant
            | Decoration::Coherent
            | Decoration::NonWritable
            | Decoration::NonReadable
            | Decoration::Uniform
            | Decoration::SaturatedConversion
            | Decoration::NoContraction
            | Decoration::SaturatedToLargestFloat8NormalConversionEXT
            | Decoration::NoSignedWrap
            | Decoration::NoUnsignedWrap
            | Decoration::WeightTextureQCOM
            | Decoration::BlockMatchTextureQCOM
            | Decoration::BlockMatchSamplerQCOM
            | Decoration::ExplicitInterpAMD
            | Decoration::TrackFinishWritingAMDX
            | Decoration::PayloadNodeSparseArrayAMDX
            | Decoration::PayloadDispatchIndirectAMDX
            | Decoration::UTFEncodedKHR
            | Decoration::OverrideCoverageNV
            | Decoration::PassthroughNV
            | Decoration::ViewportRelativeNV
            | Decoration::PerPrimitiveEXT
            | Decoration::PerViewNV
            | Decoration::PerTaskNV
            | Decoration::PerVertexKHR
            | Decoration::NonUniform
            | Decoration::RestrictPointer
            | Decoration::AliasedPointer
            | Decoration::HitObjectShaderRecordBufferNV
            | Decoration::HitObjectShaderRecordBufferEXT
            | Decoration::BindlessSamplerNV
            | Decoration::BindlessImageNV
            | Decoration::BoundSamplerNV
            | Decoration::BoundImageNV
            | Decoration::ReferencedIndirectlyINTEL
            | Decoration::SideEffectsINTEL
            | Decoration::VectorComputeVariableINTEL
            | Decoration::VectorComputeFunctionINTEL
            | Decoration::StackCallINTEL
            | Decoration::RegisterALTERA
            | Decoration::SinglepumpALTERA
            | Decoration::DoublepumpALTERA
            | Decoration::SimpleDualPortALTERA
            | Decoration::TrueDualPortALTERA
            | Decoration::BurstCoalesceALTERA
            | Decoration::DontStaticallyCoalesceALTERA
            | Decoration::StallEnableALTERA
            | Decoration::FuseLoopsInFunctionALTERA
            | Decoration::SingleElementVectorINTEL
            | Decoration::VectorComputeCallableFunctionINTEL
            | Decoration::MediaBlockIOINTEL
            | Decoration::StallFreeALTERA
            | Decoration::ConduitKernelArgumentALTERA
            | Decoration::RegisterMapKernelArgumentALTERA
            | Decoration::StableKernelArgumentALTERA => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<UnitAttr>().unwrap();
                vec![]
            }
            Decoration::FPRoundingMode => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<FPRoundingModeAttr>().unwrap();
                vec![Operand::FPRoundingMode(attr.0)]
            }
            Decoration::BuiltIn => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<BuiltInAttr>().unwrap();
                vec![Operand::BuiltIn(attr.0)]
            }
            Decoration::FuncParamAttr => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<VecAttr>().unwrap();
                todo!()
            }
            Decoration::ArrayStride
            | Decoration::MatrixStride
            | Decoration::Stream
            | Decoration::Location
            | Decoration::Component
            | Decoration::Index
            | Decoration::Binding
            | Decoration::DescriptorSet
            | Decoration::Offset
            | Decoration::XfbBuffer
            | Decoration::XfbStride
            | Decoration::InputAttachmentIndex
            | Decoration::Alignment
            | Decoration::MaxByteOffset
            | Decoration::SecondaryViewportRelativeNV
            | Decoration::MemberOffsetNV
            | Decoration::BankNV
            | Decoration::SIMTCallINTEL
            | Decoration::FuncParamIOKindINTEL
            | Decoration::GlobalVariableOffsetINTEL
            | Decoration::NumbanksALTERA
            | Decoration::BankwidthALTERA
            | Decoration::MaxPrivateCopiesALTERA
            | Decoration::MaxReplicatesALTERA
            | Decoration::ForcePow2DepthALTERA
            | Decoration::StridesizeALTERA
            | Decoration::WordsizeALTERA
            | Decoration::CacheSizeALTERA
            | Decoration::PrefetchALTERA
            | Decoration::InitiationIntervalALTERA
            | Decoration::MaxConcurrencyALTERA
            | Decoration::PipelineEnableALTERA
            | Decoration::BufferLocationALTERA
            | Decoration::IOPipeStorageALTERA
            | Decoration::LatencyControlLabelALTERA
            | Decoration::MMHostInterfaceAddressWidthALTERA
            | Decoration::MMHostInterfaceDataWidthALTERA
            | Decoration::MMHostInterfaceLatencyALTERA
            | Decoration::MMHostInterfaceMaxBurstALTERA
            | Decoration::MMHostInterfaceWaitRequestALTERA
            | Decoration::ImplementInRegisterMapALTERA => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<LiteralIntegerAttr>().unwrap();
                vec![Operand::LiteralBit32(attr.0)]
            }
            Decoration::FPFastMathMode => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<FPFastMathModeAttr>().unwrap();
                vec![Operand::FPFastMathMode(attr.0)]
            }
            Decoration::ClobberINTEL
            | Decoration::UserSemantic
            | Decoration::UserTypeGOOGLE
            | Decoration::MemoryALTERA => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<StringAttr>().unwrap();
                vec![Operand::LiteralString(attr.as_str().to_string())]
            }
            _ => unimplemented!("Unsupported decoration"),
        }
    }
}
