use crate::codegen::Codegen;
use gneurshk_parser::Stmt;
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_function_call(
        &mut self,
        name: String,
        args: Vec<Stmt>,
    ) -> Option<BasicValueEnum<'ctx>> {
        // Handle built-in functions
        match name.as_str() {
            "println" => return self.build_println(args),
            "print" => return self.build_print(args),
            _ => (),
        }

        // Get the function from the scope
        let function = self.scope.get_function(&name)?;

        // Compile the arguments
        let mut arg_values = Vec::new();
        for arg in args {
            if let Some(value) = self.build_stmt(arg) {
                arg_values.push(value.into());
            }
        }

        // Build the function call
        let call_result = self
            .builder
            .build_call(function, &arg_values, &format!("call_{}", name))
            .unwrap();

        Some(call_result.try_as_basic_value().unwrap_basic())
    }

    fn build_println(&mut self, args: Vec<Stmt>) -> Option<BasicValueEnum<'ctx>> {
        // Compile the arguments and create format string
        let mut arg_values: Vec<BasicMetadataValueEnum<'ctx>> = Vec::new();
        let mut format_str = String::new();

        for (i, arg) in args.iter().enumerate() {
            if let Some(value) = self.build_stmt(arg.clone()) {
                // Depending on the type, add the appropriate format specifier
                match value {
                    BasicValueEnum::FloatValue(float_val) => {
                        format_str.push_str("%f");

                        // Convert f32 to f64 for printf
                        let f64_type = self.context.f64_type();
                        let double_val = self
                            .builder
                            .build_float_ext(float_val, f64_type, "f64_ext")
                            .unwrap();

                        arg_values.push(double_val.into());
                    }
                    BasicValueEnum::IntValue(_) => {
                        format_str.push_str("%d");
                        arg_values.push(value.into());
                    }
                    BasicValueEnum::PointerValue(_) => {
                        // WARNING: Not all pointers are will be strings
                        format_str.push_str("%s");
                        arg_values.push(value.into());
                    }
                    _ => panic!("Unsupported argument type"),
                }
            }

            // Add a space between arguments
            if i != args.len() - 1 {
                format_str.push(' ');
            }
        }

        format_str.push_str("\n\0"); // Append a new line and null terminator

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

    fn build_print(&mut self, args: Vec<Stmt>) -> Option<BasicValueEnum<'ctx>> {
        // Compile the arguments and create format string
        let mut arg_values: Vec<BasicMetadataValueEnum<'ctx>> = Vec::new();
        let mut format_str = String::new();

        for (i, arg) in args.iter().enumerate() {
            if let Some(value) = self.build_stmt(arg.clone()) {
                // Depending on the type, add the appropriate format specifier
                match value {
                    BasicValueEnum::FloatValue(float_val) => {
                        format_str.push_str("%f");

                        // Convert f32 to f64 for printf
                        let f64_type = self.context.f64_type();
                        let double_val = self
                            .builder
                            .build_float_ext(float_val, f64_type, "f64_ext")
                            .unwrap();

                        arg_values.push(double_val.into());
                    }
                    BasicValueEnum::IntValue(_) => {
                        format_str.push_str("%d");
                        arg_values.push(value.into());
                    }
                    BasicValueEnum::PointerValue(_) => {
                        // WARNING: Not all pointers are will be strings
                        format_str.push_str("%s");
                        arg_values.push(value.into());
                    }
                    _ => panic!("Unsupported argument type"),
                }
            }

            // Add a space between arguments
            if i != args.len() - 1 {
                format_str.push(' ');
            }
        }

        format_str.push('\0'); // Append null terminator

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
