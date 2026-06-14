use crate::{errors::SematicError, function::FunctionAnalyzer};
use gneurshk_parser::{ElseBranch, IfStatement, types::DataType};

impl<'a> FunctionAnalyzer<'a> {
    pub(crate) fn analyze_if(&mut self, if_stmt: IfStatement) -> Option<DataType> {
        let data_type = self.analyze_block(*if_stmt.if_block);

        // Make sure the condition evaluates to a boolean
        if self.analyze_expression(*if_stmt.condition) != Some(DataType::Boolean) {
            self.errors.push(SematicError::BooleanOnlyIfCondition);
        }

        // Enforce else branch type consistency when the if block has a data type
        if let Some(expected_type) = &data_type {
            // Check if there is an else branch
            if let Some(else_branch) = if_stmt.else_statement {
                let branch_type = match *else_branch {
                    ElseBranch::Block(block) => self.analyze_block(block),
                    ElseBranch::IfStatement(if_stmt2) => self.analyze_if(if_stmt2),
                };

                // Make sure the else block matches the expected type
                if branch_type != Some(expected_type.clone()) {
                    self.errors.push(SematicError::IfElseTypeMismatch);
                }
            } else {
                // Else branches are required when the if block has a data type
                self.errors.push(SematicError::IfMissingElse)
            }
        }

        // Return the data type of the if statement
        data_type
    }
}
