use crate::codegen::Codegen;
use gneurshk_parser::Stmt;
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_block(&mut self, body: Vec<Stmt>) -> Option<BasicValueEnum<'ctx>> {
        self.enter_new_scope();

        let mut last_value = None;
        for stmt in body {
            last_value = self.build_stmt(stmt);
        }

        self.exit_scope();

        last_value
    }
}
