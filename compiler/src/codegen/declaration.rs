use crate::codegen::{Codegen, scope::{AllocationKind, Variable}};
use gneurshk_parser::VariableDeclaration;
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_declaration(
        &mut self,
        variable_declaration: VariableDeclaration,
    ) -> Option<BasicValueEnum<'ctx>> {
        // Get name and value
        let (name, value) = match variable_declaration {
            VariableDeclaration::Mutable { name, value, .. } => (name, value),
            VariableDeclaration::Constant { name, value, .. } => (name, Some(value)),
        };

        // Create variable allocation
        let i32_type = self.context.i32_type();

        let ptr = self.builder.build_alloca(i32_type, &name).unwrap();

        // If its initial value is provided, compile and store it
        if let Some(val) = value {
            if let Some(init_value) = self.build_expression(val) {
                self.builder.build_store(ptr, init_value).unwrap();
            } else {
                // Default to 0 if no value provided
                let zero = i32_type.const_int(0, false);
                self.builder.build_store(ptr, zero).unwrap();
            }
        } else {
            // Default to 0 if no initial value is provided
            let zero = i32_type.const_int(0, false);
            self.builder.build_store(ptr, zero).unwrap();
        }

        // Store variable in the current scope
        self.scope.set_variable(
            name.clone(),
            Variable {
                pointer: ptr,
                alloc: AllocationKind::Stack,
            },
        );

        None
    }
}
