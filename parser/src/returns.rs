use crate::{Return, Stmt, expressions::parse_expression};
use anyhow::{Result, anyhow};
use gneurshk_lexer::{TokenStream, tokens::Token};

pub fn parse_return_statement(tokens: &mut TokenStream) -> Result<Stmt> {
    // Consume the Return token
    match tokens.next() {
        Some((Token::Return, _)) => {}
        _ => return Err(anyhow!("Expected return statement")),
    }

    // Check if the next token is something that can be parsed as an expression
    let value = match tokens.peek() {
        Some((Token::Integer(_), _)) | Some((Token::OpenParen, _)) | Some((Token::Word(_), _)) => {
            Some(parse_expression(tokens)?)
        }
        _ => None,
    };

    Ok(Stmt::Return(Return { value }))
}

#[cfg(test)]
mod tests {
    use crate::{
        BinaryExpression, BinaryOperator, Block, Expression, FunctionDeclaration, IntegerLit,
        Program, Return, Stmt, parse,
    };
    use gneurshk_lexer::lex;

    /// Helper function for testing the parse function
    fn lex_then_parse(input: &'static str) -> Program {
        let tokens = lex(input).expect("Failed to lex");

        match parse(&mut tokens.clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {e}"),
        }
    }

    #[test]
    fn return_nothing() {
        let stmt = lex_then_parse(
            r#"
func main() {
    return
}
            "#,
        );

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block {
                        body: vec![Stmt::Return(Return { value: None })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn return_literal() {
        let stmt = lex_then_parse(
            r#"
func main() {
    return 1
}
            "#,
        );

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block {
                        body: vec![Stmt::Return(Return {
                            value: Some(Expression::Integer(IntegerLit {
                                value: 1,
                                span: 26..27
                            }))
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn return_expression() {
        let stmt = lex_then_parse(
            r#"
func main() {
    return 1 + 2
}
            "#,
        );

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block {
                        body: vec![Stmt::Return(Return {
                            value: Some(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 26..27
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 30..31
                                })),
                                operator: BinaryOperator::Add,
                            }))
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn return_nothing_in_a_block() {
        let stmt = lex_then_parse(
            r#"
func main() {
    {
        return
    }
}
            "#,
        );

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block {
                        body: vec![Stmt::Block(Block {
                            body: vec![Stmt::Return(Return { value: None })]
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn return_literal_in_a_block() {
        let stmt = lex_then_parse(
            r#"
func main() {
    { return 1 }
}
            "#,
        );

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block {
                        body: vec![Stmt::Block(Block {
                            body: vec![Stmt::Return(Return {
                                value: Some(Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 28..29
                                }))
                            })]
                        })],
                    }),
                }],
            }
        );
    }
}
