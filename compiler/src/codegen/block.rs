use crate::codegen::Codegen;
use gneurshk_parser::Block;
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_block(&mut self, block: Block) -> Option<BasicValueEnum<'ctx>> {
        self.enter_new_scope();

        let mut last_value = None;
        for stmt in block.body {
            last_value = self.build_stmt(stmt);
        }

        self.exit_scope();

        last_value
    }
}
