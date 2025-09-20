use crate::codegen::Codegen;
use gneurshk_parser::Stmt;
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_return_statement(
        &mut self,
        value: Option<Box<Stmt>>,
    ) -> Option<BasicValueEnum<'ctx>> {
        if let Some(value) = value {
            let return_value = self.build_stmt(*value)?;
            self.builder
                .build_return(Some(&return_value.into_int_value()))
                .unwrap();
        } else {
            self.builder.build_return(None).unwrap();
        }

        None
    }
}
