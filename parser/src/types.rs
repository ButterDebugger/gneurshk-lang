use anyhow::{Result, anyhow};
use gneurshk_lexer::{TokenStream, tokens::Token};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Clone, PartialEq, Eq, Hash, strum_macros::Display, EnumString)]
pub enum DataType {
    // Integer types
    #[strum(to_string = "Int8")]
    Int8,
    #[strum(to_string = "Int16")]
    Int16,
    #[strum(to_string = "Int32")]
    Int32,
    #[strum(to_string = "Int64")]
    Int64,
    // Unsigned integer types
    #[strum(to_string = "UInt8")]
    UInt8,
    #[strum(to_string = "UInt16")]
    UInt16,
    #[strum(to_string = "UInt32")]
    UInt32,
    #[strum(to_string = "UInt64")]
    UInt64,
    // Floating point types
    #[strum(to_string = "Float32")]
    Float32,
    #[strum(to_string = "Float64")]
    Float64,
    // Other types
    #[strum(to_string = "String")]
    String,
    #[strum(to_string = "Boolean")]
    Boolean,
    #[strum(to_string = "{0}")]
    Custom(String),
}

pub(crate) fn parse_type(tokens: &mut TokenStream) -> Result<Option<DataType>> {
    if let Some((Token::Word(name), _)) = tokens.next() {
        match DataType::from_str(name.as_str()) {
            Ok(primitive) => Ok(Some(primitive)),
            Err(_) => Ok(Some(DataType::Custom(name))),
        }
    } else {
        Err(anyhow!("Expected a type name"))
    }
}
