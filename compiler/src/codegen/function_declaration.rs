use crate::codegen::Codegen;
use gneurshk_parser::types::DataType;
use gneurshk_parser::{FunctionParam, Stmt};
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn compile_function_declaration(
        &mut self,
        name: String,
        params: Vec<FunctionParam>,
        return_type: DataType,
        block: Stmt,
    ) -> Option<BasicValueEnum<'ctx>> {
        // Get i32 type since functions only support that type TODO: Support other types
        let i32_type = self.context.i32_type();

        // Create vector of parameter types (assuming they are all i32)
        let mut param_types = Vec::new();
        for _ in &params {
            param_types.push(i32_type.into());
        }

        // Create function type
        let fn_type = i32_type.fn_type(&param_types, false);
        let function = self.module.add_function(&name, fn_type, None);

        // Store function in the current scope
        self.scope.set_function(name.clone(), function);

        // Create entry block
        let entry_block = self.context.append_basic_block(function, "entry");
        let previous_block = self.builder.get_insert_block();

        self.builder.position_at_end(entry_block);

        // Create new scope for the function
        self.enter_new_scope();

        // Create a variable for each parameter in the current scope
        for (i, param) in params.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            let alloca = self.builder.build_alloca(i32_type, &param.name).unwrap();
            self.builder.build_store(alloca, param_value).unwrap();

            self.scope.set_variable(param.name.clone(), alloca);
        }

        // Compile function body
        let return_value = self.compile_stmt(block);

        // Add default return
        if let Some(return_value) = return_value {
            self.builder
                .build_return(Some(&return_value.into_int_value()))
                .unwrap();
        } else {
            // Default to 0 if no return value provided
            // NOTE: This is a temporary solution and should be removed later when other types are added
            let i32_type = self.context.i32_type();
            self.builder
                .build_return(Some(&i32_type.const_int(0, true)))
                .unwrap();
        }

        // Exit function scope
        self.exit_scope();

        // Restore previous block if it exists
        if let Some(prev_block) = previous_block {
            self.builder.position_at_end(prev_block);
        }

        None
    }
}
