use crate::codegen::Codegen;
use inkwell::values::{BasicValue, BasicValueEnum};

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_global_string(&mut self, value: String) -> Option<BasicValueEnum<'ctx>> {
        // Create global string
        let global_value = self
            .builder
            .build_global_string_ptr(&value, "const_str")
            .unwrap();

        // Make it mutable
        global_value.set_constant(false);

        // Return a pointer to the string
        Some(global_value.as_pointer_value().as_basic_value_enum())
    }

    #[allow(dead_code)]
    pub(crate) fn build_byte_string(&mut self, value: String) -> Option<BasicValueEnum<'ctx>> {
        // Create a byte array from the string
        let str_array = self.context.const_string(value.as_bytes(), false);

        // Return the array as a BasicValueEnum
        Some(str_array.as_basic_value_enum())
    }
}
