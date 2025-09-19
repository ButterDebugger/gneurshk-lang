use crate::codegen::Codegen;
use gneurshk_parser::Stmt;
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn compile_function_call(
        &mut self,
        name: String,
        args: Vec<Stmt>,
    ) -> Option<BasicValueEnum<'ctx>> {
        // Handle built-in println function
        if name == "println" {
            return self.compile_println(args);
        }

        // Get the function from the scope
        let function = self.scope.get_function(&name)?;

        // Compile the arguments
        let mut arg_values = Vec::new();
        for arg in args {
            if let Some(value) = self.compile_stmt(arg) {
                arg_values.push(value.into());
            }
        }

        // Build the function call
        let call_result = self
            .builder
            .build_call(function, &arg_values, &format!("call_{}", name))
            .unwrap();

        call_result.try_as_basic_value().left()
    }

    fn compile_println(&mut self, args: Vec<Stmt>) -> Option<BasicValueEnum<'ctx>> {
        // Compile the arguments
        let mut arg_values: Vec<BasicMetadataValueEnum<'ctx>> = Vec::new();
        for arg in args {
            if let Some(value) = self.compile_stmt(arg) {
                arg_values.push(value.into());
            }
        }

        // Create format string
        let mut format_str = String::new();

        for i in 0..arg_values.len() {
            format_str.push_str("%d");

            if i != arg_values.len() - 1 {
                format_str.push(' ');
            }
        }

        format_str.push_str("\n\0");

        // Create global string
        let format_str_global = self
            .builder
            .build_global_string_ptr(&format_str, "format_str")
            .unwrap();

        // Get printf function
        let printf_fn = self.scope.get_function("printf")?;

        // Call printf
        let mut printf_args = vec![format_str_global.as_pointer_value().into()];
        printf_args.extend(arg_values);

        self.builder
            .build_call(printf_fn, &printf_args, "printf_call")
            .unwrap();

        None
    }
}
