# Pliron-SPIRV

Pliron dialect for SPIR-V. Mostly auto-generated from the SPIR-V headers.

## Debug info

`PlironBuilder::with_debug_info` converts the location of each op to SPIR-V debug data. The format
is core `OpLine`, or `NonSemantic.Shader.DebugInfo.100` with inlined frames. See the `debug_info`
module for the rules.
