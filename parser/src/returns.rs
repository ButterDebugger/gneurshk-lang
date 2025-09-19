use crate::{StatementResult, Stmt, expressions::parse_expression};
use gneurshk_lexer::{TokenStream, tokens::Token};

pub fn parse_return_statement(tokens: &mut TokenStream) -> StatementResult {
    // Consume the Return token
    match tokens.next() {
        Some((Token::Return, _)) => {}
        _ => return Err("Expected return statement"),
    }

    // Check if the next token is something that can be parsed as an expression
    let value = match tokens.peek() {
        Some((Token::Integer(_), _)) | Some((Token::OpenParen, _)) | Some((Token::Word(_), _)) => {
            Some(Box::new(parse_expression(tokens)?))
        }
        _ => None,
    };

    Ok(Stmt::ReturnStatement { value })
}

#[cfg(test)]
mod tests {
    use crate::BinaryOperator;
    use crate::Stmt;
    use crate::parse;
    use gneurshk_lexer::lex;

    /// Helper function for testing the parse function
    fn lex_then_parse(input: &'static str) -> Vec<Stmt> {
        let tokens = lex(input).expect("Failed to lex");

        println!("tokens {tokens:?}");

        match parse(&mut tokens.clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {e}"),
        }
    }

    #[test]
    fn return_nothing() {
        let stmt = lex_then_parse("return");

        assert_eq!(stmt, vec![Stmt::ReturnStatement { value: None }]);
    }

    #[test]
    fn return_literal() {
        let stmt = lex_then_parse("return 1");

        assert_eq!(
            stmt,
            vec![Stmt::ReturnStatement {
                value: Some(Box::new(Stmt::Literal { value: 1 }))
            }]
        );
    }

    #[test]
    fn return_expression() {
        let stmt = lex_then_parse("return 1 + 2");

        assert_eq!(
            stmt,
            vec![Stmt::ReturnStatement {
                value: Some(Box::new(Stmt::BinaryExpression {
                    left: Box::new(Stmt::Literal { value: 1 }),
                    right: Box::new(Stmt::Literal { value: 2 }),
                    operator: BinaryOperator::Add,
                }))
            }]
        );
    }

    #[test]
    fn return_nothing_in_a_block() {
        let stmt = lex_then_parse("{ return }");

        assert_eq!(
            stmt,
            vec![Stmt::Block {
                body: vec![Stmt::ReturnStatement { value: None }]
            }]
        );
    }

    #[test]
    fn return_literal_in_a_block() {
        let stmt = lex_then_parse("{ return 1 }");

        assert_eq!(
            stmt,
            vec![Stmt::Block {
                body: vec![Stmt::ReturnStatement {
                    value: Some(Box::new(Stmt::Literal { value: 1 }))
                }]
            }]
        );
    }
}
