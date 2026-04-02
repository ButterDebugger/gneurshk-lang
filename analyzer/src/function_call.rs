use crate::{Analyzer, errors::SematicError};
use gneurshk_parser::{Expression, types::DataType};

impl Analyzer {
    pub(crate) fn analyze_function_call(
        &mut self,
        name: String,
        args: Vec<Expression>,
    ) -> Option<DataType> {
        // Handle built-in functions
        if matches!(name.as_str(), "println" | "print") {
            // Analyze arguments and ignore types for these functions
            for arg in args {
                self.analyze_expression(arg);
            }
            return Some(DataType::Void);
        }

        if let Some(function) = self.functions.get(&name).cloned() {
            // Check for correct number of arguments
            if args.len() != function.params.len() {
                self.errors
                    .push(SematicError::FunctionCallArgumentCountMismatch(
                        name.clone(),
                        function.params.len(),
                        args.len(),
                    ));
            }
            // Check for correct types of arguments
            else {
                let mut arg_types = Vec::with_capacity(args.len());
                for arg in args {
                    arg_types.push(self.analyze_expression(arg)?);
                }

                let expected_types = function
                    .params
                    .iter()
                    .map(|param| param.data_type.clone())
                    .collect::<Vec<_>>();

                for (i, (expected, actual)) in
                    expected_types.iter().zip(arg_types.iter()).enumerate()
                {
                    if expected != actual {
                        self.errors.push(SematicError::FunctionCallArgumentMismatch(
                            name.clone(),
                            i + 1,
                            expected.clone(),
                            actual.clone(),
                        ));
                    }
                }
            }

            Some(function.return_type.clone())
        } else {
            self.errors.push(SematicError::FunctionNotFound(name));

            // TODO: analyze arguments anyway

            None
        }
    }
}
