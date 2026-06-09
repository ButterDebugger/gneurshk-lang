use super::{Stmt, TokenStream, expressions::parse_expression};
use crate::{IfStatement, block::parse_block};
use anyhow::{Result, anyhow};
use gneurshk_lexer::tokens::Token;

pub fn parse_if_statement(tokens: &mut TokenStream) -> Result<Stmt> {
    // Consume the If token
    match tokens.next() {
        Some((Token::If, _)) => {}
        _ => return Err(anyhow!("Expected if statement")),
    }

    // Parse the condition
    let condition = parse_expression(tokens)?;

    // Parse the body of the if statement
    let if_block = parse_block(tokens)?;

    // Parse the else block if it exists
    let else_block = if let Some((Token::Else, _)) = tokens.peek() {
        tokens.next(); // Consume the Else token

        // Determine what type of statement follows
        match tokens.peek() {
            Some((Token::If, _)) => Some(Box::new(parse_if_statement(tokens)?)),
            Some((Token::OpenBrace, _)) => Some(Box::new(Stmt::Block(parse_block(tokens)?))),
            _ => None,
        }
    } else {
        None
    };

    Ok(Stmt::IfStatement(IfStatement {
        condition: Box::new(condition),
        if_block: Box::new(if_block),
        else_statement: else_block,
    }))
}

