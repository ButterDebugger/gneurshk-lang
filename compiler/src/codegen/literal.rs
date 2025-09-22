use crate::codegen::Codegen;
use inkwell::values::{BasicValue, BasicValueEnum};

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_integer(&mut self, value: u64) -> Option<BasicValueEnum<'ctx>> {
        let i32_type = self.context.i32_type();

        Some(i32_type.const_int(value, true).as_basic_value_enum())
    }

    pub(crate) fn build_float(&mut self, value: f64) -> Option<BasicValueEnum<'ctx>> {
        let f32_type = self.context.f32_type();

        Some(f32_type.const_float(value).as_basic_value_enum())
    }
}
