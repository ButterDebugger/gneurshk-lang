use crate::codegen::{Codegen, scope::AllocationKind};
use gneurshk_parser::Block;
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_block(&mut self, block: Block) -> Option<BasicValueEnum<'ctx>> {
        self.enter_new_scope();

        // Build each statement and take the last value as the value of the block
        let mut last_value = None;

        for stmt in block.body {
            last_value = self.build_stmt(stmt);
        }

        // Delete variables falling out of scope
        for local_var in self.scope.get_local_variables() {
            // NOTE: No heap variables are ever defined so this is unused and untested
            if local_var.alloc == AllocationKind::Heap {
                let _ = self.builder.build_free(local_var.pointer);
            }
        }

        self.exit_scope();

        last_value
    }
}
