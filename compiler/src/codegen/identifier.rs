use crate::codegen::Codegen;
use gneurshk_parser::Identifier;
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_identifier(
        &mut self,
        identifier: Identifier,
    ) -> Option<BasicValueEnum<'ctx>> {
        let name = identifier.name;

        let variable_pointer = self.scope.get_variable(&name)?;
        let loaded_value = self
            .builder
            .build_load(self.context.i32_type(), variable_pointer, &name)
            .unwrap();

        Some(loaded_value)
    }
}
