use anyhow::{Result, anyhow};
use gneurshk_lexer::{TokenStream, tokens::Token};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataType {
    Int32,
    Float32,
    String,
    Boolean,
    Custom(String),
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Int32 => write!(f, "Int32"),
            DataType::Float32 => write!(f, "Float32"),
            DataType::String => write!(f, "String"),
            DataType::Boolean => write!(f, "Boolean"),
            DataType::Custom(name) => write!(f, "{}", name),
        }
    }
}

pub(crate) fn parse_type(tokens: &mut TokenStream) -> Result<Option<DataType>> {
    if let Some((Token::Word(name), _)) = tokens.next() {
        match name.as_str() {
            "Int32" => Ok(Some(DataType::Int32)),
            "Float32" => Ok(Some(DataType::Float32)),
            "String" => Ok(Some(DataType::String)),
            "Boolean" => Ok(Some(DataType::Boolean)),
            "void" => Ok(None),
            _ => Ok(Some(DataType::Custom(name))),
        }
    } else {
        Err(anyhow!("Expected a type name"))
    }
}
