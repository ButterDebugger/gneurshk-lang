use crate::{errors::SematicError, function::FunctionAnalyzer};
use gneurshk_parser::{UnaryExpression, UnaryOperator, types::DataType};

impl<'a> FunctionAnalyzer<'a> {
    pub(crate) fn analyze_unary_expression(&mut self, expr: UnaryExpression) -> Option<DataType> {
        let UnaryExpression { value, operator } = expr;

        // Analyze the inner expression
        let value_type = self.analyze_expression(*value)?;

        match operator {
            UnaryOperator::Not => match value_type {
                DataType::Boolean => Some(DataType::Boolean),
                _ => {
                    self.errors.push(SematicError::CannotUseNot);

                    None
                }
            },
            UnaryOperator::Negative => match value_type {
                DataType::Int32 | DataType::Float32 => Some(value_type),
                _ => {
                    self.errors.push(SematicError::CannotUseNegative);

                    None
                }
            },
        }
    }
}
