use crate::codegen::Codegen;
use gneurshk_parser::Stmt;
use inkwell::{IntPredicate, values::BasicValueEnum};

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn compile_if_statement(
        &mut self,
        condition: Stmt,
        block: Stmt,
        else_block: Option<Stmt>,
    ) -> Option<BasicValueEnum<'ctx>> {
        // Compile the condition
        let condition_value = self.compile_stmt(condition)?;

        // Convert to boolean (non-zero is true)
        let zero = self.context.i32_type().const_int(0, false);
        let condition_bool = self
            .builder
            .build_int_compare(
                IntPredicate::NE,
                condition_value.into_int_value(),
                zero,
                "condition",
            )
            .unwrap();

        // Get current function
        let current_function = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();

        // Create basic blocks
        let then_branch = self
            .context
            .append_basic_block(current_function, "--if-then");
        let else_branch = if else_block.is_some() {
            Some(
                self.context
                    .append_basic_block(current_function, "--if-else"),
            )
        } else {
            None
        };
        let merge_branch = self
            .context
            .append_basic_block(current_function, "--if-merge");

        // Build conditional branch
        self.builder
            .build_conditional_branch(
                condition_bool,
                then_branch,
                else_branch.unwrap_or(merge_branch),
            )
            .unwrap();

        // Build the then block
        self.builder.position_at_end(then_branch);
        self.compile_stmt(block);
        self.builder
            .build_unconditional_branch(merge_branch)
            .unwrap();

        // Build the else block
        if let Some(else_block) = else_block {
            self.builder.position_at_end(else_branch.unwrap());
            self.compile_stmt(else_block);
            self.builder
                .build_unconditional_branch(merge_branch)
                .unwrap();
        }

        // Position at merge block for subsequent code
        self.builder.position_at_end(merge_branch);

        None
    }
}
