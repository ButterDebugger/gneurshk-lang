use crate::codegen::Codegen;
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn compile_literal(&mut self, value: isize) -> Option<BasicValueEnum<'ctx>> {
        let i32_type = self.context.i32_type();
        Some(i32_type.const_int(value as u64, true).into())
    }
}
