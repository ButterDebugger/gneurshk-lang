use super::{StatementResult, Stmt, TokenStream, expressions::parse_expression};
use crate::block::parse_block;
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
    let body = parse_block(tokens)?;

    // Parse the else block if it exists
    let else_block = if let Some((Token::Else, _)) = tokens.peek() {
        tokens.next(); // Consume the Else token

        // Determine what type of statement follows
        match tokens.peek() {
            Some((Token::If, _)) => Some(Box::new(parse_if_statement(tokens)?)),
            Some((Token::OpenBrace, _)) => Some(Box::new(parse_block(tokens)?)),
            _ => None,
        }
    } else {
        None
    };

    Ok(Stmt::IfStatement {
        condition: Box::new(condition),
        block: Box::new(body),
        else_block,
    })
}

#[cfg(test)]
mod tests {
    use crate::{BinaryOperator, Program, Stmt, parse};
    use gneurshk_lexer::lex;

    /// Helper function for testing the parse function
    fn lex_then_parse(input: &'static str) -> Program {
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
if 10 + 10 {
    var apple = 1








    var green = 3
}
const borg = 5
",
        )
        .body;

        assert_eq!(
            stmt,
            vec![
                Stmt::IfStatement {
                    condition: Box::new(Stmt::BinaryExpression {
                        left: Box::new(Stmt::Integer {
                            value: 10,
                            span: 4..6
                        }),
                        right: Box::new(Stmt::Integer {
                            value: 10,
                            span: 9..11
                        }),
                        operator: BinaryOperator::Add,
                    }),
                    block: Box::new(Stmt::Block {
                        body: vec![
                            Stmt::Declaration {
                                mutable: true,
                                name: "apple".to_string(),
                                data_type: None,
                                value: Some(Box::new(Stmt::Integer {
                                    value: 1,
                                    span: 30..31
                                })),
                            },
                            Stmt::Declaration {
                                mutable: true,
                                name: "green".to_string(),
                                data_type: None,
                                value: Some(Box::new(Stmt::Integer {
                                    value: 3,
                                    span: 56..57
                                })),
                            },
                        ],
                    }),
                    else_block: None,
                },
                Stmt::Declaration {
                    mutable: false,
                    name: "borg".to_string(),
                    data_type: None,
                    value: Some(Box::new(Stmt::Integer {
                        value: 5,
                        span: 73..74
                    })),
                },
            ]
        );
    }

    #[test]
    fn nested_if_blocks() {
        let stmt = lex_then_parse(
            r"
if 10 + 10 {
    if 20 + 20 {
        var apple = 1
    }
    if 30 + 30 {
        var green = 3
    }
}
const borg = 5
",
        )
        .body;

        assert_eq!(
            stmt,
            vec![
                Stmt::IfStatement {
                    condition: Box::new(Stmt::BinaryExpression {
                        left: Box::new(Stmt::Integer {
                            value: 10,
                            span: 4..6
                        }),
                        right: Box::new(Stmt::Integer {
                            value: 10,
                            span: 9..11
                        }),
                        operator: BinaryOperator::Add,
                    }),
                    block: Box::new(Stmt::Block {
                        body: vec![
                            Stmt::IfStatement {
                                condition: Box::new(Stmt::BinaryExpression {
                                    left: Box::new(Stmt::Integer {
                                        value: 20,
                                        span: 21..23
                                    }),
                                    right: Box::new(Stmt::Integer {
                                        value: 20,
                                        span: 26..28
                                    }),
                                    operator: BinaryOperator::Add,
                                }),
                                block: Box::new(Stmt::Block {
                                    body: vec![Stmt::Declaration {
                                        mutable: true,
                                        name: "apple".to_string(),
                                        data_type: None,
                                        value: Some(Box::new(Stmt::Integer {
                                            value: 1,
                                            span: 51..52
                                        })),
                                    }]
                                }),
                                else_block: None,
                            },
                            Stmt::IfStatement {
                                condition: Box::new(Stmt::BinaryExpression {
                                    left: Box::new(Stmt::Integer {
                                        value: 30,
                                        span: 66..68
                                    }),
                                    right: Box::new(Stmt::Integer {
                                        value: 30,
                                        span: 71..73
                                    }),
                                    operator: BinaryOperator::Add,
                                }),
                                block: Box::new(Stmt::Block {
                                    body: vec![Stmt::Declaration {
                                        mutable: true,
                                        name: "green".to_string(),
                                        data_type: None,
                                        value: Some(Box::new(Stmt::Integer {
                                            value: 3,
                                            span: 96..97
                                        })),
                                    }]
                                }),
                                else_block: None,
                            }
                        ]
                    }),
                    else_block: None,
                },
                Stmt::Declaration {
                    mutable: false,
                    name: "borg".to_string(),
                    data_type: None,
                    value: Some(Box::new(Stmt::Integer {
                        value: 5,
                        span: 119..120
                    })),
                },
            ]
        );
    }
    #[test]
    fn else_block() {
        let stmt = lex_then_parse(
            r"
if 10 + 10 {
    1
} else {
    2
}
",
        )
        .body;

        assert_eq!(
            stmt,
            vec![Stmt::IfStatement {
                condition: Box::new(Stmt::BinaryExpression {
                    left: Box::new(Stmt::Integer {
                        value: 10,
                        span: 4..6
                    }),
                    right: Box::new(Stmt::Integer {
                        value: 10,
                        span: 9..11
                    }),
                    operator: BinaryOperator::Add,
                }),
                block: Box::new(Stmt::Block {
                    body: vec![Stmt::Integer {
                        value: 1,
                        span: 18..19
                    }]
                }),
                else_block: Some(Box::new(Stmt::Block {
                    body: vec![Stmt::Integer {
                        value: 2,
                        span: 33..34
                    }]
                })),
            }]
        );
    }

    #[test]
    fn else_if_block() {
        let stmt = lex_then_parse(
            r"
if 10 + 10 {
    1
} else if 20 + 20 {
    2
}
",
        )
        .body;

        assert_eq!(
            stmt,
            vec![Stmt::IfStatement {
                condition: Box::new(Stmt::BinaryExpression {
                    left: Box::new(Stmt::Integer {
                        value: 10,
                        span: 4..6
                    }),
                    right: Box::new(Stmt::Integer {
                        value: 10,
                        span: 9..11
                    }),
                    operator: BinaryOperator::Add,
                }),
                block: Box::new(Stmt::Block {
                    body: vec![Stmt::Integer {
                        value: 1,
                        span: 18..19
                    }]
                }),
                else_block: Some(Box::new(Stmt::IfStatement {
                    condition: Box::new(Stmt::BinaryExpression {
                        left: Box::new(Stmt::Integer {
                            value: 20,
                            span: 30..32
                        }),
                        right: Box::new(Stmt::Integer {
                            value: 20,
                            span: 35..37
                        }),
                        operator: BinaryOperator::Add,
                    }),
                    block: Box::new(Stmt::Block {
                        body: vec![Stmt::Integer {
                            value: 2,
                            span: 44..45
                        }]
                    }),
                    else_block: None,
                })),
            }]
        );
    }

    #[test]
    fn else_if_else_block() {
        let stmt = lex_then_parse(
            r"
if 10 + 10 {
    1
} else if 20 + 20 {
    2
} else {
    3
}
",
        )
        .body;

        assert_eq!(
            stmt,
            vec![Stmt::IfStatement {
                condition: Box::new(Stmt::BinaryExpression {
                    left: Box::new(Stmt::Integer {
                        value: 10,
                        span: 4..6
                    }),
                    right: Box::new(Stmt::Integer {
                        value: 10,
                        span: 9..11
                    }),
                    operator: BinaryOperator::Add,
                }),
                block: Box::new(Stmt::Block {
                    body: vec![Stmt::Integer {
                        value: 1,
                        span: 18..19
                    }]
                }),
                else_block: Some(Box::new(Stmt::IfStatement {
                    condition: Box::new(Stmt::BinaryExpression {
                        left: Box::new(Stmt::Integer {
                            value: 20,
                            span: 30..32
                        }),
                        right: Box::new(Stmt::Integer {
                            value: 20,
                            span: 35..37
                        }),
                        operator: BinaryOperator::Add,
                    }),
                    block: Box::new(Stmt::Block {
                        body: vec![Stmt::Integer {
                            value: 2,
                            span: 44..45
                        }]
                    }),
                    else_block: Some(Box::new(Stmt::Block {
                        body: vec![Stmt::Integer {
                            value: 3,
                            span: 59..60
                        }]
                    })),
                })),
            }]
        );
    }
}
