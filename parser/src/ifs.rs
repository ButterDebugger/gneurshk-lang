use super::{
    StatementResult, Stmt, TokenStream, expressions::parse_expression, parse_wrapped_body,
};
use gneurshk_lexer::tokens::Token;

pub fn parse_if_statement(tokens: &mut TokenStream) -> StatementResult {
    // Consume the If token
    match tokens.next() {
        Some((Token::If, _)) => {}
        _ => return Err("Expected if statement"),
    }

    // Parse the condition
    let condition = parse_expression(tokens)?;

    // Parse the body of the if statement
    let body = parse_wrapped_body(tokens)?;

    Ok(Stmt::IfStatement {
        condition: Box::new(condition),
        body,
    })
}

#[cfg(test)]
mod tests {
    use crate::Operator;
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
    fn large_indented_if_block() {
        let stmt = lex_then_parse(
            r"
if 10 + 10:
    var apple = 1








    var green = 3

const borg = 5
",
        );

        assert_eq!(
            stmt,
            vec![
                Stmt::IfStatement {
                    condition: Box::new(Stmt::BinaryExpression {
                        left: Box::new(Stmt::Literal { value: 10 }),
                        right: Box::new(Stmt::Literal { value: 10 }),
                        operator: Operator::Add,
                    }),
                    body: vec![
                        Stmt::Declaration {
                            mutable: true,
                            name: "apple".to_string(),
                            value: Some(Box::new(Stmt::Literal { value: 1 })),
                        },
                        Stmt::Declaration {
                            mutable: true,
                            name: "green".to_string(),
                            value: Some(Box::new(Stmt::Literal { value: 3 })),
                        },
                    ],
                },
                Stmt::Declaration {
                    mutable: false,
                    name: "borg".to_string(),
                    value: Some(Box::new(Stmt::Literal { value: 5 })),
                },
            ]
        );
    }

    #[test]
    fn nested_if_blocks() {
        let stmt = lex_then_parse(
            r"
if 10 + 10:
    if 20 + 20:
        var apple = 1

    if 30 + 30:
        var green = 3

const borg = 5
",
        );

        assert_eq!(
            stmt,
            vec![
                Stmt::IfStatement {
                    condition: Box::new(Stmt::BinaryExpression {
                        left: Box::new(Stmt::Literal { value: 10 }),
                        right: Box::new(Stmt::Literal { value: 10 }),
                        operator: Operator::Add,
                    }),
                    body: vec![
                        Stmt::IfStatement {
                            condition: Box::new(Stmt::BinaryExpression {
                                left: Box::new(Stmt::Literal { value: 20 }),
                                right: Box::new(Stmt::Literal { value: 20 }),
                                operator: Operator::Add,
                            }),
                            body: vec![Stmt::Declaration {
                                mutable: true,
                                name: "apple".to_string(),
                                value: Some(Box::new(Stmt::Literal { value: 1 })),
                            }],
                        },
                        Stmt::IfStatement {
                            condition: Box::new(Stmt::BinaryExpression {
                                left: Box::new(Stmt::Literal { value: 30 }),
                                right: Box::new(Stmt::Literal { value: 30 }),
                                operator: Operator::Add,
                            }),
                            body: vec![Stmt::Declaration {
                                mutable: true,
                                name: "green".to_string(),
                                value: Some(Box::new(Stmt::Literal { value: 3 })),
                            }],
                        }
                    ],
                },
                Stmt::Declaration {
                    mutable: false,
                    name: "borg".to_string(),
                    value: Some(Box::new(Stmt::Literal { value: 5 })),
                },
            ]
        );
    }
}
