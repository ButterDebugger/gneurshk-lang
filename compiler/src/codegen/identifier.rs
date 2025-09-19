use crate::codegen::Codegen;
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn compile_identifier(&mut self, name: String) -> Option<BasicValueEnum<'ctx>> {
        let variable_pointer = self.scope.get_variable(&name)?;
        let loaded_value = self
            .builder
            .build_load(self.context.i32_type(), variable_pointer, &name)
            .unwrap();
        Some(loaded_value)
    }
}
