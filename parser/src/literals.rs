use crate::{BooleanLit, FloatLit, IntegerLit, StringLit};
use anyhow::{Result, anyhow};
use gneurshk_lexer::{TokenStream, tokens::Token};

pub fn parse_integer_literal(tokens: &mut TokenStream) -> Result<IntegerLit> {
    match tokens.next() {
        Some((Token::Integer(value), span)) => Ok(IntegerLit { value, span }),
        _ => Err(anyhow!("Expected integer literal")),
    }
}

pub fn parse_float_literal(tokens: &mut TokenStream) -> Result<FloatLit> {
    match tokens.next() {
        Some((Token::Float(value), span)) => Ok(FloatLit { value, span }),
        _ => Err(anyhow!("Expected float literal")),
    }
}

pub fn parse_boolean_literal(tokens: &mut TokenStream) -> Result<BooleanLit> {
    match tokens.next() {
        Some((Token::Boolean(value), span)) => Ok(BooleanLit { value, span }),
        _ => Err(anyhow!("Expected boolean literal")),
    }
}

pub fn parse_string_literal(tokens: &mut TokenStream) -> Result<StringLit> {
    match tokens.next() {
        Some((Token::String(value), span)) => Ok(StringLit { value, span }),
        _ => Err(anyhow!("Expected string literal")),
    }
}
