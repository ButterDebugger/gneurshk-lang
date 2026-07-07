use super::{TokenStream, expressions::parse_expression};
use crate::{ElseBranch, IfStatement, block::parse_block, consume_all_newlines};
use anyhow::{Result, anyhow};
use gneurshk_lexer::tokens::Token;

pub fn parse_if_statement(tokens: &mut TokenStream) -> Result<IfStatement> {
    // Consume the If token
    match tokens.next() {
        Some((Token::If, _)) => {}
        _ => return Err(anyhow!("Expected if statement")),
    }

    // Parse the condition
    let condition = parse_expression(tokens)?;

    // Parse the body of the if statement
    let if_block = parse_block(tokens)?;

    // Consume all new line tokens
    consume_all_newlines(tokens);

    // Parse the else block if it exists
    let else_block = if let Some((Token::Else, _)) = tokens.peek() {
        tokens.next(); // Consume the Else token

        // Consume all new line tokens
        consume_all_newlines(tokens);

        // Determine what type of statement follows
        match tokens.peek() {
            Some((Token::If, _)) => Some(Box::new(ElseBranch::IfStatement(parse_if_statement(
                tokens,
            )?))),
            Some((Token::OpenBrace, _)) => Some(Box::new(ElseBranch::Block(parse_block(tokens)?))),
            _ => None,
        }
    } else {
        None
    };

    Ok(IfStatement {
        condition: Box::new(condition),
        if_block: Box::new(if_block),
        else_statement: else_block,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        BinaryExpression, BinaryOperator, Block, BooleanLit, ElseBranch, Expression,
        FunctionDeclaration, IfStatement, IntegerLit, Program, Stmt, VariableDeclaration, parse,
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
    fn nested_if_blocks() {
        let source = include_str!("../tests/ifs/nested_if_blocks.iv");
        let stmt = lex_then_parse(source);

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
                                            span: 21..23
                                        })),
                                        right: Box::new(Expression::Integer(IntegerLit {
                                            value: 10,
                                            span: 26..28
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
                                                            span: 42..44
                                                        }
                                                    )),
                                                    right: Box::new(Expression::Integer(
                                                        IntegerLit {
                                                            value: 20,
                                                            span: 47..49
                                                        }
                                                    )),
                                                    operator: BinaryOperator::Add,
                                                }
                                            )),
                                            if_block: Box::new(Block {
                                                body: vec![Stmt::VariableDeclaration(
                                                    VariableDeclaration::Mutable {
                                                        name: "apple".to_string(),
                                                        data_type: None,
                                                        value: Some(Expression::Integer(
                                                            IntegerLit {
                                                                value: 1,
                                                                span: 76..77
                                                            }
                                                        )),
                                                    }
                                                )]
                                            }),
                                            else_statement: None,
                                        }),
                                        Stmt::IfStatement(IfStatement {
                                            condition: Box::new(Expression::BinaryExpression(
                                                BinaryExpression {
                                                    left: Box::new(Expression::Integer(
                                                        IntegerLit {
                                                            value: 30,
                                                            span: 99..101
                                                        }
                                                    )),
                                                    right: Box::new(Expression::Integer(
                                                        IntegerLit {
                                                            value: 30,
                                                            span: 104..106
                                                        }
                                                    )),
                                                    operator: BinaryOperator::Add,
                                                }
                                            )),
                                            if_block: Box::new(Block {
                                                body: vec![Stmt::VariableDeclaration(
                                                    VariableDeclaration::Mutable {
                                                        name: "green".to_string(),
                                                        data_type: None,
                                                        value: Some(Expression::Integer(
                                                            IntegerLit {
                                                                value: 3,
                                                                span: 133..134
                                                            }
                                                        )),
                                                    }
                                                )]
                                            }),
                                            else_statement: None,
                                        })
                                    ]
                                }),
                                else_statement: None,
                            }),
                            Stmt::VariableDeclaration(VariableDeclaration::Constant {
                                name: "borg".to_string(),
                                data_type: None,
                                value: Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 168..169
                                }),
                            }),
                        ],
                    }),
                }],
            }
        );
    }
    #[test]
    fn else_block() {
        let source = include_str!("../tests/ifs/else_block.iv");
        let stmt = lex_then_parse(source);

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
                                    span: 21..23
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 10,
                                    span: 26..28
                                })),
                                operator: BinaryOperator::Add,
                            })),
                            if_block: Box::new(Block {
                                body: vec![Stmt::Integer(IntegerLit {
                                    value: 1,
                                    span: 39..40
                                })]
                            }),
                            else_statement: Some(Box::new(ElseBranch::Block(Block {
                                body: vec![Stmt::Integer(IntegerLit {
                                    value: 2,
                                    span: 62..63
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
        let source = include_str!("../tests/ifs/else_if_block.iv");
        let stmt = lex_then_parse(source);

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
                                    span: 21..23
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 10,
                                    span: 26..28
                                })),
                                operator: BinaryOperator::Add,
                            })),
                            if_block: Box::new(Block {
                                body: vec![Stmt::Integer(IntegerLit {
                                    value: 1,
                                    span: 39..40
                                })]
                            }),
                            else_statement: Some(Box::new(ElseBranch::IfStatement(IfStatement {
                                condition: Box::new(Expression::BinaryExpression(
                                    BinaryExpression {
                                        left: Box::new(Expression::Integer(IntegerLit {
                                            value: 20,
                                            span: 55..57
                                        })),
                                        right: Box::new(Expression::Integer(IntegerLit {
                                            value: 20,
                                            span: 60..62
                                        })),
                                        operator: BinaryOperator::Add,
                                    }
                                )),
                                if_block: Box::new(Block {
                                    body: vec![Stmt::Integer(IntegerLit {
                                        value: 2,
                                        span: 73..74
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
        let source = include_str!("../tests/ifs/else_if_else_block.iv");
        let stmt = lex_then_parse(source);

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
                                    span: 21..23
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 10,
                                    span: 26..28
                                })),
                                operator: BinaryOperator::Add,
                            })),
                            if_block: Box::new(Block {
                                body: vec![Stmt::Integer(IntegerLit {
                                    value: 1,
                                    span: 39..40
                                })]
                            }),
                            else_statement: Some(Box::new(ElseBranch::IfStatement(IfStatement {
                                condition: Box::new(Expression::BinaryExpression(
                                    BinaryExpression {
                                        left: Box::new(Expression::Integer(IntegerLit {
                                            value: 20,
                                            span: 55..57
                                        })),
                                        right: Box::new(Expression::Integer(IntegerLit {
                                            value: 20,
                                            span: 60..62
                                        })),
                                        operator: BinaryOperator::Add,
                                    }
                                )),
                                if_block: Box::new(Block {
                                    body: vec![Stmt::Integer(IntegerLit {
                                        value: 2,
                                        span: 73..74
                                    })]
                                }),
                                else_statement: Some(Box::new(ElseBranch::Block(Block {
                                    body: vec![Stmt::Integer(IntegerLit {
                                        value: 3,
                                        span: 96..97
                                    })]
                                }))),
                            }))),
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn if_else_on_new_lines() {
        let source = include_str!("../tests/ifs/if_else_on_new_lines.iv");
        let stmt = lex_then_parse(source);

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
                            condition: Box::new(Expression::Boolean(BooleanLit {
                                value: true,
                                span: 22..26
                            })),
                            if_block: Box::new(Block {
                                body: vec![Stmt::Integer(IntegerLit {
                                    value: 1,
                                    span: 43..44
                                })]
                            }),
                            else_statement: Some(Box::new(ElseBranch::Block(Block {
                                body: vec![Stmt::Integer(IntegerLit {
                                    value: 2,
                                    span: 78..79
                                })]
                            }))),
                        })],
                    }),
                }],
            }
        );
    }
}