#[cfg(test)]
mod tests {
    use crate::{
        BinaryExpression, BinaryOperator, Block, Expression, FunctionDeclaration, IfStatement,
        IntegerLit, Program, Stmt, parse,
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
    fn large_indented_if_block() {
        let stmt = lex_then_parse(
            r#"
func main() {

    if 10 + 10 {
        var apple = 1



        var green = 3
    }
    const borg = 5

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
                        body: vec![
                            Stmt::IfStatement(IfStatement {
                                condition: Box::new(Expression::BinaryExpression(
                                    BinaryExpression {
                                        left: Box::new(Expression::Integer(IntegerLit {
                                            value: 10,
                                            span: 23..25
                                        })),
                                        right: Box::new(Expression::Integer(IntegerLit {
                                            value: 10,
                                            span: 28..30
                                        })),
                                        operator: BinaryOperator::Add,
                                    }
                                )),
                                if_block: Box::new(Block {
                                    body: vec![
                                        Stmt::Declaration {
                                            mutable: true,
                                            name: "apple".to_string(),
                                            data_type: None,
                                            value: Some(Expression::Integer(IntegerLit {
                                                value: 1,
                                                span: 53..54
                                            })),
                                        },
                                        Stmt::Declaration {
                                            mutable: true,
                                            name: "green".to_string(),
                                            data_type: None,
                                            value: Some(Expression::Integer(IntegerLit {
                                                value: 3,
                                                span: 78..79
                                            })),
                                        },
                                    ],
                                }),
                                else_statement: None,
                            }),
                            Stmt::Declaration {
                                mutable: false,
                                name: "borg".to_string(),
                                data_type: None,
                                value: Some(Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 103..104
                                })),
                            },
                        ],
                    }),
                }],
            }
        );
    }

    #[test]
    fn nested_if_blocks() {
        let stmt = lex_then_parse(
            r#"
func main() {

    if 10 + 10 {
        if 20 + 20 {
            var apple = 1
        }
        if 30 + 30 {
            var green = 3
        }
    }
    const borg = 5

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
                        body: vec![
                            Stmt::IfStatement(IfStatement {
                                condition: Box::new(Expression::BinaryExpression(
                                    BinaryExpression {
                                        left: Box::new(Expression::Integer(IntegerLit {
                                            value: 10,
                                            span: 23..25
                                        })),
                                        right: Box::new(Expression::Integer(IntegerLit {
                                            value: 10,
                                            span: 28..30
                                        })),
                                        operator: BinaryOperator::Add,
                                    }
                                )),
                                if_block: Box::new(Block {
                                    body: vec![
                                        Stmt::IfStatement(IfStatement {
                                            condition: Box::new(Expression::BinaryExpression(
                                                BinaryExpression {
                                                    left: Box::new(Expression::Integer(
                                                        IntegerLit {
                                                            value: 20,
                                                            span: 44..46
                                                        }
                                                    )),
                                                    right: Box::new(Expression::Integer(
                                                        IntegerLit {
                                                            value: 20,
                                                            span: 49..51
                                                        }
                                                    )),
                                                    operator: BinaryOperator::Add,
                                                }
                                            )),
                                            if_block: Box::new(Block {
                                                body: vec![Stmt::Declaration {
                                                    mutable: true,
                                                    name: "apple".to_string(),
                                                    data_type: None,
                                                    value: Some(Expression::Integer(IntegerLit {
                                                        value: 1,
                                                        span: 78..79
                                                    })),
                                                }]
                                            }),
                                            else_statement: None,
                                        }),
                                        Stmt::IfStatement(IfStatement {
                                            condition: Box::new(Expression::BinaryExpression(
                                                BinaryExpression {
                                                    left: Box::new(Expression::Integer(
                                                        IntegerLit {
                                                            value: 30,
                                                            span: 101..103
                                                        }
                                                    )),
                                                    right: Box::new(Expression::Integer(
                                                        IntegerLit {
                                                            value: 30,
                                                            span: 106..108
                                                        }
                                                    )),
                                                    operator: BinaryOperator::Add,
                                                }
                                            )),
                                            if_block: Box::new(Block {
                                                body: vec![Stmt::Declaration {
                                                    mutable: true,
                                                    name: "green".to_string(),
                                                    data_type: None,
                                                    value: Some(Expression::Integer(IntegerLit {
                                                        value: 3,
                                                        span: 135..136
                                                    })),
                                                }]
                                            }),
                                            else_statement: None,
                                        })
                                    ]
                                }),
                                else_statement: None,
                            }),
                            Stmt::Declaration {
                                mutable: false,
                                name: "borg".to_string(),
                                data_type: None,
                                value: Some(Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 170..171
                                })),
                            },
                        ],
                    }),
                }],
            }
        );
    }
    #[test]
    fn else_block() {
        let stmt = lex_then_parse(
            r#"
func main() {

    if 10 + 10 {
        1
    } else {
        2
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
                        body: vec![Stmt::IfStatement(IfStatement {
                            condition: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 10,
                                    span: 23..25
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 10,
                                    span: 28..30
                                })),
                                operator: BinaryOperator::Add,
                            })),
                            if_block: Box::new(Block {
                                body: vec![Stmt::Integer(IntegerLit {
                                    value: 1,
                                    span: 41..42
                                })]
                            }),
                            else_statement: Some(Box::new(Stmt::Block(Block {
                                body: vec![Stmt::Integer(IntegerLit {
                                    value: 2,
                                    span: 64..65
                                })]
                            }))),
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn else_if_block() {
        let stmt = lex_then_parse(
            r#"
func main() {

    if 10 + 10 {
        1
    } else if 20 + 20 {
        2
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
                        body: vec![Stmt::IfStatement(IfStatement {
                            condition: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 10,
                                    span: 23..25
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 10,
                                    span: 28..30
                                })),
                                operator: BinaryOperator::Add,
                            })),
                            if_block: Box::new(Block {
                                body: vec![Stmt::Integer(IntegerLit {
                                    value: 1,
                                    span: 41..42
                                })]
                            }),
                            else_statement: Some(Box::new(Stmt::IfStatement(IfStatement {
                                condition: Box::new(Expression::BinaryExpression(
                                    BinaryExpression {
                                        left: Box::new(Expression::Integer(IntegerLit {
                                            value: 20,
                                            span: 57..59
                                        })),
                                        right: Box::new(Expression::Integer(IntegerLit {
                                            value: 20,
                                            span: 62..64
                                        })),
                                        operator: BinaryOperator::Add,
                                    }
                                )),
                                if_block: Box::new(Block {
                                    body: vec![Stmt::Integer(IntegerLit {
                                        value: 2,
                                        span: 75..76
                                    })]
                                }),
                                else_statement: None,
                            }))),
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn else_if_else_block() {
        let stmt = lex_then_parse(
            r#"
func main() {

    if 10 + 10 {
        1
    } else if 20 + 20 {
        2
    } else {
        3
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
                        body: vec![Stmt::IfStatement(IfStatement {
                            condition: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 10,
                                    span: 23..25
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 10,
                                    span: 28..30
                                })),
                                operator: BinaryOperator::Add,
                            })),
                            if_block: Box::new(Block {
                                body: vec![Stmt::Integer(IntegerLit {
                                    value: 1,
                                    span: 41..42
                                })]
                            }),
                            else_statement: Some(Box::new(Stmt::IfStatement(IfStatement {
                                condition: Box::new(Expression::BinaryExpression(
                                    BinaryExpression {
                                        left: Box::new(Expression::Integer(IntegerLit {
                                            value: 20,
                                            span: 57..59
                                        })),
                                        right: Box::new(Expression::Integer(IntegerLit {
                                            value: 20,
                                            span: 62..64
                                        })),
                                        operator: BinaryOperator::Add,
                                    }
                                )),
                                if_block: Box::new(Block {
                                    body: vec![Stmt::Integer(IntegerLit {
                                        value: 2,
                                        span: 75..76
                                    })]
                                }),
                                else_statement: Some(Box::new(Stmt::Block(Block {
                                    body: vec![Stmt::Integer(IntegerLit {
                                        value: 3,
                                        span: 98..99
                                    })]
                                }))),
                            }))),
                        })],
                    }),
                }],
            }
        );
    }
}
