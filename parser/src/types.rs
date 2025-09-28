use gneurshk_lexer::{TokenStream, tokens::Token};
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataType {
    Int32,
    Float32,
    String,
    Boolean,
    #[default]
    Void,
    Custom(String),
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Int32 => write!(f, "Int32"),
            DataType::Float32 => write!(f, "Float32"),
            DataType::String => write!(f, "String"),
            DataType::Boolean => write!(f, "Boolean"),
            DataType::Void => write!(f, "void"),
            DataType::Custom(name) => write!(f, "{}", name),
        }
    }
}

pub(crate) fn parse_type(tokens: &mut TokenStream) -> Result<DataType, &'static str> {
    if let Some((Token::Word(name), _)) = tokens.next() {
        match name.as_str() {
            "Int32" => Ok(DataType::Int32),
            "Float32" => Ok(DataType::Float32),
            "String" => Ok(DataType::String),
            "Boolean" => Ok(DataType::Boolean),
            "void" => Ok(DataType::Void),
            _ => Ok(DataType::Custom(name)),
        }
    } else {
        Err("Expected a type")
    }
}
