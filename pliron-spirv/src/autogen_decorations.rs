// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use crate::{
    prelude::*, decorations::{DecorationInfo, DecorationExt, all_decorations_for_op},
};
use core::cell::Ref;
pub static ATTR_RELAXED_PRECISION: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_relaxed_precision".try_into().unwrap()
});
pub static ATTR_BLOCK: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_block".try_into().unwrap()
});
pub static ATTR_BUFFER_BLOCK: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_buffer_block".try_into().unwrap()
});
pub static ATTR_ROW_MAJOR: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_row_major".try_into().unwrap()
});
pub static ATTR_COL_MAJOR: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_col_major".try_into().unwrap()
});
pub static ATTR_ARRAY_STRIDE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_array_stride".try_into().unwrap()
});
pub static ATTR_MATRIX_STRIDE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_matrix_stride".try_into().unwrap()
});
pub static ATTR_GLSL_SHARED: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_glsl_shared".try_into().unwrap()
});
pub static ATTR_GLSL_PACKED: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_glsl_packed".try_into().unwrap()
});
pub static ATTR_C_PACKED: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_c_packed".try_into().unwrap()
});
pub static ATTR_BUILT_IN: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_built_in".try_into().unwrap()
});
pub static ATTR_NO_PERSPECTIVE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_no_perspective".try_into().unwrap()
});
pub static ATTR_FLAT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_flat".try_into().unwrap()
});
pub static ATTR_PATCH: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_patch".try_into().unwrap()
});
pub static ATTR_CENTROID: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_centroid".try_into().unwrap()
});
pub static ATTR_SAMPLE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_sample".try_into().unwrap()
});
pub static ATTR_INVARIANT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_invariant".try_into().unwrap()
});
pub static ATTR_RESTRICT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_restrict".try_into().unwrap()
});
pub static ATTR_ALIASED: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_aliased".try_into().unwrap()
});
pub static ATTR_VOLATILE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_volatile".try_into().unwrap()
});
pub static ATTR_CONSTANT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_constant".try_into().unwrap()
});
pub static ATTR_COHERENT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_coherent".try_into().unwrap()
});
pub static ATTR_NON_WRITABLE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_non_writable".try_into().unwrap()
});
pub static ATTR_NON_READABLE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_non_readable".try_into().unwrap()
});
pub static ATTR_UNIFORM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_uniform".try_into().unwrap()
});
pub static ATTR_SATURATED_CONVERSION: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_saturated_conversion".try_into().unwrap()
});
pub static ATTR_STREAM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_stream".try_into().unwrap()
});
pub static ATTR_LOCATION: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_location".try_into().unwrap()
});
pub static ATTR_COMPONENT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_component".try_into().unwrap()
});
pub static ATTR_INDEX: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_index".try_into().unwrap()
});
pub static ATTR_BINDING: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_binding".try_into().unwrap()
});
pub static ATTR_DESCRIPTOR_SET: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_descriptor_set".try_into().unwrap()
});
pub static ATTR_OFFSET: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_offset".try_into().unwrap()
});
pub static ATTR_XFB_BUFFER: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_xfb_buffer".try_into().unwrap()
});
pub static ATTR_XFB_STRIDE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_xfb_stride".try_into().unwrap()
});
pub static ATTR_FUNC_PARAM_ATTR: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_func_param_attr".try_into().unwrap()
});
pub static ATTR_FP_ROUNDING_MODE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_fp_rounding_mode".try_into().unwrap()
});
pub static ATTR_FP_FAST_MATH_MODE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_fp_fast_math_mode".try_into().unwrap()
});
pub static ATTR_NO_CONTRACTION: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_no_contraction".try_into().unwrap()
});
pub static ATTR_INPUT_ATTACHMENT_INDEX: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_input_attachment_index".try_into().unwrap()
});
pub static ATTR_ALIGNMENT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_alignment".try_into().unwrap()
});
pub static ATTR_MAX_BYTE_OFFSET: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_max_byte_offset".try_into().unwrap()
});
pub static ATTR_SATURATED_TO_LARGEST_FLOAT8_NORMAL_CONVERSION_EXT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_saturated_to_largest_float8_normal_conversion_ext"
        .try_into()
        .unwrap()
});
pub static ATTR_NO_SIGNED_WRAP: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_no_signed_wrap".try_into().unwrap()
});
pub static ATTR_NO_UNSIGNED_WRAP: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_no_unsigned_wrap".try_into().unwrap()
});
pub static ATTR_WEIGHT_TEXTURE_QCOM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_weight_texture_qcom".try_into().unwrap()
});
pub static ATTR_BLOCK_MATCH_TEXTURE_QCOM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_block_match_texture_qcom".try_into().unwrap()
});
pub static ATTR_BLOCK_MATCH_SAMPLER_QCOM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_block_match_sampler_qcom".try_into().unwrap()
});
pub static ATTR_EXPLICIT_INTERP_AMD: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_explicit_interp_amd".try_into().unwrap()
});
pub static ATTR_TRACK_FINISH_WRITING_AMDX: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_track_finish_writing_amdx".try_into().unwrap()
});
pub static ATTR_PAYLOAD_NODE_SPARSE_ARRAY_AMDX: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_payload_node_sparse_array_amdx".try_into().unwrap()
});
pub static ATTR_PAYLOAD_DISPATCH_INDIRECT_AMDX: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_payload_dispatch_indirect_amdx".try_into().unwrap()
});
pub static ATTR_UTF_ENCODED_KHR: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_utf_encoded_khr".try_into().unwrap()
});
pub static ATTR_OVERRIDE_COVERAGE_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_override_coverage_nv".try_into().unwrap()
});
pub static ATTR_PASSTHROUGH_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_passthrough_nv".try_into().unwrap()
});
pub static ATTR_VIEWPORT_RELATIVE_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_viewport_relative_nv".try_into().unwrap()
});
pub static ATTR_SECONDARY_VIEWPORT_RELATIVE_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_secondary_viewport_relative_nv".try_into().unwrap()
});
pub static ATTR_PER_PRIMITIVE_EXT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_per_primitive_ext".try_into().unwrap()
});
pub static ATTR_PER_VIEW_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_per_view_nv".try_into().unwrap()
});
pub static ATTR_PER_TASK_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_per_task_nv".try_into().unwrap()
});
pub static ATTR_PER_VERTEX_KHR: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_per_vertex_khr".try_into().unwrap()
});
pub static ATTR_NON_UNIFORM: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_non_uniform".try_into().unwrap()
});
pub static ATTR_RESTRICT_POINTER: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_restrict_pointer".try_into().unwrap()
});
pub static ATTR_ALIASED_POINTER: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_aliased_pointer".try_into().unwrap()
});
pub static ATTR_MEMBER_OFFSET_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_member_offset_nv".try_into().unwrap()
});
pub static ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_hit_object_shader_record_buffer_nv".try_into().unwrap()
});
pub static ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_EXT: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_hit_object_shader_record_buffer_ext".try_into().unwrap()
});
pub static ATTR_BANK_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_bank_nv".try_into().unwrap()
});
pub static ATTR_BINDLESS_SAMPLER_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_bindless_sampler_nv".try_into().unwrap()
});
pub static ATTR_BINDLESS_IMAGE_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_bindless_image_nv".try_into().unwrap()
});
pub static ATTR_BOUND_SAMPLER_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_bound_sampler_nv".try_into().unwrap()
});
pub static ATTR_BOUND_IMAGE_NV: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_bound_image_nv".try_into().unwrap()
});
pub static ATTR_SIMT_CALL_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_simt_call_intel".try_into().unwrap()
});
pub static ATTR_REFERENCED_INDIRECTLY_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_referenced_indirectly_intel".try_into().unwrap()
});
pub static ATTR_CLOBBER_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_clobber_intel".try_into().unwrap()
});
pub static ATTR_SIDE_EFFECTS_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_side_effects_intel".try_into().unwrap()
});
pub static ATTR_VECTOR_COMPUTE_VARIABLE_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_vector_compute_variable_intel".try_into().unwrap()
});
pub static ATTR_FUNC_PARAM_IO_KIND_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_func_param_io_kind_intel".try_into().unwrap()
});
pub static ATTR_VECTOR_COMPUTE_FUNCTION_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_vector_compute_function_intel".try_into().unwrap()
});
pub static ATTR_STACK_CALL_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_stack_call_intel".try_into().unwrap()
});
pub static ATTR_GLOBAL_VARIABLE_OFFSET_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_global_variable_offset_intel".try_into().unwrap()
});
pub static ATTR_USER_SEMANTIC: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_user_semantic".try_into().unwrap()
});
pub static ATTR_USER_TYPE_GOOGLE: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_user_type_google".try_into().unwrap()
});
pub static ATTR_REGISTER_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_register_altera".try_into().unwrap()
});
pub static ATTR_MEMORY_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_memory_altera".try_into().unwrap()
});
pub static ATTR_NUMBANKS_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_numbanks_altera".try_into().unwrap()
});
pub static ATTR_BANKWIDTH_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_bankwidth_altera".try_into().unwrap()
});
pub static ATTR_MAX_PRIVATE_COPIES_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_max_private_copies_altera".try_into().unwrap()
});
pub static ATTR_SINGLEPUMP_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_singlepump_altera".try_into().unwrap()
});
pub static ATTR_DOUBLEPUMP_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_doublepump_altera".try_into().unwrap()
});
pub static ATTR_MAX_REPLICATES_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_max_replicates_altera".try_into().unwrap()
});
pub static ATTR_SIMPLE_DUAL_PORT_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_simple_dual_port_altera".try_into().unwrap()
});
pub static ATTR_FORCE_POW2_DEPTH_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_force_pow2_depth_altera".try_into().unwrap()
});
pub static ATTR_STRIDESIZE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_stridesize_altera".try_into().unwrap()
});
pub static ATTR_WORDSIZE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_wordsize_altera".try_into().unwrap()
});
pub static ATTR_TRUE_DUAL_PORT_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_true_dual_port_altera".try_into().unwrap()
});
pub static ATTR_BURST_COALESCE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_burst_coalesce_altera".try_into().unwrap()
});
pub static ATTR_CACHE_SIZE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_cache_size_altera".try_into().unwrap()
});
pub static ATTR_DONT_STATICALLY_COALESCE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_dont_statically_coalesce_altera".try_into().unwrap()
});
pub static ATTR_PREFETCH_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_prefetch_altera".try_into().unwrap()
});
pub static ATTR_STALL_ENABLE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_stall_enable_altera".try_into().unwrap()
});
pub static ATTR_FUSE_LOOPS_IN_FUNCTION_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_fuse_loops_in_function_altera".try_into().unwrap()
});
pub static ATTR_INITIATION_INTERVAL_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_initiation_interval_altera".try_into().unwrap()
});
pub static ATTR_MAX_CONCURRENCY_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_max_concurrency_altera".try_into().unwrap()
});
pub static ATTR_PIPELINE_ENABLE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_pipeline_enable_altera".try_into().unwrap()
});
pub static ATTR_BUFFER_LOCATION_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_buffer_location_altera".try_into().unwrap()
});
pub static ATTR_IO_PIPE_STORAGE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_io_pipe_storage_altera".try_into().unwrap()
});
pub static ATTR_SINGLE_ELEMENT_VECTOR_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_single_element_vector_intel".try_into().unwrap()
});
pub static ATTR_VECTOR_COMPUTE_CALLABLE_FUNCTION_INTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_vector_compute_callable_function_intel".try_into().unwrap()
});
pub static ATTR_MEDIA_BLOCK_IOINTEL: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_media_block_iointel".try_into().unwrap()
});
pub static ATTR_STALL_FREE_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_stall_free_altera".try_into().unwrap()
});
pub static ATTR_LATENCY_CONTROL_LABEL_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_latency_control_label_altera".try_into().unwrap()
});
pub static ATTR_CONDUIT_KERNEL_ARGUMENT_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_conduit_kernel_argument_altera".try_into().unwrap()
});
pub static ATTR_REGISTER_MAP_KERNEL_ARGUMENT_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_register_map_kernel_argument_altera".try_into().unwrap()
});
pub static ATTR_MM_HOST_INTERFACE_ADDRESS_WIDTH_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_mm_host_interface_address_width_altera".try_into().unwrap()
});
pub static ATTR_MM_HOST_INTERFACE_DATA_WIDTH_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_mm_host_interface_data_width_altera".try_into().unwrap()
});
pub static ATTR_MM_HOST_INTERFACE_LATENCY_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_mm_host_interface_latency_altera".try_into().unwrap()
});
pub static ATTR_MM_HOST_INTERFACE_MAX_BURST_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_mm_host_interface_max_burst_altera".try_into().unwrap()
});
pub static ATTR_MM_HOST_INTERFACE_WAIT_REQUEST_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_mm_host_interface_wait_request_altera".try_into().unwrap()
});
pub static ATTR_STABLE_KERNEL_ARGUMENT_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_stable_kernel_argument_altera".try_into().unwrap()
});
pub static ATTR_IMPLEMENT_IN_REGISTER_MAP_ALTERA: ::pliron::std_deps::sync::LazyLock<
    ::pliron::identifier::Identifier,
