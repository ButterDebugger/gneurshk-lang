use crate::{StatementResult, Stmt, expressions::parse_expression};
use gneurshk_lexer::{TokenStream, tokens::Token};

pub fn parse_identifier_or_function_call(tokens: &mut TokenStream) -> StatementResult {
    match tokens.next() {
        Some((Token::Word(name), word_span)) => {
            if let Some((Token::OpenParen, _)) = tokens.peek() {
                tokens.next(); // Consume the opening parenthesis

                // Parse the arguments
                let mut args = Vec::new();

                // Handle empty argument list
                if let Some((Token::CloseParen, close_paren_span)) = tokens.peek() {
                    let close_paren_span = close_paren_span.clone();

                    tokens.next(); // Consume the closing parenthesis
                    return Ok(Stmt::FunctionCall {
                        name,
                        args,
                        span: word_span.start..close_paren_span.end,
                    });
                }

                // Loop while there are still arguments to parse
                let close_paren_end: usize;

                loop {
                    // Parse the argument as an expression
                    let arg = parse_expression(tokens)?;
                    args.push(arg);

                    // Check for comma or closing parenthesis
                    match tokens.peek() {
                        Some((Token::Comma, _)) => {
                            tokens.next(); // Consume the comma
                        }
                        Some((Token::CloseParen, close_paren_span)) => {
                            close_paren_end = close_paren_span.end;

                            tokens.next(); // Consume the closing parenthesis
                            break;
                        }
                        _ => {
                            return Err(
                                "Expected a comma or closing parenthesis in the function call",
                            );
                        }
                    }
                }

                Ok(Stmt::FunctionCall {
                    name,
                    args,
                    span: word_span.start..close_paren_end,
                })
            } else {
                Ok(Stmt::Identifier {
                    name,
                    span: word_span,
                })
            }
        }
        _ => Err("Expected identifier"),
    }
}
