use crate::{Analyzer, errors::SematicError};
use gneurshk_parser::{BinaryOperator, Expression, types::DataType};

impl Analyzer {
    pub(crate) fn analyze_binary_expression(
        &mut self,
        left: Box<Expression>,
        right: Box<Expression>,
        operator: BinaryOperator,
    ) -> Option<DataType> {
        let left_type = self.analyze_expression(*left)?;
        let right_type = self.analyze_expression(*right)?;

        match operator {
            BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanEqual
            | BinaryOperator::LessThan
            | BinaryOperator::LessThanEqual
            | BinaryOperator::And
            | BinaryOperator::Or => {
                return Some(DataType::Boolean);
            }
            _ => (),
        }

        match (left_type.clone(), right_type.clone()) {
            (DataType::Int32, DataType::Int32) => Some(DataType::Int32),
            (DataType::Float32, DataType::Float32) => Some(DataType::Float32),
            _ => {
                self.errors
                    .push(SematicError::TypeMismatch(left_type, right_type));

                None
            }
        }
    }
}
