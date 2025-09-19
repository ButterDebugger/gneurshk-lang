use crate::codegen::Codegen;
use gneurshk_parser::Stmt;
use inkwell::values::BasicValueEnum;

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
        // Expect exactly one argument TODO: Support multiple arguments
        if args.len() != 1 {
            return None;
        }

        // Compile the argument
        let arg_value = self.compile_stmt(args.into_iter().next().unwrap())?;

        // Create format string
        let format_str = "%d\n\0";
        let format_str_global = self
            .builder
            .build_global_string_ptr(format_str, "format_str")
            .unwrap();

        // Get printf function
        let printf_fn = self.scope.get_function("printf")?;

        // Call printf
        let args = vec![
            format_str_global.as_pointer_value().into(),
            arg_value.into(),
        ];
        self.builder
            .build_call(printf_fn, &args, "printf_call")
            .unwrap();

        None
    }
}
