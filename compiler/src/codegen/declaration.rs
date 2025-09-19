use crate::codegen::Codegen;
use gneurshk_parser::Stmt;
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn compile_declaration(
        &mut self,
        name: String,
        value: Option<Box<Stmt>>,
    ) -> Option<BasicValueEnum<'ctx>> {
        let i32_type = self.context.i32_type();

        // Create variable allocation
        let alloca = self.builder.build_alloca(i32_type, &name).unwrap();

        // If its initial value is provided, compile and store it
        if let Some(val) = value {
            if let Some(init_value) = self.compile_stmt(*val) {
                self.builder.build_store(alloca, init_value).unwrap();
            } else {
                // Default to 0 if no value provided
                let zero = i32_type.const_int(0, false);
                self.builder.build_store(alloca, zero).unwrap();
            }
        } else {
            // Default to 0 if no initial value is provided
            let zero = i32_type.const_int(0, false);
            self.builder.build_store(alloca, zero).unwrap();
        }

        // Store variable in the current scope
        self.scope.set_variable(name.clone(), alloca);

        None
    }
}
