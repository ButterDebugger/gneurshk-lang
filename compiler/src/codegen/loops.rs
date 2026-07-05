use crate::codegen::{Codegen, LoopContext};
use gneurshk_parser::LoopStmt;
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_loop(&mut self, loop_stmt: LoopStmt) -> Option<BasicValueEnum<'ctx>> {
        // Get current function
        let current_function = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();

        // Create basic blocks
        let loop_body = self
            .context
            .append_basic_block(current_function, "--loop-body");
        let after_loop = self
            .context
            .append_basic_block(current_function, "--loop-after");

        // Jump into the loop body
        self.builder.build_unconditional_branch(loop_body).unwrap();

        // Build the loop body
        self.builder.position_at_end(loop_body);
        self.loop_stack.push(LoopContext {
            continue_target: loop_body,
            break_target: after_loop,
        });
        self.build_block(*loop_stmt.block);
        self.loop_stack.pop();

        // Jump back to the start of the loop unless the body already terminated
        let current_block = self.builder.get_insert_block().unwrap();

        if current_block.get_terminator().is_none() {
            self.builder.build_unconditional_branch(loop_body).unwrap();
        }

        // Position at the block after the loop for subsequent code
        self.builder.position_at_end(after_loop);

        None
    }

    pub(crate) fn build_break_statement(&mut self) -> Option<BasicValueEnum<'ctx>> {
        let loop_context = self
            .loop_stack
            .last()
            .expect("break statement outside of a loop");

        self.builder
            .build_unconditional_branch(loop_context.break_target)
            .unwrap();

        None
    }

    pub(crate) fn build_continue_statement(&mut self) -> Option<BasicValueEnum<'ctx>> {
        let loop_context = self
            .loop_stack
            .last()
            .expect("continue statement outside of a loop");

        self.builder
            .build_unconditional_branch(loop_context.continue_target)
            .unwrap();

        None
    }
}
