// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use crate::{
    dr::{self, Builder},
    spirv,
};
impl Builder {
    #[allow(clippy::too_many_arguments)]
    pub fn debug_printf(
        &mut self,
        format: spirv::Word,
        id_ref: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        self.debug_printf_id(None, format, id_ref)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn debug_printf_id(
        &mut self,
        result_id: Option<spirv::Word>,
        format: spirv::Word,
        id_ref: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "NonSemantic.DebugPrintf");
        let result_type = self.type_void();
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(format)];
        args.extend(id_ref.into_iter().map(dr::Operand::IdRef));
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::DebugPrintfOp::DebugPrintf as spirv::Word,
            args,
        )
    }
}
