use crate::{errors::SematicError, function::FunctionAnalyzer};
use gneurshk_parser::{Return, types::DataType};

impl<'a> FunctionAnalyzer<'a> {
    pub(crate) fn analyze_return(&mut self, return_stmt: Return) -> Option<DataType> {
        // Analyze the return's expression type
        let return_type = return_stmt
            .value
            .and_then(|expr| self.analyze_expression(expr));

        // Check if the return type doesn't match the return type of the function
        if return_type != self.function_declaration.return_type {
            self.errors.push(SematicError::FunctionReturnTypeMismatch(
                self.function_declaration.name.clone(),
            ));
        }

        // Returns don't have a value
        None
    }
}