> = ::pliron::std_deps::sync::LazyLock::new(|| {
    "spirv_decoration_implement_in_register_map_altera".try_into().unwrap()
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
    fn has_decoration_relaxed_precision(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_relaxed_precision(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_block(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_block(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_buffer_block(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_buffer_block(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_row_major(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_row_major(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_col_major(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_col_major(self, ctx)
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
    fn has_decoration_glsl_shared(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_glsl_shared(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_glsl_packed(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_glsl_packed(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_c_packed(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_c_packed(self, ctx)
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
    fn has_decoration_no_perspective(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_no_perspective(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_flat(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_flat(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_patch(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_patch(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_centroid(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_centroid(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_sample(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_sample(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_invariant(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_invariant(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_restrict(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_restrict(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_aliased(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_aliased(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_volatile(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_volatile(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_constant(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_constant(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_coherent(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_coherent(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_non_writable(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_non_writable(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_non_readable(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_non_readable(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_uniform(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_uniform(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_saturated_conversion(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_saturated_conversion(self, ctx)
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
    fn has_decoration_no_contraction(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_no_contraction(self, ctx)
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
    fn has_decoration_saturated_to_largest_float8_normal_conversion_ext(
        &self,
        ctx: &Context,
    ) -> bool
    where
        Self: Sized,
    {
        has_decoration_saturated_to_largest_float8_normal_conversion_ext(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_no_signed_wrap(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_no_signed_wrap(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_no_unsigned_wrap(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_no_unsigned_wrap(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_weight_texture_qcom(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_weight_texture_qcom(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_block_match_texture_qcom(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_block_match_texture_qcom(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_block_match_sampler_qcom(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_block_match_sampler_qcom(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_explicit_interp_amd(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_explicit_interp_amd(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_track_finish_writing_amdx(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_track_finish_writing_amdx(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_payload_node_sparse_array_amdx(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_payload_node_sparse_array_amdx(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_payload_dispatch_indirect_amdx(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_payload_dispatch_indirect_amdx(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_utf_encoded_khr(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_utf_encoded_khr(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_override_coverage_nv(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_override_coverage_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_passthrough_nv(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_passthrough_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_viewport_relative_nv(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_viewport_relative_nv(self, ctx)
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
    fn has_decoration_per_primitive_ext(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_per_primitive_ext(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_per_view_nv(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_per_view_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_per_task_nv(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_per_task_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_per_vertex_khr(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_per_vertex_khr(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_non_uniform(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_non_uniform(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_restrict_pointer(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_restrict_pointer(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_aliased_pointer(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_aliased_pointer(self, ctx)
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
    fn has_decoration_hit_object_shader_record_buffer_nv(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_hit_object_shader_record_buffer_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_hit_object_shader_record_buffer_ext(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_hit_object_shader_record_buffer_ext(self, ctx)
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
    fn has_decoration_bindless_sampler_nv(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_bindless_sampler_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_bindless_image_nv(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_bindless_image_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_bound_sampler_nv(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_bound_sampler_nv(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_bound_image_nv(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_bound_image_nv(self, ctx)
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
    fn has_decoration_referenced_indirectly_intel(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_referenced_indirectly_intel(self, ctx)
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
    fn has_decoration_side_effects_intel(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_side_effects_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_vector_compute_variable_intel(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_vector_compute_variable_intel(self, ctx)
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
    fn has_decoration_vector_compute_function_intel(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_vector_compute_function_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_stack_call_intel(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_stack_call_intel(self, ctx)
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
    fn has_decoration_register_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_register_altera(self, ctx)
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
    fn has_decoration_singlepump_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_singlepump_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_doublepump_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_doublepump_altera(self, ctx)
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
    fn has_decoration_simple_dual_port_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_simple_dual_port_altera(self, ctx)
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
    fn has_decoration_true_dual_port_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_true_dual_port_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_burst_coalesce_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_burst_coalesce_altera(self, ctx)
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
    fn has_decoration_dont_statically_coalesce_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_dont_statically_coalesce_altera(self, ctx)
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
    fn has_decoration_stall_enable_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_stall_enable_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_fuse_loops_in_function_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_fuse_loops_in_function_altera(self, ctx)
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
    fn has_decoration_single_element_vector_intel(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_single_element_vector_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_vector_compute_callable_function_intel(
        &self,
        ctx: &Context,
    ) -> bool
    where
        Self: Sized,
    {
        has_decoration_vector_compute_callable_function_intel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_media_block_iointel(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_media_block_iointel(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_stall_free_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_stall_free_altera(self, ctx)
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
    fn has_decoration_conduit_kernel_argument_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_conduit_kernel_argument_altera(self, ctx)
    }
    #[allow(non_snake_case)]
    fn has_decoration_register_map_kernel_argument_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_register_map_kernel_argument_altera(self, ctx)
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
    fn has_decoration_stable_kernel_argument_altera(&self, ctx: &Context) -> bool
    where
        Self: Sized,
    {
        has_decoration_stable_kernel_argument_altera(self, ctx)
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
    fn set_decoration_relaxed_precision(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_relaxed_precision(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_block(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_block(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_buffer_block(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_buffer_block(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_row_major(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_row_major(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_col_major(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_col_major(self, ctx);
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
    fn set_decoration_glsl_shared(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_glsl_shared(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_glsl_packed(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_glsl_packed(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_c_packed(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_c_packed(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_built_in(&self, ctx: &Context, value: BuiltInAttr)
    where
        Self: Sized,
    {
        set_decoration_built_in(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_no_perspective(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_no_perspective(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_flat(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_flat(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_patch(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_patch(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_centroid(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_centroid(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_sample(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_sample(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_invariant(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_invariant(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_restrict(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_restrict(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_aliased(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_aliased(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_volatile(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_volatile(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_constant(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_constant(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_coherent(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_coherent(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_non_writable(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_non_writable(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_non_readable(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_non_readable(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_uniform(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_uniform(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_saturated_conversion(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_saturated_conversion(self, ctx);
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
    fn set_decoration_no_contraction(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_no_contraction(self, ctx);
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
    )
    where
        Self: Sized,
    {
        set_decoration_saturated_to_largest_float8_normal_conversion_ext(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_no_signed_wrap(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_no_signed_wrap(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_no_unsigned_wrap(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_no_unsigned_wrap(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_weight_texture_qcom(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_weight_texture_qcom(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_block_match_texture_qcom(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_block_match_texture_qcom(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_block_match_sampler_qcom(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_block_match_sampler_qcom(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_explicit_interp_amd(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_explicit_interp_amd(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_track_finish_writing_amdx(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_track_finish_writing_amdx(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_payload_node_sparse_array_amdx(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_payload_node_sparse_array_amdx(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_payload_dispatch_indirect_amdx(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_payload_dispatch_indirect_amdx(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_utf_encoded_khr(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_utf_encoded_khr(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_override_coverage_nv(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_override_coverage_nv(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_passthrough_nv(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_passthrough_nv(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_viewport_relative_nv(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_viewport_relative_nv(self, ctx);
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
    fn set_decoration_per_primitive_ext(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_per_primitive_ext(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_per_view_nv(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_per_view_nv(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_per_task_nv(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_per_task_nv(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_per_vertex_khr(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_per_vertex_khr(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_non_uniform(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_non_uniform(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_restrict_pointer(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_restrict_pointer(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_aliased_pointer(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_aliased_pointer(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_member_offset_nv(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_member_offset_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_hit_object_shader_record_buffer_nv(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_hit_object_shader_record_buffer_nv(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_hit_object_shader_record_buffer_ext(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_hit_object_shader_record_buffer_ext(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_bank_nv(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_bank_nv(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_bindless_sampler_nv(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_bindless_sampler_nv(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_bindless_image_nv(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_bindless_image_nv(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_bound_sampler_nv(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_bound_sampler_nv(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_bound_image_nv(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_bound_image_nv(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_simt_call_intel(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_simt_call_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_referenced_indirectly_intel(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_referenced_indirectly_intel(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_clobber_intel(&self, ctx: &Context, value: StringAttr)
    where
        Self: Sized,
    {
        set_decoration_clobber_intel(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_side_effects_intel(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_side_effects_intel(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_vector_compute_variable_intel(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_vector_compute_variable_intel(self, ctx);
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
    fn set_decoration_vector_compute_function_intel(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_vector_compute_function_intel(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_stack_call_intel(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_stack_call_intel(self, ctx);
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
    fn set_decoration_register_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_register_altera(self, ctx);
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
    fn set_decoration_singlepump_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_singlepump_altera(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_doublepump_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_doublepump_altera(self, ctx);
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
    fn set_decoration_simple_dual_port_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_simple_dual_port_altera(self, ctx);
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
    fn set_decoration_true_dual_port_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_true_dual_port_altera(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_burst_coalesce_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_burst_coalesce_altera(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_cache_size_altera(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_cache_size_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_dont_statically_coalesce_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_dont_statically_coalesce_altera(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_prefetch_altera(&self, ctx: &Context, value: LiteralIntegerAttr)
    where
        Self: Sized,
    {
        set_decoration_prefetch_altera(self, ctx, value);
    }
    #[allow(non_snake_case)]
    fn set_decoration_stall_enable_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_stall_enable_altera(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_fuse_loops_in_function_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_fuse_loops_in_function_altera(self, ctx);
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
    fn set_decoration_single_element_vector_intel(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_single_element_vector_intel(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_vector_compute_callable_function_intel(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_vector_compute_callable_function_intel(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_media_block_iointel(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_media_block_iointel(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_stall_free_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_stall_free_altera(self, ctx);
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
    fn set_decoration_conduit_kernel_argument_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_conduit_kernel_argument_altera(self, ctx);
    }
    #[allow(non_snake_case)]
    fn set_decoration_register_map_kernel_argument_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_register_map_kernel_argument_altera(self, ctx);
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
    fn set_decoration_stable_kernel_argument_altera(&self, ctx: &Context)
    where
        Self: Sized,
    {
        set_decoration_stable_kernel_argument_altera(self, ctx);
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
pub fn has_decoration_relaxed_precision(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_RELAXED_PRECISION)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_block(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_BLOCK)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_buffer_block(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_BUFFER_BLOCK)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_row_major(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_ROW_MAJOR)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_col_major(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_COL_MAJOR)
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
pub fn has_decoration_glsl_shared(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_GLSL_SHARED)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_glsl_packed(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_GLSL_PACKED)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_c_packed(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_C_PACKED)
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
pub fn has_decoration_no_perspective(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_NO_PERSPECTIVE)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_flat(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_FLAT)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_patch(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_PATCH)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_centroid(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_CENTROID)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_sample(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_SAMPLE)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_invariant(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_INVARIANT)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_restrict(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_RESTRICT)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_aliased(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_ALIASED)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_volatile(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_VOLATILE)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_constant(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_CONSTANT)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_coherent(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_COHERENT)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_non_writable(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_NON_WRITABLE)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_non_readable(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_NON_READABLE)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_uniform(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_UNIFORM)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_saturated_conversion(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_SATURATED_CONVERSION)
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
pub fn has_decoration_no_contraction(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_NO_CONTRACTION)
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
pub fn has_decoration_saturated_to_largest_float8_normal_conversion_ext(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes
        .0
        .contains_key(&*ATTR_SATURATED_TO_LARGEST_FLOAT8_NORMAL_CONVERSION_EXT)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_no_signed_wrap(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_NO_SIGNED_WRAP)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_no_unsigned_wrap(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_NO_UNSIGNED_WRAP)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_weight_texture_qcom(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_WEIGHT_TEXTURE_QCOM)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_block_match_texture_qcom(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_BLOCK_MATCH_TEXTURE_QCOM)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_block_match_sampler_qcom(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_BLOCK_MATCH_SAMPLER_QCOM)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_explicit_interp_amd(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_EXPLICIT_INTERP_AMD)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_track_finish_writing_amdx(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_TRACK_FINISH_WRITING_AMDX)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_payload_node_sparse_array_amdx(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_PAYLOAD_NODE_SPARSE_ARRAY_AMDX)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_payload_dispatch_indirect_amdx(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_PAYLOAD_DISPATCH_INDIRECT_AMDX)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_utf_encoded_khr(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_UTF_ENCODED_KHR)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_override_coverage_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_OVERRIDE_COVERAGE_NV)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_passthrough_nv(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_PASSTHROUGH_NV)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_viewport_relative_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_VIEWPORT_RELATIVE_NV)
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
pub fn has_decoration_per_primitive_ext(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_PER_PRIMITIVE_EXT)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_per_view_nv(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_PER_VIEW_NV)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_per_task_nv(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_PER_TASK_NV)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_per_vertex_khr(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_PER_VERTEX_KHR)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_non_uniform(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_NON_UNIFORM)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_restrict_pointer(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_RESTRICT_POINTER)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_aliased_pointer(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_ALIASED_POINTER)
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
pub fn has_decoration_hit_object_shader_record_buffer_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_NV)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_hit_object_shader_record_buffer_ext(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_EXT)
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
pub fn has_decoration_bindless_sampler_nv(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_BINDLESS_SAMPLER_NV)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_bindless_image_nv(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_BINDLESS_IMAGE_NV)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_bound_sampler_nv(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_BOUND_SAMPLER_NV)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_bound_image_nv(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_BOUND_IMAGE_NV)
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
pub fn has_decoration_referenced_indirectly_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_REFERENCED_INDIRECTLY_INTEL)
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
pub fn has_decoration_side_effects_intel(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_SIDE_EFFECTS_INTEL)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_vector_compute_variable_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_VECTOR_COMPUTE_VARIABLE_INTEL)
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
pub fn has_decoration_vector_compute_function_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_VECTOR_COMPUTE_FUNCTION_INTEL)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_stack_call_intel(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_STACK_CALL_INTEL)
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
pub fn has_decoration_register_altera(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_REGISTER_ALTERA)
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
pub fn has_decoration_singlepump_altera(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_SINGLEPUMP_ALTERA)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_doublepump_altera(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_DOUBLEPUMP_ALTERA)
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
pub fn has_decoration_simple_dual_port_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_SIMPLE_DUAL_PORT_ALTERA)
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
pub fn has_decoration_true_dual_port_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_TRUE_DUAL_PORT_ALTERA)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_burst_coalesce_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_BURST_COALESCE_ALTERA)
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
pub fn has_decoration_dont_statically_coalesce_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_DONT_STATICALLY_COALESCE_ALTERA)
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
pub fn has_decoration_stall_enable_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_STALL_ENABLE_ALTERA)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_fuse_loops_in_function_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_FUSE_LOOPS_IN_FUNCTION_ALTERA)
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
pub fn has_decoration_single_element_vector_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_SINGLE_ELEMENT_VECTOR_INTEL)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_vector_compute_callable_function_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_VECTOR_COMPUTE_CALLABLE_FUNCTION_INTEL)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_media_block_iointel(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_MEDIA_BLOCK_IOINTEL)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_stall_free_altera(op: &dyn DecoratableOp, ctx: &Context) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_STALL_FREE_ALTERA)
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
pub fn has_decoration_conduit_kernel_argument_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_CONDUIT_KERNEL_ARGUMENT_ALTERA)
}
#[allow(non_snake_case)]
#[inline(never)]
pub fn has_decoration_register_map_kernel_argument_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_REGISTER_MAP_KERNEL_ARGUMENT_ALTERA)
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
pub fn has_decoration_stable_kernel_argument_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
) -> bool {
    let op = op.get_operation().deref(ctx);
    op.attributes.0.contains_key(&*ATTR_STABLE_KERNEL_ARGUMENT_ALTERA)
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
pub fn set_decoration_relaxed_precision(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_RELAXED_PRECISION.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_block(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BLOCK.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_buffer_block(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BUFFER_BLOCK.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_row_major(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_ROW_MAJOR.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_col_major(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_COL_MAJOR.clone(), UnitAttr::new());
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
pub fn set_decoration_glsl_shared(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_GLSL_SHARED.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_glsl_packed(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_GLSL_PACKED.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_c_packed(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_C_PACKED.clone(), UnitAttr::new());
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
pub fn set_decoration_no_perspective(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_NO_PERSPECTIVE.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_flat(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation().deref_mut(ctx).attributes.set(ATTR_FLAT.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_patch(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PATCH.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_centroid(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_CENTROID.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_sample(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SAMPLE.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_invariant(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_INVARIANT.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_restrict(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_RESTRICT.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_aliased(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_ALIASED.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_volatile(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_VOLATILE.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_constant(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_CONSTANT.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_coherent(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_COHERENT.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_non_writable(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_NON_WRITABLE.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_non_readable(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_NON_READABLE.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_uniform(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_UNIFORM.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_saturated_conversion(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SATURATED_CONVERSION.clone(), UnitAttr::new());
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
pub fn set_decoration_no_contraction(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_NO_CONTRACTION.clone(), UnitAttr::new());
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
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(
            ATTR_SATURATED_TO_LARGEST_FLOAT8_NORMAL_CONVERSION_EXT.clone(),
            UnitAttr::new(),
        );
}
#[allow(non_snake_case)]
pub fn set_decoration_no_signed_wrap(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_NO_SIGNED_WRAP.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_no_unsigned_wrap(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_NO_UNSIGNED_WRAP.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_weight_texture_qcom(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_WEIGHT_TEXTURE_QCOM.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_block_match_texture_qcom(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BLOCK_MATCH_TEXTURE_QCOM.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_block_match_sampler_qcom(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BLOCK_MATCH_SAMPLER_QCOM.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_explicit_interp_amd(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_EXPLICIT_INTERP_AMD.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_track_finish_writing_amdx(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_TRACK_FINISH_WRITING_AMDX.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_payload_node_sparse_array_amdx(
    op: &dyn DecoratableOp,
    ctx: &Context,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PAYLOAD_NODE_SPARSE_ARRAY_AMDX.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_payload_dispatch_indirect_amdx(
    op: &dyn DecoratableOp,
    ctx: &Context,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PAYLOAD_DISPATCH_INDIRECT_AMDX.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_utf_encoded_khr(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_UTF_ENCODED_KHR.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_override_coverage_nv(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_OVERRIDE_COVERAGE_NV.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_passthrough_nv(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PASSTHROUGH_NV.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_viewport_relative_nv(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_VIEWPORT_RELATIVE_NV.clone(), UnitAttr::new());
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
pub fn set_decoration_per_primitive_ext(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PER_PRIMITIVE_EXT.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_per_view_nv(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PER_VIEW_NV.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_per_task_nv(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PER_TASK_NV.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_per_vertex_khr(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_PER_VERTEX_KHR.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_non_uniform(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_NON_UNIFORM.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_restrict_pointer(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_RESTRICT_POINTER.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_aliased_pointer(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_ALIASED_POINTER.clone(), UnitAttr::new());
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
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_NV.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_hit_object_shader_record_buffer_ext(
    op: &dyn DecoratableOp,
    ctx: &Context,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_HIT_OBJECT_SHADER_RECORD_BUFFER_EXT.clone(), UnitAttr::new());
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
pub fn set_decoration_bindless_sampler_nv(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BINDLESS_SAMPLER_NV.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_bindless_image_nv(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BINDLESS_IMAGE_NV.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_bound_sampler_nv(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BOUND_SAMPLER_NV.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_bound_image_nv(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BOUND_IMAGE_NV.clone(), UnitAttr::new());
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
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_REFERENCED_INDIRECTLY_INTEL.clone(), UnitAttr::new());
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
pub fn set_decoration_side_effects_intel(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SIDE_EFFECTS_INTEL.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_vector_compute_variable_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_VECTOR_COMPUTE_VARIABLE_INTEL.clone(), UnitAttr::new());
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
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_VECTOR_COMPUTE_FUNCTION_INTEL.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_stack_call_intel(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_STACK_CALL_INTEL.clone(), UnitAttr::new());
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
pub fn set_decoration_register_altera(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_REGISTER_ALTERA.clone(), UnitAttr::new());
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
pub fn set_decoration_singlepump_altera(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SINGLEPUMP_ALTERA.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_doublepump_altera(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_DOUBLEPUMP_ALTERA.clone(), UnitAttr::new());
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
pub fn set_decoration_simple_dual_port_altera(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SIMPLE_DUAL_PORT_ALTERA.clone(), UnitAttr::new());
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
pub fn set_decoration_true_dual_port_altera(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_TRUE_DUAL_PORT_ALTERA.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_burst_coalesce_altera(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_BURST_COALESCE_ALTERA.clone(), UnitAttr::new());
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
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_DONT_STATICALLY_COALESCE_ALTERA.clone(), UnitAttr::new());
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
pub fn set_decoration_stall_enable_altera(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_STALL_ENABLE_ALTERA.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_fuse_loops_in_function_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_FUSE_LOOPS_IN_FUNCTION_ALTERA.clone(), UnitAttr::new());
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
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_SINGLE_ELEMENT_VECTOR_INTEL.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_vector_compute_callable_function_intel(
    op: &dyn DecoratableOp,
    ctx: &Context,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_VECTOR_COMPUTE_CALLABLE_FUNCTION_INTEL.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_media_block_iointel(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_MEDIA_BLOCK_IOINTEL.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_stall_free_altera(op: &dyn DecoratableOp, ctx: &Context) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_STALL_FREE_ALTERA.clone(), UnitAttr::new());
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
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_CONDUIT_KERNEL_ARGUMENT_ALTERA.clone(), UnitAttr::new());
}
#[allow(non_snake_case)]
pub fn set_decoration_register_map_kernel_argument_altera(
    op: &dyn DecoratableOp,
    ctx: &Context,
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_REGISTER_MAP_KERNEL_ARGUMENT_ALTERA.clone(), UnitAttr::new());
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
) {
    op.get_operation()
        .deref_mut(ctx)
        .attributes
        .set(ATTR_STABLE_KERNEL_ARGUMENT_ALTERA.clone(), UnitAttr::new());
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
        "spirv_decoration_relaxed_precision" => Some(Decoration::RelaxedPrecision),
        "spirv_decoration_block" => Some(Decoration::Block),
        "spirv_decoration_buffer_block" => Some(Decoration::BufferBlock),
        "spirv_decoration_row_major" => Some(Decoration::RowMajor),
        "spirv_decoration_col_major" => Some(Decoration::ColMajor),
        "spirv_decoration_array_stride" => Some(Decoration::ArrayStride),
        "spirv_decoration_matrix_stride" => Some(Decoration::MatrixStride),
        "spirv_decoration_glsl_shared" => Some(Decoration::GLSLShared),
        "spirv_decoration_glsl_packed" => Some(Decoration::GLSLPacked),
        "spirv_decoration_c_packed" => Some(Decoration::CPacked),
        "spirv_decoration_built_in" => Some(Decoration::BuiltIn),
        "spirv_decoration_no_perspective" => Some(Decoration::NoPerspective),
        "spirv_decoration_flat" => Some(Decoration::Flat),
        "spirv_decoration_patch" => Some(Decoration::Patch),
        "spirv_decoration_centroid" => Some(Decoration::Centroid),
        "spirv_decoration_sample" => Some(Decoration::Sample),
        "spirv_decoration_invariant" => Some(Decoration::Invariant),
        "spirv_decoration_restrict" => Some(Decoration::Restrict),
        "spirv_decoration_aliased" => Some(Decoration::Aliased),
        "spirv_decoration_volatile" => Some(Decoration::Volatile),
        "spirv_decoration_constant" => Some(Decoration::Constant),
        "spirv_decoration_coherent" => Some(Decoration::Coherent),
        "spirv_decoration_non_writable" => Some(Decoration::NonWritable),
        "spirv_decoration_non_readable" => Some(Decoration::NonReadable),
        "spirv_decoration_uniform" => Some(Decoration::Uniform),
        "spirv_decoration_saturated_conversion" => Some(Decoration::SaturatedConversion),
        "spirv_decoration_stream" => Some(Decoration::Stream),
        "spirv_decoration_location" => Some(Decoration::Location),
        "spirv_decoration_component" => Some(Decoration::Component),
        "spirv_decoration_index" => Some(Decoration::Index),
        "spirv_decoration_binding" => Some(Decoration::Binding),
        "spirv_decoration_descriptor_set" => Some(Decoration::DescriptorSet),
        "spirv_decoration_offset" => Some(Decoration::Offset),
        "spirv_decoration_xfb_buffer" => Some(Decoration::XfbBuffer),
        "spirv_decoration_xfb_stride" => Some(Decoration::XfbStride),
        "spirv_decoration_func_param_attr" => Some(Decoration::FuncParamAttr),
        "spirv_decoration_fp_rounding_mode" => Some(Decoration::FPRoundingMode),
        "spirv_decoration_fp_fast_math_mode" => Some(Decoration::FPFastMathMode),
        "spirv_decoration_no_contraction" => Some(Decoration::NoContraction),
        "spirv_decoration_input_attachment_index" => {
            Some(Decoration::InputAttachmentIndex)
        }
        "spirv_decoration_alignment" => Some(Decoration::Alignment),
        "spirv_decoration_max_byte_offset" => Some(Decoration::MaxByteOffset),
        "spirv_decoration_saturated_to_largest_float8_normal_conversion_ext" => {
            Some(Decoration::SaturatedToLargestFloat8NormalConversionEXT)
        }
        "spirv_decoration_no_signed_wrap" => Some(Decoration::NoSignedWrap),
        "spirv_decoration_no_unsigned_wrap" => Some(Decoration::NoUnsignedWrap),
        "spirv_decoration_weight_texture_qcom" => Some(Decoration::WeightTextureQCOM),
        "spirv_decoration_block_match_texture_qcom" => {
            Some(Decoration::BlockMatchTextureQCOM)
        }
        "spirv_decoration_block_match_sampler_qcom" => {
            Some(Decoration::BlockMatchSamplerQCOM)
        }
        "spirv_decoration_explicit_interp_amd" => Some(Decoration::ExplicitInterpAMD),
        "spirv_decoration_track_finish_writing_amdx" => {
            Some(Decoration::TrackFinishWritingAMDX)
        }
        "spirv_decoration_payload_node_sparse_array_amdx" => {
            Some(Decoration::PayloadNodeSparseArrayAMDX)
        }
        "spirv_decoration_payload_dispatch_indirect_amdx" => {
            Some(Decoration::PayloadDispatchIndirectAMDX)
        }
        "spirv_decoration_utf_encoded_khr" => Some(Decoration::UTFEncodedKHR),
        "spirv_decoration_override_coverage_nv" => Some(Decoration::OverrideCoverageNV),
        "spirv_decoration_passthrough_nv" => Some(Decoration::PassthroughNV),
        "spirv_decoration_viewport_relative_nv" => Some(Decoration::ViewportRelativeNV),
        "spirv_decoration_secondary_viewport_relative_nv" => {
            Some(Decoration::SecondaryViewportRelativeNV)
        }
        "spirv_decoration_per_primitive_ext" => Some(Decoration::PerPrimitiveEXT),
        "spirv_decoration_per_view_nv" => Some(Decoration::PerViewNV),
        "spirv_decoration_per_task_nv" => Some(Decoration::PerTaskNV),
        "spirv_decoration_per_vertex_khr" => Some(Decoration::PerVertexKHR),
        "spirv_decoration_non_uniform" => Some(Decoration::NonUniform),
        "spirv_decoration_restrict_pointer" => Some(Decoration::RestrictPointer),
        "spirv_decoration_aliased_pointer" => Some(Decoration::AliasedPointer),
        "spirv_decoration_member_offset_nv" => Some(Decoration::MemberOffsetNV),
        "spirv_decoration_hit_object_shader_record_buffer_nv" => {
            Some(Decoration::HitObjectShaderRecordBufferNV)
        }
        "spirv_decoration_hit_object_shader_record_buffer_ext" => {
            Some(Decoration::HitObjectShaderRecordBufferEXT)
        }
        "spirv_decoration_bank_nv" => Some(Decoration::BankNV),
        "spirv_decoration_bindless_sampler_nv" => Some(Decoration::BindlessSamplerNV),
        "spirv_decoration_bindless_image_nv" => Some(Decoration::BindlessImageNV),
        "spirv_decoration_bound_sampler_nv" => Some(Decoration::BoundSamplerNV),
        "spirv_decoration_bound_image_nv" => Some(Decoration::BoundImageNV),
        "spirv_decoration_simt_call_intel" => Some(Decoration::SIMTCallINTEL),
        "spirv_decoration_referenced_indirectly_intel" => {
            Some(Decoration::ReferencedIndirectlyINTEL)
        }
        "spirv_decoration_clobber_intel" => Some(Decoration::ClobberINTEL),
        "spirv_decoration_side_effects_intel" => Some(Decoration::SideEffectsINTEL),
        "spirv_decoration_vector_compute_variable_intel" => {
            Some(Decoration::VectorComputeVariableINTEL)
        }
        "spirv_decoration_func_param_io_kind_intel" => {
            Some(Decoration::FuncParamIOKindINTEL)
        }
        "spirv_decoration_vector_compute_function_intel" => {
            Some(Decoration::VectorComputeFunctionINTEL)
        }
        "spirv_decoration_stack_call_intel" => Some(Decoration::StackCallINTEL),
        "spirv_decoration_global_variable_offset_intel" => {
            Some(Decoration::GlobalVariableOffsetINTEL)
        }
        "spirv_decoration_user_semantic" => Some(Decoration::UserSemantic),
        "spirv_decoration_user_type_google" => Some(Decoration::UserTypeGOOGLE),
        "spirv_decoration_register_altera" => Some(Decoration::RegisterALTERA),
        "spirv_decoration_memory_altera" => Some(Decoration::MemoryALTERA),
        "spirv_decoration_numbanks_altera" => Some(Decoration::NumbanksALTERA),
        "spirv_decoration_bankwidth_altera" => Some(Decoration::BankwidthALTERA),
        "spirv_decoration_max_private_copies_altera" => {
            Some(Decoration::MaxPrivateCopiesALTERA)
        }
        "spirv_decoration_singlepump_altera" => Some(Decoration::SinglepumpALTERA),
        "spirv_decoration_doublepump_altera" => Some(Decoration::DoublepumpALTERA),
        "spirv_decoration_max_replicates_altera" => Some(Decoration::MaxReplicatesALTERA),
        "spirv_decoration_simple_dual_port_altera" => {
            Some(Decoration::SimpleDualPortALTERA)
        }
        "spirv_decoration_force_pow2_depth_altera" => {
            Some(Decoration::ForcePow2DepthALTERA)
        }
        "spirv_decoration_stridesize_altera" => Some(Decoration::StridesizeALTERA),
        "spirv_decoration_wordsize_altera" => Some(Decoration::WordsizeALTERA),
        "spirv_decoration_true_dual_port_altera" => Some(Decoration::TrueDualPortALTERA),
        "spirv_decoration_burst_coalesce_altera" => Some(Decoration::BurstCoalesceALTERA),
        "spirv_decoration_cache_size_altera" => Some(Decoration::CacheSizeALTERA),
        "spirv_decoration_dont_statically_coalesce_altera" => {
            Some(Decoration::DontStaticallyCoalesceALTERA)
        }
        "spirv_decoration_prefetch_altera" => Some(Decoration::PrefetchALTERA),
        "spirv_decoration_stall_enable_altera" => Some(Decoration::StallEnableALTERA),
        "spirv_decoration_fuse_loops_in_function_altera" => {
            Some(Decoration::FuseLoopsInFunctionALTERA)
        }
        "spirv_decoration_initiation_interval_altera" => {
            Some(Decoration::InitiationIntervalALTERA)
        }
        "spirv_decoration_max_concurrency_altera" => {
            Some(Decoration::MaxConcurrencyALTERA)
        }
        "spirv_decoration_pipeline_enable_altera" => {
            Some(Decoration::PipelineEnableALTERA)
        }
        "spirv_decoration_buffer_location_altera" => {
            Some(Decoration::BufferLocationALTERA)
        }
        "spirv_decoration_io_pipe_storage_altera" => {
            Some(Decoration::IOPipeStorageALTERA)
        }
        "spirv_decoration_single_element_vector_intel" => {
            Some(Decoration::SingleElementVectorINTEL)
        }
        "spirv_decoration_vector_compute_callable_function_intel" => {
            Some(Decoration::VectorComputeCallableFunctionINTEL)
        }
        "spirv_decoration_media_block_iointel" => Some(Decoration::MediaBlockIOINTEL),
        "spirv_decoration_stall_free_altera" => Some(Decoration::StallFreeALTERA),
        "spirv_decoration_latency_control_label_altera" => {
            Some(Decoration::LatencyControlLabelALTERA)
        }
        "spirv_decoration_conduit_kernel_argument_altera" => {
            Some(Decoration::ConduitKernelArgumentALTERA)
        }
        "spirv_decoration_register_map_kernel_argument_altera" => {
            Some(Decoration::RegisterMapKernelArgumentALTERA)
        }
        "spirv_decoration_mm_host_interface_address_width_altera" => {
            Some(Decoration::MMHostInterfaceAddressWidthALTERA)
        }
        "spirv_decoration_mm_host_interface_data_width_altera" => {
            Some(Decoration::MMHostInterfaceDataWidthALTERA)
        }
        "spirv_decoration_mm_host_interface_latency_altera" => {
            Some(Decoration::MMHostInterfaceLatencyALTERA)
        }
        "spirv_decoration_mm_host_interface_max_burst_altera" => {
            Some(Decoration::MMHostInterfaceMaxBurstALTERA)
        }
        "spirv_decoration_mm_host_interface_wait_request_altera" => {
            Some(Decoration::MMHostInterfaceWaitRequestALTERA)
        }
        "spirv_decoration_stable_kernel_argument_altera" => {
            Some(Decoration::StableKernelArgumentALTERA)
        }
        "spirv_decoration_implement_in_register_map_altera" => {
            Some(Decoration::ImplementInRegisterMapALTERA)
        }
        _ => None,
    }
}
impl DecorationInfo {
    pub fn as_operands(&self) -> Vec<Operand> {
        match self.decoration {
            Decoration::FPRoundingMode => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<FPRoundingModeAttr>().unwrap();
                vec![Operand::FPRoundingMode(attr.0)]
            }
            Decoration::ClobberINTEL
            | Decoration::UserSemantic
            | Decoration::UserTypeGOOGLE
            | Decoration::MemoryALTERA => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<StringAttr>().unwrap();
                vec![Operand::LiteralString(attr.as_str().to_string())]
            }
            Decoration::FPFastMathMode => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<FPFastMathModeAttr>().unwrap();
                vec![Operand::FPFastMathMode(attr.0)]
            }
            Decoration::FuncParamAttr => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<VecAttr>().unwrap();
                todo!()
            }
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
            Decoration::BuiltIn => {
                #[allow(unused)]
                let attr = self.value.downcast_ref::<BuiltInAttr>().unwrap();
                vec![Operand::BuiltIn(attr.0)]
            }
            _ => unimplemented!("Unsupported decoration"),
        }
    }
}
