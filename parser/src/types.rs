use gneurshk_lexer::{TokenStream, tokens::Token};

#[derive(Default, Debug, PartialEq, Clone)]
pub enum DataType {
    Int32,
    #[default]
    Void,
    Custom(String),
}

pub(crate) fn parse_type(tokens: &mut TokenStream) -> Result<DataType, &'static str> {
    if let Some((Token::Word(name), _)) = tokens.next() {
        match name.as_str() {
            "Int32" => Ok(DataType::Int32),
            "void" => Ok(DataType::Void),
            _ => Ok(DataType::Custom(name)),
        }
    } else {
        Err("Expected a type")
    }
}
