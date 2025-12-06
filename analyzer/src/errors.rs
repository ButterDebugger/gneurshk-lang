use gneurshk_parser::types::DataType;
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

    #[error("Variable '{0}' not found")]
    VariableNotFound(String),

    #[error("Type mismatch: '{0}' != '{1}'")]
    TypeMismatch(DataType, DataType),

    #[error("No type or value provided for variable declaration")]
    NoTypeOrValueProvided,
}

#[derive(Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SematicWarning {
    #[error("Variable '{0}' is never used")]
    UnusedVariable(String),
}
