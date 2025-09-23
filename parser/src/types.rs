use gneurshk_lexer::{TokenStream, tokens::Token};

#[derive(Default, Debug, PartialEq, Clone)]
pub enum DataType {
    Int32,
    Float32,
    String,
    Boolean,
    #[default]
    Void,
    Custom(String),
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
