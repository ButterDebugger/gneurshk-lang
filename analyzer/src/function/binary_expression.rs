use crate::{errors::SematicError, function::FunctionAnalyzer};
use gneurshk_parser::{BinaryOperator, Expression, types::DataType};

impl<'a> FunctionAnalyzer<'a> {
    pub(crate) fn analyze_binary_expression(
        &mut self,
        left: Expression,
        right: Expression,
        operator: BinaryOperator,
    ) -> Option<DataType> {
        let left_type = self.analyze_expression(left)?;
        let right_type = self.analyze_expression(right)?;

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
            // Same integer types
            (DataType::Int8, DataType::Int8) => Some(DataType::Int8),
            (DataType::Int16, DataType::Int16) => Some(DataType::Int16),
            (DataType::Int32, DataType::Int32) => Some(DataType::Int32),
            (DataType::Int64, DataType::Int64) => Some(DataType::Int64),
            // Same unsigned integer types
            (DataType::UInt8, DataType::UInt8) => Some(DataType::UInt8),
            (DataType::UInt16, DataType::UInt16) => Some(DataType::UInt16),
            (DataType::UInt32, DataType::UInt32) => Some(DataType::UInt32),
            (DataType::UInt64, DataType::UInt64) => Some(DataType::UInt64),
            // Same floating point types
            (DataType::Float32, DataType::Float32) => Some(DataType::Float32),
            (DataType::Float64, DataType::Float64) => Some(DataType::Float64),
            _ => {
                self.program_analyzer
                    .errors
                    .push(SematicError::TypeMismatch(left_type, right_type));

                None
            }
        }
    }
}
