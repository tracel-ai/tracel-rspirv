use crate::dr::{self, Builder};
use crate::spirv::{
    self,
    BuildIdentifierFlags,
    DebugBaseTypeAttributeEncoding,
    DebugCompositeType,
    DebugImportedEntity,
    DebugInfoFlags,
    DebugOperation,
    DebugTypeQualifier,
    SourceLanguage,
    StorageClass,
};

use super::{const_u32, debug_string};
pub trait DebugInfoBuilder {
    #[doc = "Appends an DebugInfoNone instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_info_none(&mut self) -> spirv::Word;
    #[doc = "Appends an DebugCompilationUnit instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_compilation_unit(
        &mut self,
        version: u32,
        dwarf_version: u32,
        source: spirv::Word,
        language: SourceLanguage,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeBasic instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_basic(
        &mut self,
        name: impl Into<String>,
        size: u32,
        encoding: DebugBaseTypeAttributeEncoding,
        flags: DebugInfoFlags,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypePointer instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_pointer(
        &mut self,
        base_type: spirv::Word,
        storage_class: StorageClass,
        flags: DebugInfoFlags,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeQualifier instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_qualifier(
        &mut self,
        base_type: spirv::Word,
        type_qualifier: DebugTypeQualifier,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeArray instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_array(
        &mut self,
        base_type: spirv::Word,
        component_counts: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeVector instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_vector(&mut self, base_type: spirv::Word, component_count: u32) -> spirv::Word;
    #[doc = "Appends an DebugTypedef instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_typedef(
        &mut self,
        name: impl Into<String>,
        base_type: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeFunction instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_function(
        &mut self,
        flags: DebugInfoFlags,
        return_type: spirv::Word,
        parameter_types: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeEnum instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn debug_type_enum(
        &mut self,
        name: impl Into<String>,
        underlying_type: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        size: u32,
        flags: DebugInfoFlags,
        value_name_value_name: impl IntoIterator<Item = (u32, String)>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeComposite instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn debug_type_composite(
        &mut self,
        name: impl Into<String>,
        tag: DebugCompositeType,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        linkage_name: impl Into<String>,
        size: u32,
        flags: DebugInfoFlags,
        members: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeMember instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn debug_type_member(
        &mut self,
        name: impl Into<String>,
        ty: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        offset: u32,
        size: u32,
        flags: DebugInfoFlags,
        value: Option<spirv::Word>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeInheritance instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn debug_type_inheritance(
        &mut self,
        parent: spirv::Word,
        offset: u32,
        size: u32,
        flags: DebugInfoFlags,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypePtrToMember instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_ptr_to_member(
        &mut self,
        member_type: spirv::Word,
        parent: spirv::Word,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeTemplate instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_template(
        &mut self,
        target: spirv::Word,
        parameters: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeTemplateParameter instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_template_parameter(
        &mut self,
        name: impl Into<String>,
        actual_type: spirv::Word,
        value: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeTemplateTemplateParameter instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_template_template_parameter(
        &mut self,
        name: impl Into<String>,
        template_name: impl Into<String>,
        source: spirv::Word,
        line: u32,
        column: u32,
    ) -> spirv::Word;
    #[doc = "Appends an DebugTypeTemplateParameterPack instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_template_parameter_pack(
        &mut self,
        name: impl Into<String>,
        source: spirv::Word,
        line: u32,
        column: u32,
        template_parameters: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugGlobalVariable instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn debug_global_variable(
        &mut self,
        name: impl Into<String>,
        ty: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        linkage_name: impl Into<String>,
        variable: spirv::Word,
        flags: DebugInfoFlags,
        static_member_declaration: Option<spirv::Word>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugFunctionDeclaration instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn debug_function_declaration(
        &mut self,
        name: impl Into<String>,
        ty: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        linkage_name: impl Into<String>,
        flags: DebugInfoFlags,
    ) -> spirv::Word;
    #[doc = "Appends an DebugFunction instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn debug_function(
        &mut self,
        name: impl Into<String>,
        ty: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        linkage_name: impl Into<String>,
        flags: DebugInfoFlags,
        scope_line: u32,
        declaration: Option<spirv::Word>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugLexicalBlock instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_lexical_block(
        &mut self,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        name: Option<impl Into<String>>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugLexicalBlockDiscriminator instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_lexical_block_discriminator(
        &mut self,
        source: spirv::Word,
        discriminator: u32,
        parent: spirv::Word,
    ) -> spirv::Word;
    #[doc = "Appends an DebugScope instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_scope(
        &mut self,
        scope: spirv::Word,
        inlined_at: Option<spirv::Word>,
    ) -> Result<(), dr::Error>;
    #[doc = "Appends an DebugNoScope instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_no_scope(&mut self) -> Result<(), dr::Error>;
    #[doc = "Appends an DebugInlinedAt instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_inlined_at(
        &mut self,
        line: u32,
        scope: spirv::Word,
        inlined: Option<spirv::Word>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugLocalVariable instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn debug_local_variable(
        &mut self,
        name: impl Into<String>,
        ty: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        flags: DebugInfoFlags,
        arg_number: Option<spirv::Word>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugInlinedVariable instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_inlined_variable(
        &mut self,
        variable: spirv::Word,
        inlined: spirv::Word,
    ) -> spirv::Word;
    #[doc = "Appends an DebugDeclare instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_declare(
        &mut self,
        local_variable: spirv::Word,
        variable: spirv::Word,
        expression: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an DebugValue instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_value(
        &mut self,
        local_variable: spirv::Word,
        value: spirv::Word,
        expression: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<(), dr::Error>;
    #[doc = "Appends an DebugOperation instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_operation(
        &mut self,
        op_code: DebugOperation,
        operands: impl IntoIterator<Item = u32>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugExpression instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_expression(&mut self, operands: impl IntoIterator<Item = spirv::Word>) -> spirv::Word;
    #[doc = "Appends an DebugMacroDef instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_macro_def(
        &mut self,
        source: impl Into<String>,
        line: u32,
        name: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugMacroUndef instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_macro_undef(&mut self, source: impl Into<String>, line: u32, macro_: spirv::Word);
    #[doc = "Appends an DebugImportedEntity instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn debug_imported_entity(
        &mut self,
        name: impl Into<String>,
        tag: DebugImportedEntity,
        source: spirv::Word,
        entity: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
    ) -> spirv::Word;
    #[doc = "Appends an DebugSource instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_source(
        &mut self,
        file: impl Into<String>,
        text: Option<impl Into<String>>,
    ) -> spirv::Word;
    #[doc = "Appends an DebugFunctionDefinition instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_function_definition(
        &mut self,
        function: spirv::Word,
        definition: spirv::Word,
    ) -> Result<(), dr::Error>;
    #[doc = "Appends an DebugSourceContinued instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_source_continued(&mut self, text: impl Into<String>);
    #[doc = "Appends an DebugLine instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_line(
        &mut self,
        source: spirv::Word,
        line_start: u32,
        line_end: u32,
        column_start: u32,
        column_end: u32,
    ) -> Result<(), dr::Error>;
    #[doc = "Appends an DebugNoLine instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_no_line(&mut self) -> Result<(), dr::Error>;
    #[doc = "Appends an DebugBuildIdentifier instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_build_identifier(
        &mut self,
        identifier: impl Into<String>,
        flags: BuildIdentifierFlags,
    );
    #[doc = "Appends an DebugStoragePath instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_storage_path(&mut self, path: impl Into<String>);
    #[doc = "Appends an DebugEntryPoint instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_entry_point(
        &mut self,
        entry_point: spirv::Word,
        compilation_unit: spirv::Word,
        compiler_signature: impl Into<String>,
        command_line_arguments: impl Into<String>,
    );
    #[doc = "Appends an DebugTypeMatrix instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_type_matrix(
        &mut self,
        vector_type: spirv::Word,
        vector_count: u32,
        column_major: bool,
    ) -> spirv::Word;
}
impl DebugInfoBuilder for Builder {
    fn debug_info_none(&mut self) -> spirv::Word {
        Builder::shader_debug_info_none(self)
    }
    fn debug_compilation_unit(
        &mut self,
        version: u32,
        dwarf_version: u32,
        source: spirv::Word,
        language: SourceLanguage,
    ) -> spirv::Word {
        let version = const_u32(self, version);
        let dwarf_version = const_u32(self, dwarf_version);
        let language = const_u32(self, language as u32);

        Builder::shader_debug_compilation_unit(self, version, dwarf_version, source, language)
    }
    fn debug_type_basic(
        &mut self,
        name: impl Into<String>,
        size: u32,
        encoding: DebugBaseTypeAttributeEncoding,
        flags: DebugInfoFlags,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let size = const_u32(self, size);
        let encoding = const_u32(self, encoding as u32);
        let flags = const_u32(self, flags.bits());

        Builder::shader_debug_type_basic(self, name, size, encoding, flags)
    }
    fn debug_type_pointer(
        &mut self,
        base_type: spirv::Word,
        storage_class: StorageClass,
        flags: DebugInfoFlags,
    ) -> spirv::Word {
        let storage_class = const_u32(self, storage_class as u32);
        let flags = const_u32(self, flags.bits());

        Builder::shader_debug_type_pointer(self, base_type, storage_class, flags)
    }
    fn debug_type_qualifier(
        &mut self,
        base_type: spirv::Word,
        type_qualifier: DebugTypeQualifier,
    ) -> spirv::Word {
        let type_qualifier = const_u32(self, type_qualifier as u32);

        Builder::shader_debug_type_qualifier(self, base_type, type_qualifier)
    }
    fn debug_type_array(
        &mut self,
        base_type: spirv::Word,
        component_counts: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        Builder::shader_debug_type_array(self, base_type, component_counts)
    }
    fn debug_type_vector(&mut self, base_type: spirv::Word, component_count: u32) -> spirv::Word {
        let component_count = const_u32(self, component_count);
        Builder::shader_debug_type_vector(self, base_type, component_count)
    }
    fn debug_typedef(
        &mut self,
        name: impl Into<String>,
        base_type: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let line = const_u32(self, line);
        let column = const_u32(self, column);

        Builder::shader_debug_typedef(self, name, base_type, source, line, column, parent)
    }
    fn debug_type_function(
        &mut self,
        flags: DebugInfoFlags,
        return_type: spirv::Word,
        parameter_types: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let flags = const_u32(self, flags.bits());

        Builder::shader_debug_type_function(self, flags, return_type, parameter_types)
    }
    fn debug_type_enum(
        &mut self,
        name: impl Into<String>,
        underlying_type: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        size: u32,
        flags: DebugInfoFlags,
        value_name_value_name: impl IntoIterator<Item = (u32, String)>,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let line = const_u32(self, line);
        let column = const_u32(self, column);
        let size = const_u32(self, size);
        let flags = const_u32(self, flags.bits());

        let value_name_value_name = value_name_value_name
            .into_iter()
            .map(|(value, name)| {
                let value = const_u32(self, value);
                let name = debug_string(self, name);
                (value, name)
            })
            .collect::<Vec<_>>();

        Builder::shader_debug_type_enum(
            self,
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
    fn debug_type_composite(
        &mut self,
        name: impl Into<String>,
        tag: DebugCompositeType,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        linkage_name: impl Into<String>,
        size: u32,
        flags: DebugInfoFlags,
        members: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let tag = const_u32(self, tag as u32);
        let line = const_u32(self, line);
        let column = const_u32(self, column);
        let linkage_name = debug_string(self, linkage_name);
        let size = const_u32(self, size);
        let flags = const_u32(self, flags.bits());

        Builder::shader_debug_type_composite(
            self,
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
    fn debug_type_member(
        &mut self,
        name: impl Into<String>,
        ty: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        offset: u32,
        size: u32,
        flags: DebugInfoFlags,
        value: Option<spirv::Word>,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let line = const_u32(self, line);
        let column = const_u32(self, column);
        let offset = const_u32(self, offset);
        let size = const_u32(self, size);
        let flags = const_u32(self, flags.bits());

        Builder::shader_debug_type_member(
            self, name, ty, source, line, column, offset, size, flags, value,
        )
    }
    fn debug_type_inheritance(
        &mut self,
        parent: spirv::Word,
        offset: u32,
        size: u32,
        flags: DebugInfoFlags,
    ) -> spirv::Word {
        let offset = const_u32(self, offset);
        let size = const_u32(self, size);
        let flags = const_u32(self, flags.bits());

        Builder::shader_debug_type_inheritance(self, parent, offset, size, flags)
    }
    fn debug_type_ptr_to_member(
        &mut self,
        member_type: spirv::Word,
        parent: spirv::Word,
    ) -> spirv::Word {
        Builder::shader_debug_type_ptr_to_member(self, member_type, parent)
    }
    fn debug_type_template(
        &mut self,
        target: spirv::Word,
        parameters: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        Builder::shader_debug_type_template(self, target, parameters)
    }
    fn debug_type_template_parameter(
        &mut self,
        name: impl Into<String>,
        actual_type: spirv::Word,
        value: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let line = const_u32(self, line);
        let column = const_u32(self, column);

        Builder::shader_debug_type_template_parameter(
            self,
            name,
            actual_type,
            value,
            source,
            line,
            column,
        )
    }
    fn debug_type_template_template_parameter(
        &mut self,
        name: impl Into<String>,
        template_name: impl Into<String>,
        source: spirv::Word,
        line: u32,
        column: u32,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let template_name = debug_string(self, template_name);
        let line = const_u32(self, line);
        let column = const_u32(self, column);

        Builder::shader_debug_type_template_template_parameter(
            self,
            name,
            template_name,
            source,
            line,
            column,
        )
    }
    fn debug_type_template_parameter_pack(
        &mut self,
        name: impl Into<String>,
        source: spirv::Word,
        line: u32,
        column: u32,
        template_parameters: impl IntoIterator<Item = spirv::Word>,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let line = const_u32(self, line);
        let column = const_u32(self, column);

        Builder::shader_debug_type_template_parameter_pack(
            self,
            name,
            source,
            line,
            column,
            template_parameters,
        )
    }
    fn debug_global_variable(
        &mut self,
        name: impl Into<String>,
        ty: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        linkage_name: impl Into<String>,
        variable: spirv::Word,
        flags: DebugInfoFlags,
        static_member_declaration: Option<spirv::Word>,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let line = const_u32(self, line);
        let column = const_u32(self, column);
        let linkage_name = debug_string(self, linkage_name);
        let flags = const_u32(self, flags.bits());

        Builder::shader_debug_global_variable(
            self,
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
    fn debug_function_declaration(
        &mut self,
        name: impl Into<String>,
        ty: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        linkage_name: impl Into<String>,
        flags: DebugInfoFlags,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let line = const_u32(self, line);
        let column = const_u32(self, column);
        let linkage_name = debug_string(self, linkage_name);
        let flags = const_u32(self, flags.bits());

        Builder::shader_debug_function_declaration(
            self,
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
    fn debug_function(
        &mut self,
        name: impl Into<String>,
        ty: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        linkage_name: impl Into<String>,
        flags: DebugInfoFlags,
        scope_line: u32,
        declaration: Option<spirv::Word>,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let line = const_u32(self, line);
        let column = const_u32(self, column);
        let linkage_name = debug_string(self, linkage_name);
        let flags = const_u32(self, flags.bits());
        let scope_line = const_u32(self, scope_line);

        Builder::shader_debug_function(
            self,
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
    fn debug_lexical_block(
        &mut self,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        name: Option<impl Into<String>>,
    ) -> spirv::Word {
        let line = const_u32(self, line);
        let column = const_u32(self, column);
        let name = name.map(|name| debug_string(self, name));

        Builder::shader_debug_lexical_block(self, source, line, column, parent, name)
    }
    fn debug_lexical_block_discriminator(
        &mut self,
        source: spirv::Word,
        discriminator: u32,
        parent: spirv::Word,
    ) -> spirv::Word {
        let discriminator = const_u32(self, discriminator);

        Builder::shader_debug_lexical_block_discriminator(self, source, discriminator, parent)
    }
    fn debug_scope(
        &mut self,
        scope: spirv::Word,
        inlined_at: Option<spirv::Word>,
    ) -> Result<(), dr::Error> {
        Builder::shader_debug_scope(self, scope, inlined_at).map(|_| ())
    }
    fn debug_no_scope(&mut self) -> Result<(), dr::Error> {
        Builder::shader_debug_no_scope(self).map(|_| ())
    }
    fn debug_inlined_at(
        &mut self,
        line: u32,
        scope: spirv::Word,
        inlined: Option<spirv::Word>,
    ) -> spirv::Word {
        let line = const_u32(self, line);

        Builder::shader_debug_inlined_at(self, line, scope, inlined)
    }
    fn debug_local_variable(
        &mut self,
        name: impl Into<String>,
        ty: spirv::Word,
        source: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
        flags: DebugInfoFlags,
        arg_number: Option<u32>,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let line = const_u32(self, line);
        let column = const_u32(self, column);
        let flags = const_u32(self, flags.bits());
        let arg_number = arg_number.map(|arg| const_u32(self, arg));

        Builder::shader_debug_local_variable(
            self, name, ty, source, line, column, parent, flags, arg_number,
        )
    }
    fn debug_inlined_variable(
        &mut self,
        variable: spirv::Word,
        inlined: spirv::Word,
    ) -> spirv::Word {
        Builder::shader_debug_inlined_variable(self, variable, inlined)
    }
    fn debug_declare(
        &mut self,
        local_variable: spirv::Word,
        variable: spirv::Word,
        expression: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        Builder::shader_debug_declare(self, local_variable, variable, expression, indexes)
    }
    fn debug_value(
        &mut self,
        local_variable: spirv::Word,
        value: spirv::Word,
        expression: spirv::Word,
        indexes: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<(), dr::Error> {
        Builder::shader_debug_value(self, local_variable, value, expression, indexes).map(|_| ())
    }
    fn debug_operation(
        &mut self,
        op_code: DebugOperation,
        operands: impl IntoIterator<Item = u32>,
    ) -> spirv::Word {
        let op_code = const_u32(self, op_code as u32);
        let operands = operands
            .into_iter()
            .map(|operand| const_u32(self, operand))
            .collect::<Vec<_>>();

        Builder::shader_debug_operation(self, op_code, operands)
    }
    fn debug_expression(&mut self, operands: impl IntoIterator<Item = spirv::Word>) -> spirv::Word {
        Builder::shader_debug_expression(self, operands)
    }
    fn debug_macro_def(
        &mut self,
        source: impl Into<String>,
        line: u32,
        name: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> spirv::Word {
        let source = debug_string(self, source);
        let line = const_u32(self, line);
        let name = debug_string(self, name);
        let value = value.map(|value| debug_string(self, value));

        Builder::shader_debug_macro_def(self, source, line, name, value)
    }
    fn debug_macro_undef(&mut self, source: impl Into<String>, line: u32, macro_: spirv::Word) {
        let source = debug_string(self, source);
        let line = const_u32(self, line);

        Builder::shader_debug_macro_undef(self, source, line, macro_);
    }
    fn debug_imported_entity(
        &mut self,
        name: impl Into<String>,
        tag: DebugImportedEntity,
        source: spirv::Word,
        entity: spirv::Word,
        line: u32,
        column: u32,
        parent: spirv::Word,
    ) -> spirv::Word {
        let name = debug_string(self, name);
        let tag = const_u32(self, tag as u32);
        let line = const_u32(self, line);
        let column = const_u32(self, column);

        Builder::shader_debug_imported_entity(self, name, tag, source, entity, line, column, parent)
    }
    fn debug_source(
        &mut self,
        file: impl Into<String>,
        text: Option<impl Into<String>>,
    ) -> spirv::Word {
        let file = debug_string(self, file);
        let text = text.map(|text| debug_string(self, text));

        Builder::shader_debug_source(self, file, text)
    }
    fn debug_function_definition(
        &mut self,
        function: spirv::Word,
        definition: spirv::Word,
    ) -> Result<(), dr::Error> {
        Builder::shader_debug_function_definition(self, function, definition).map(|_| ())
    }
    #[allow(clippy::too_many_arguments)]
    fn debug_source_continued(&mut self, text: impl Into<String>) {
        let text = debug_string(self, text);

        Builder::shader_debug_source_continued(self, text);
    }
    fn debug_line(
        &mut self,
        source: spirv::Word,
        line_start: u32,
        line_end: u32,
        column_start: u32,
        column_end: u32,
    ) -> Result<(), dr::Error> {
        let line_start = const_u32(self, line_start);
        let line_end = const_u32(self, line_end);
        let column_start = const_u32(self, column_start);
        let column_end = const_u32(self, column_end);

        Builder::shader_debug_line(self, source, line_start, line_end, column_start, column_end)
            .map(|_| ())
    }
    fn debug_no_line(&mut self) -> Result<(), dr::Error> {
        Builder::shader_debug_no_line(self).map(|_| ())
    }
    fn debug_build_identifier(
        &mut self,
        identifier: impl Into<String>,
        flags: BuildIdentifierFlags,
    ) {
        let identifier = debug_string(self, identifier);
        let flags = const_u32(self, flags.bits());

        Builder::shader_debug_build_identifier(self, identifier, flags);
    }
    fn debug_storage_path(&mut self, path: impl Into<String>) {
        let path = debug_string(self, path);

        Builder::shader_debug_storage_path(self, path);
    }
    fn debug_entry_point(
        &mut self,
        entry_point: spirv::Word,
        compilation_unit: spirv::Word,
        compiler_signature: impl Into<String>,
        command_line_arguments: impl Into<String>,
    ) {
        let compiler_signature = debug_string(self, compiler_signature);
        let command_line_arguments = debug_string(self, command_line_arguments);

        Builder::shader_debug_entry_point(
            self,
            entry_point,
            compilation_unit,
            compiler_signature,
            command_line_arguments,
        );
    }
    fn debug_type_matrix(
        &mut self,
        vector_type: spirv::Word,
        vector_count: u32,
        column_major: bool,
    ) -> spirv::Word {
        let bool = self.type_bool();
        let vector_count = const_u32(self, vector_count);
        let column_major = match column_major {
            true => self.constant_true(bool),
            false => self.constant_false(bool),
        };

        Builder::shader_debug_type_matrix(self, vector_type, vector_count, column_major)
    }
}
