use gneurshk_parser::{BinaryOperator, types::DataType};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SematicError {
    #[error("Function '{0}' not found")]
    FunctionNotFound(String),

    #[error("Function '{0}' takes {1} arguments but {2} were given")]
    FunctionCallArgumentCountMismatch(String, usize, usize),

    #[error(
        "Function call for '{0}' type mismatch. Expected '{2}' for argument {1}, but found '{3}'"
    )]
    FunctionCallArgumentMismatch(String, usize, DataType, DataType),

    #[error("Function '{0}' return type mismatch.")]
    FunctionReturnTypeMismatch(String),

    #[error("Variable '{0}' not found")]
    VariableNotFound(String),

    #[error("Variable '{0}' of type '{1}' cannot be set to a value of type '{2}'")]
    AssignmentTypeMismatch(String, DataType, DataType),

    #[error("Cannot use the '{1}' operator to types '{0}' and '{2}'")]
    UnsupportedOperator(DataType, BinaryOperator, DataType),

    #[error("No type or value provided for variable declaration")]
    NoTypeOrValueProvided,

    #[error("Variable '{0}' is not able to be modified")]
    VariableUnmodifiable(String),

    #[error("The if statement requires an else that evaluates to the expected type")]
    IfMissingElse,

    #[error("The if and else types do not match")]
    IfElseTypeMismatch,

    #[error("If condition must evaluate to a boolean value")]
    BooleanOnlyIfCondition,

    #[error("Break statement cannot belong outside of a loop")]
    BreakOutsideLoop,

    #[error("Continue statement cannot belong outside of a loop")]
    ContinueOutsideLoop,

    #[error("Cannot apply the not operator to a non-boolean type")]
    CannotUseNot,

    #[error("Cannot apply the negative operator to a non-numeric type")]
    CannotUseNegative,
}

#[derive(Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SematicWarning {
    #[error("Variable '{0}' is never used")]
    UnusedVariable(String),
}
