use crate::{StatementResult, Stmt};
use gneurshk_lexer::{TokenStream, tokens::Token};

pub fn parse_string(tokens: &mut TokenStream) -> StatementResult {
    match tokens.next() {
        Some((Token::String(value), _)) => Ok(Stmt::String { value }),
        _ => Err("Expected string"),
    }
}
