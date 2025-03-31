use super::{
    expressions::parse_expression, parse_indented_body, StatementResult, Stmt, TokenStream,
};
use crate::lexer::tokens::Token;

pub fn parse_if_statement(tokens: &mut TokenStream) -> StatementResult {
    // Consume the If token
    match tokens.next() {
        Some(Token::If) => {}
        _ => return Err("Expected if statement"),
    }

    // Parse the condition
    let condition = match parse_expression(tokens) {
        Ok(e) => e,
        _ => return Err("Expected expression"),
    };

    // Expect a colon after the condition
    match tokens.next() {
        Some(Token::Colon) => {}
        _ => return Err("Expected colon after if condition"),
    }

    // Parse the body of the if statement
    let body = parse_indented_body(tokens)?;

    Ok(Stmt::IfStatement {
        condition: Box::new(condition),
        body,
    })
}
