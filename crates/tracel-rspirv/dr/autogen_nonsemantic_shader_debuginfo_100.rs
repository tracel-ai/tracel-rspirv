// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use crate::{
    dr::{self, Builder},
    spirv,
};
impl Builder {
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_info_none(&mut self) -> spirv::Word {
        self.shader_debug_info_none_id(None)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_info_none_id(&mut self, result_id: Option<spirv::Word>) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugInfoNone as spirv::Word,
                ),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_compilation_unit(
        &mut self,
        version: spirv::Word,
        dwarf_version: spirv::Word,
        source: spirv::Word,
        language: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_compilation_unit_id(None, version, dwarf_version, source, language)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_compilation_unit_id(
        &mut self,
        result_id: Option<spirv::Word>,
        version: spirv::Word,
        dwarf_version: spirv::Word,
        source: spirv::Word,
        language: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugCompilationUnit as spirv::Word,
                ),
                dr::Operand::IdRef(version),
                dr::Operand::IdRef(dwarf_version),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(language),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_basic(
        &mut self,
        name: spirv::Word,
        size: spirv::Word,
        encoding: spirv::Word,
        flags: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_type_basic_id(None, name, size, encoding, flags)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_basic_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        size: spirv::Word,
        encoding: spirv::Word,
        flags: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeBasic as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(size),
                dr::Operand::IdRef(encoding),
                dr::Operand::IdRef(flags),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_pointer(
        &mut self,
        base_type: spirv::Word,
        storage_class: spirv::Word,
        flags: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_type_pointer_id(None, base_type, storage_class, flags)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_pointer_id(
        &mut self,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        storage_class: spirv::Word,
        flags: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypePointer as spirv::Word,
                ),
                dr::Operand::IdRef(base_type),
                dr::Operand::IdRef(storage_class),
                dr::Operand::IdRef(flags),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_qualifier(
        &mut self,
        base_type: spirv::Word,
        type_qualifier: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_type_qualifier_id(None, base_type, type_qualifier)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_qualifier_id(
        &mut self,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        type_qualifier: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeQualifier as spirv::Word,
                ),
                dr::Operand::IdRef(base_type),
                dr::Operand::IdRef(type_qualifier),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_array(
        &mut self,
        base_type: spirv::Word,
        component_counts: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_type_array_id(None, base_type, component_counts)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_array_id(
        &mut self,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        component_counts: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeArray as spirv::Word,
                ),
                dr::Operand::IdRef(base_type),
            ],
        );
        inst.operands
            .extend(component_counts.into_iter().map(dr::Operand::IdRef));
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_vector(
        &mut self,
        base_type: spirv::Word,
        component_count: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_type_vector_id(None, base_type, component_count)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_vector_id(
        &mut self,
        result_id: Option<spirv::Word>,
        base_type: spirv::Word,
        component_count: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeVector as spirv::Word,
                ),
                dr::Operand::IdRef(base_type),
                dr::Operand::IdRef(component_count),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_typedef(
        &mut self,
        name: spirv::Word,
        base_type: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_typedef_id(None, name, base_type, source, line, column, parent)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_typedef_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        base_type: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypedef as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(base_type),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
                dr::Operand::IdRef(parent),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_function(
        &mut self,
        flags: spirv::Word,
        return_type: spirv::Word,
        parameter_types: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_type_function_id(None, flags, return_type, parameter_types)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_function_id(
        &mut self,
        result_id: Option<spirv::Word>,
        flags: spirv::Word,
        return_type: spirv::Word,
        parameter_types: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeFunction as spirv::Word,
                ),
                dr::Operand::IdRef(flags),
                dr::Operand::IdRef(return_type),
            ],
        );
        inst.operands
            .extend(parameter_types.into_iter().map(dr::Operand::IdRef));
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_enum(
        &mut self,
        name: spirv::Word,
        underlying_type: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        size: spirv::Word,
        flags: spirv::Word,
        value_name_value_name: impl IntoIterator<Item = (spirv::Word, spirv::Word)>,
    ) -> spirv::Word {
        self.shader_debug_type_enum_id(
            None,
            name,
            underlying_type,
            source,
            line,
            column,
            parent,
            size,
            flags,
            value_name_value_name,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_enum_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        underlying_type: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        size: spirv::Word,
        flags: spirv::Word,
        value_name_value_name: impl IntoIterator<Item = (spirv::Word, spirv::Word)>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeEnum as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(underlying_type),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
                dr::Operand::IdRef(parent),
                dr::Operand::IdRef(size),
                dr::Operand::IdRef(flags),
            ],
        );
        for v in value_name_value_name {
            inst.operands.push(dr::Operand::IdRef(v.0));
            inst.operands.push(dr::Operand::IdRef(v.1));
        }
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_composite(
        &mut self,
        name: spirv::Word,
        tag: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        linkage_name: spirv::Word,
        size: spirv::Word,
        flags: spirv::Word,
        members: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_type_composite_id(
            None,
            name,
            tag,
            source,
            line,
            column,
            parent,
            linkage_name,
            size,
            flags,
            members,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_composite_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        tag: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        linkage_name: spirv::Word,
        size: spirv::Word,
        flags: spirv::Word,
        members: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeComposite as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(tag),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
                dr::Operand::IdRef(parent),
                dr::Operand::IdRef(linkage_name),
                dr::Operand::IdRef(size),
                dr::Operand::IdRef(flags),
            ],
        );
        inst.operands
            .extend(members.into_iter().map(dr::Operand::IdRef));
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_member(
        &mut self,
        name: spirv::Word,
        ty: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        offset: spirv::Word,
        size: spirv::Word,
        flags: spirv::Word,
        value: Option<spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_type_member_id(
            None, name, ty, source, line, column, offset, size, flags, value,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_member_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        ty: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        offset: spirv::Word,
        size: spirv::Word,
        flags: spirv::Word,
        value: Option<spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeMember as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(ty),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(size),
                dr::Operand::IdRef(flags),
            ],
        );
        if let Some(v) = value {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_inheritance(
        &mut self,
        parent: spirv::Word,
        offset: spirv::Word,
        size: spirv::Word,
        flags: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_type_inheritance_id(None, parent, offset, size, flags)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_inheritance_id(
        &mut self,
        result_id: Option<spirv::Word>,
        parent: spirv::Word,
        offset: spirv::Word,
        size: spirv::Word,
        flags: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeInheritance as spirv::Word,
                ),
                dr::Operand::IdRef(parent),
                dr::Operand::IdRef(offset),
                dr::Operand::IdRef(size),
                dr::Operand::IdRef(flags),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_ptr_to_member(
        &mut self,
        member_type: spirv::Word,
        parent: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_type_ptr_to_member_id(None, member_type, parent)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_ptr_to_member_id(
        &mut self,
        result_id: Option<spirv::Word>,
        member_type: spirv::Word,
        parent: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypePtrToMember as spirv::Word,
                ),
                dr::Operand::IdRef(member_type),
                dr::Operand::IdRef(parent),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_template(
        &mut self,
        target: spirv::Word,
        parameters: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_type_template_id(None, target, parameters)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_template_id(
        &mut self,
        result_id: Option<spirv::Word>,
        target: spirv::Word,
        parameters: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeTemplate as spirv::Word,
                ),
                dr::Operand::IdRef(target),
            ],
        );
        inst.operands
            .extend(parameters.into_iter().map(dr::Operand::IdRef));
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_template_parameter(
        &mut self,
        name: spirv::Word,
        actual_type: spirv::Word,
        value: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_type_template_parameter_id(
            None,
            name,
            actual_type,
            value,
            source,
            line,
            column,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_template_parameter_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        actual_type: spirv::Word,
        value: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeTemplateParameter as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(actual_type),
                dr::Operand::IdRef(value),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_template_template_parameter(
        &mut self,
        name: spirv::Word,
        template_name: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_type_template_template_parameter_id(
            None,
            name,
            template_name,
            source,
            line,
            column,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_template_template_parameter_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        template_name: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeTemplateTemplateParameter as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(template_name),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_template_parameter_pack(
        &mut self,
        name: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        template_parameters: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_type_template_parameter_pack_id(
            None,
            name,
            source,
            line,
            column,
            template_parameters,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_template_parameter_pack_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        template_parameters: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeTemplateParameterPack as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
            ],
        );
        inst.operands
            .extend(template_parameters.into_iter().map(dr::Operand::IdRef));
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_global_variable(
        &mut self,
        name: spirv::Word,
        ty: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        linkage_name: spirv::Word,
        variable: spirv::Word,
        flags: spirv::Word,
        static_member_declaration: Option<spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_global_variable_id(
            None,
            name,
            ty,
            source,
            line,
            column,
            parent,
            linkage_name,
            variable,
            flags,
            static_member_declaration,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_global_variable_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        ty: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        linkage_name: spirv::Word,
        variable: spirv::Word,
        flags: spirv::Word,
        static_member_declaration: Option<spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugGlobalVariable as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(ty),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
                dr::Operand::IdRef(parent),
                dr::Operand::IdRef(linkage_name),
                dr::Operand::IdRef(variable),
                dr::Operand::IdRef(flags),
            ],
        );
        if let Some(v) = static_member_declaration {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_function_declaration(
        &mut self,
        name: spirv::Word,
        ty: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        linkage_name: spirv::Word,
        flags: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_function_declaration_id(
            None,
            name,
            ty,
            source,
            line,
            column,
            parent,
            linkage_name,
            flags,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_function_declaration_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        ty: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        linkage_name: spirv::Word,
        flags: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugFunctionDeclaration as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(ty),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
                dr::Operand::IdRef(parent),
                dr::Operand::IdRef(linkage_name),
                dr::Operand::IdRef(flags),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_function(
        &mut self,
        name: spirv::Word,
        ty: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        linkage_name: spirv::Word,
        flags: spirv::Word,
        scope_line: spirv::Word,
        declaration: Option<spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_function_id(
            None,
            name,
            ty,
            source,
            line,
            column,
            parent,
            linkage_name,
            flags,
            scope_line,
            declaration,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_function_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        ty: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        linkage_name: spirv::Word,
        flags: spirv::Word,
        scope_line: spirv::Word,
        declaration: Option<spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugFunction as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(ty),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
                dr::Operand::IdRef(parent),
                dr::Operand::IdRef(linkage_name),
                dr::Operand::IdRef(flags),
                dr::Operand::IdRef(scope_line),
            ],
        );
        if let Some(v) = declaration {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_lexical_block(
        &mut self,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        name: Option<spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_lexical_block_id(None, source, line, column, parent, name)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_lexical_block_id(
        &mut self,
        result_id: Option<spirv::Word>,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        name: Option<spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugLexicalBlock as spirv::Word,
                ),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
                dr::Operand::IdRef(parent),
            ],
        );
        if let Some(v) = name {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_lexical_block_discriminator(
        &mut self,
        source: spirv::Word,
        discriminator: spirv::Word,
        parent: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_lexical_block_discriminator_id(None, source, discriminator, parent)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_lexical_block_discriminator_id(
        &mut self,
        result_id: Option<spirv::Word>,
        source: spirv::Word,
        discriminator: spirv::Word,
        parent: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugLexicalBlockDiscriminator as spirv::Word,
                ),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(discriminator),
                dr::Operand::IdRef(parent),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_inlined_at(
        &mut self,
        line: spirv::Word,
        scope: spirv::Word,
        inlined: Option<spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_inlined_at_id(None, line, scope, inlined)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_inlined_at_id(
        &mut self,
        result_id: Option<spirv::Word>,
        line: spirv::Word,
        scope: spirv::Word,
        inlined: Option<spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugInlinedAt as spirv::Word,
                ),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(scope),
            ],
        );
        if let Some(v) = inlined {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_local_variable(
        &mut self,
        name: spirv::Word,
        ty: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        flags: spirv::Word,
        arg_number: Option<spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_local_variable_id(
            None, name, ty, source, line, column, parent, flags, arg_number,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_local_variable_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        ty: spirv::Word,
        source: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
        flags: spirv::Word,
        arg_number: Option<spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugLocalVariable as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(ty),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
                dr::Operand::IdRef(parent),
                dr::Operand::IdRef(flags),
            ],
        );
        if let Some(v) = arg_number {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_inlined_variable(
        &mut self,
        variable: spirv::Word,
        inlined: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_inlined_variable_id(None, variable, inlined)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_inlined_variable_id(
        &mut self,
        result_id: Option<spirv::Word>,
        variable: spirv::Word,
        inlined: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugInlinedVariable as spirv::Word,
                ),
                dr::Operand::IdRef(variable),
                dr::Operand::IdRef(inlined),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_operation(
        &mut self,
        op_code: spirv::Word,
        operands: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_operation_id(None, op_code, operands)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_operation_id(
        &mut self,
        result_id: Option<spirv::Word>,
        op_code: spirv::Word,
        operands: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugOperation as spirv::Word,
                ),
                dr::Operand::IdRef(op_code),
            ],
        );
        inst.operands
            .extend(operands.into_iter().map(dr::Operand::IdRef));
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_expression(
        &mut self,
        operands: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_expression_id(None, operands)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_expression_id(
        &mut self,
        result_id: Option<spirv::Word>,
        operands: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugExpression as spirv::Word,
                ),
            ],
        );
        inst.operands
            .extend(operands.into_iter().map(dr::Operand::IdRef));
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_macro_def(
        &mut self,
        source: spirv::Word,
        line: spirv::Word,
        name: spirv::Word,
        value: Option<spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_macro_def_id(None, source, line, name, value)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_macro_def_id(
        &mut self,
        result_id: Option<spirv::Word>,
        source: spirv::Word,
        line: spirv::Word,
        name: spirv::Word,
        value: Option<spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugMacroDef as spirv::Word,
                ),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(name),
            ],
        );
        if let Some(v) = value {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_macro_undef(
        &mut self,
        source: spirv::Word,
        line: spirv::Word,
        macro_: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_macro_undef_id(None, source, line, macro_)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_macro_undef_id(
        &mut self,
        result_id: Option<spirv::Word>,
        source: spirv::Word,
        line: spirv::Word,
        macro_: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugMacroUndef as spirv::Word,
                ),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(macro_),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_imported_entity(
        &mut self,
        name: spirv::Word,
        tag: spirv::Word,
        source: spirv::Word,
        entity: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_imported_entity_id(None, name, tag, source, entity, line, column, parent)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_imported_entity_id(
        &mut self,
        result_id: Option<spirv::Word>,
        name: spirv::Word,
        tag: spirv::Word,
        source: spirv::Word,
        entity: spirv::Word,
        line: spirv::Word,
        column: spirv::Word,
        parent: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugImportedEntity as spirv::Word,
                ),
                dr::Operand::IdRef(name),
                dr::Operand::IdRef(tag),
                dr::Operand::IdRef(source),
                dr::Operand::IdRef(entity),
                dr::Operand::IdRef(line),
                dr::Operand::IdRef(column),
                dr::Operand::IdRef(parent),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_source(
        &mut self,
        file: spirv::Word,
        text: Option<spirv::Word>,
    ) -> spirv::Word {
        self.shader_debug_source_id(None, file, text)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_source_id(
        &mut self,
        result_id: Option<spirv::Word>,
        file: spirv::Word,
        text: Option<spirv::Word>,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugSource as spirv::Word,
                ),
                dr::Operand::IdRef(file),
            ],
        );
        if let Some(v) = text {
            inst.operands.push(dr::Operand::IdRef(v));
        }
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_source_continued(&mut self, text: spirv::Word) -> spirv::Word {
        self.shader_debug_source_continued_id(None, text)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_source_continued_id(
        &mut self,
        result_id: Option<spirv::Word>,
        text: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugSourceContinued as spirv::Word,
                ),
                dr::Operand::IdRef(text),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_build_identifier(
        &mut self,
        identifier: spirv::Word,
        flags: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_build_identifier_id(None, identifier, flags)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_build_identifier_id(
        &mut self,
        result_id: Option<spirv::Word>,
        identifier: spirv::Word,
        flags: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugBuildIdentifier as spirv::Word,
                ),
                dr::Operand::IdRef(identifier),
                dr::Operand::IdRef(flags),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_storage_path(&mut self, path: spirv::Word) -> spirv::Word {
        self.shader_debug_storage_path_id(None, path)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_storage_path_id(
        &mut self,
        result_id: Option<spirv::Word>,
        path: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugStoragePath as spirv::Word,
                ),
                dr::Operand::IdRef(path),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_entry_point(
        &mut self,
        entry_point: spirv::Word,
        compilation_unit: spirv::Word,
        compiler_signature: spirv::Word,
        command_line_arguments: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_entry_point_id(
            None,
            entry_point,
            compilation_unit,
            compiler_signature,
            command_line_arguments,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_entry_point_id(
        &mut self,
        result_id: Option<spirv::Word>,
        entry_point: spirv::Word,
        compilation_unit: spirv::Word,
        compiler_signature: spirv::Word,
        command_line_arguments: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugEntryPoint as spirv::Word,
                ),
                dr::Operand::IdRef(entry_point),
                dr::Operand::IdRef(compilation_unit),
                dr::Operand::IdRef(compiler_signature),
                dr::Operand::IdRef(command_line_arguments),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_matrix(
        &mut self,
        vector_type: spirv::Word,
        vector_count: spirv::Word,
        column_major: spirv::Word,
    ) -> spirv::Word {
        self.shader_debug_type_matrix_id(None, vector_type, vector_count, column_major)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_type_matrix_id(
        &mut self,
        result_id: Option<spirv::Word>,
        vector_type: spirv::Word,
        vector_count: spirv::Word,
        column_major: spirv::Word,
    ) -> spirv::Word {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        let mut inst = dr::Instruction::new(
            spirv::Op::ExtInst,
            Some(result_type),
            result_id,
            vec![
                dr::Operand::IdRef(extension_set),
                dr::Operand::LiteralExtInstInteger(
                    crate::spirv::DebugInfoOp::DebugTypeMatrix as spirv::Word,
                ),
                dr::Operand::IdRef(vector_type),
                dr::Operand::IdRef(vector_count),
                dr::Operand::IdRef(column_major),
            ],
        );
        if let Some(id) = inst.result_id {
            self.module_mut().types_global_values.push(inst);
            id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            id
        } else {
            let id = self.id();
            inst.result_id = Some(id);
            self.module_mut().types_global_values.push(inst);
            id
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_scope(
        &mut self,
        scope: spirv::Word,
        inlined_at: Option<spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        self.shader_debug_scope_id(None, scope, inlined_at)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_scope_id(
        &mut self,
        result_id: Option<spirv::Word>,
        scope: spirv::Word,
        inlined_at: Option<spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(scope)];
        if let Some(v) = inlined_at {
            args.push(dr::Operand::IdRef(v));
        }
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::DebugInfoOp::DebugScope as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_no_scope(&mut self) -> Result<spirv::Word, dr::Error> {
        self.shader_debug_no_scope_id(None)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_no_scope_id(
        &mut self,
        result_id: Option<spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        #[allow(unused_mut)]
        let mut args = vec![];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::DebugInfoOp::DebugNoScope as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_declare(
        &mut self,
        local_variable: spirv::Word,
        variable: spirv::Word,
        expression: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        self.shader_debug_declare_id(None, local_variable, variable, expression, indexes)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_declare_id(
        &mut self,
        result_id: Option<spirv::Word>,
        local_variable: spirv::Word,
        variable: spirv::Word,
        expression: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(local_variable),
            dr::Operand::IdRef(variable),
            dr::Operand::IdRef(expression),
        ];
        args.extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::DebugInfoOp::DebugDeclare as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_value(
        &mut self,
        local_variable: spirv::Word,
        value: spirv::Word,
        expression: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        self.shader_debug_value_id(None, local_variable, value, expression, indexes)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_value_id(
        &mut self,
        result_id: Option<spirv::Word>,
        local_variable: spirv::Word,
        value: spirv::Word,
        expression: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(local_variable),
            dr::Operand::IdRef(value),
            dr::Operand::IdRef(expression),
        ];
        args.extend(indexes.into_iter().map(dr::Operand::IdRef));
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::DebugInfoOp::DebugValue as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_function_definition(
        &mut self,
        function: spirv::Word,
        definition: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.shader_debug_function_definition_id(None, function, definition)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_function_definition_id(
        &mut self,
        result_id: Option<spirv::Word>,
        function: spirv::Word,
        definition: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(function), dr::Operand::IdRef(definition)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::DebugInfoOp::DebugFunctionDefinition as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_line(
        &mut self,
        source: spirv::Word,
        line_start: spirv::Word,
        line_end: spirv::Word,
        column_start: spirv::Word,
        column_end: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.shader_debug_line_id(None, source, line_start, line_end, column_start, column_end)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_line_id(
        &mut self,
        result_id: Option<spirv::Word>,
        source: spirv::Word,
        line_start: spirv::Word,
        line_end: spirv::Word,
        column_start: spirv::Word,
        column_end: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(source),
            dr::Operand::IdRef(line_start),
            dr::Operand::IdRef(line_end),
            dr::Operand::IdRef(column_start),
            dr::Operand::IdRef(column_end),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::DebugInfoOp::DebugLine as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_no_line(&mut self) -> Result<spirv::Word, dr::Error> {
        self.shader_debug_no_line_id(None)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn shader_debug_no_line_id(
        &mut self,
        result_id: Option<spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "NonSemantic.Shader.DebugInfo.100");
        let result_type = self.type_void();
        #[allow(unused_mut)]
        let mut args = vec![];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::DebugInfoOp::DebugNoLine as spirv::Word,
            args,
        )
    }
}
