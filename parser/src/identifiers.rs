use crate::{
    FunctionCall, Identifier, MemberAccess, MemberExpressionBase, MemberExpressionMember,
    expressions::parse_expression,
};
use anyhow::{Result, anyhow};
use gneurshk_lexer::{TokenStream, tokens::Token};

// TODO: handle indexing

/// Parses function calls, identifiers, and member accessing
pub fn parse_member_expression_base(tokens: &mut TokenStream) -> Result<MemberExpressionBase> {
    // Capture the initial word
    let (name, word_span) = match tokens.next() {
        Some((Token::Word(name), span)) => (name, span),
        _ => return Err(anyhow!("Expected identifier")),
    };

    let mut base = match tokens.peek() {
        Some((Token::OpenParen, _)) => {
            tokens.next(); // Consume the opening parenthesis

            // Parse the arguments
            let mut args = Vec::new();

            let close_paren_end: usize;

            if let Some((Token::CloseParen, span)) = tokens.peek() {
                // Handle empty argument list
                close_paren_end = span.end;

                tokens.next(); // Consume the closing parenthesis
            } else {
                // Otherwise, loop while there are still arguments to parse
                loop {
                    let arg = parse_expression(tokens)?;
                    args.push(arg);

                    match tokens.peek() {
                        Some((Token::Comma, _)) => {
                            tokens.next();
                        }
                        Some((Token::CloseParen, span)) => {
                            close_paren_end = span.end;

                            tokens.next(); // Consume the closing parenthesis
                            break;
                        }
                        _ => {
                            return Err(anyhow!(
                                "Expected a comma or closing parenthesis in the function call"
                            ));
                        }
                    }
                }
            }

            MemberExpressionBase::FunctionCall(FunctionCall {
                name,
                args,
                span: word_span.start..close_paren_end,
            })
        }
        _ => MemberExpressionBase::Identifier(Identifier {
            name,
            span: word_span,
        }),
    };

    // Handle chaining with member accessing
    loop {
        // Check if its static or not
        let is_static = match tokens.peek() {
            Some((Token::Dot, _)) => {
                tokens.next();
                false
            }
            Some((Token::DoubleColon, _)) => {
                tokens.next();
                true
            }
            // Break early if chaining is over
            _ => break,
        };

        // Create the member
        let (member_name, member_span) = match tokens.next() {
            Some((Token::Word(name), span)) => (name, span),
            _ => return Err(anyhow!("Expected identifier after member access")),
        };

        let member = match tokens.peek() {
            Some((Token::OpenParen, _)) => {
                tokens.next(); // Consume the opening parenthesis

                // Parse the arguments
                let mut args = Vec::new();

                let close_paren_end: usize;

                if let Some((Token::CloseParen, span)) = tokens.peek() {
                    // Handle empty argument list
                    close_paren_end = span.end;

                    tokens.next(); // Consume the closing parenthesis
                } else {
                    // Otherwise, loop while there are still arguments to parse
                    loop {
                        let arg = parse_expression(tokens)?;
                        args.push(arg);

                        match tokens.peek() {
                            Some((Token::Comma, _)) => {
                                tokens.next();
                            }
                            Some((Token::CloseParen, span)) => {
                                close_paren_end = span.end;

                                tokens.next(); // Consume the closing parenthesis
                                break;
                            }
                            _ => {
                                return Err(anyhow!(
                                    "Expected a comma or closing parenthesis in the function call"
                                ));
                            }
                        }
                    }
                }

                MemberExpressionMember::FunctionCall(FunctionCall {
                    name: member_name,
                    args,
                    span: member_span.start..close_paren_end,
                })
            }
            _ => MemberExpressionMember::Identifier(Identifier {
                name: member_name,
                span: member_span,
            }),
        };

        // Nest the new base in a member access
        base = MemberExpressionBase::MemberAccess(MemberAccess {
            base: Box::new(base),
            member,
            is_static,
        });
    }

    Ok(base)
}

#[cfg(test)]
mod tests {
    use crate::Stmt::{self};
    use crate::{
        BinaryExpression, BinaryOperator, Block, Expression, FunctionCall, FunctionDeclaration,
        Identifier, IntegerLit, MemberAccess, MemberExpressionBase, MemberExpressionMember,
        Program, parse,
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
    fn single_identifier() {
        let source = include_str!("../tests/identifiers/single_identifier.iv");
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
                        body: vec![Stmt::Identifier(Identifier {
                            name: "chicken".to_string(),
                            span: 18..25,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn repeated_identifiers() {
        let source = include_str!("../tests/identifiers/repeated_identifiers.iv");
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
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 18..25,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 26..33,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 34..41,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 42..49,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 50..57,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 58..65,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 66..73,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 74..81,
                            }),
                        ],
                    }),
                }],
            }
        );
    }

    #[test]
    fn function_call_no_args() {
        let source = include_str!("../tests/identifiers/function_call_no_args.iv");
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
                        body: vec![Stmt::FunctionCall(FunctionCall {
                            name: "foo".to_string(),
                            args: vec![],
                            span: 18..23,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn function_call_single_arg() {
        let source = include_str!("../tests/identifiers/function_call_single_arg.iv");
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
                        body: vec![Stmt::FunctionCall(FunctionCall {
                            name: "bar".to_string(),
                            args: vec![Expression::Integer(IntegerLit {
                                value: 42,
                                span: 22..24
                            })],
                            span: 18..25,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn function_call_multiple_args() {
        let source = include_str!("../tests/identifiers/function_call_multiple_args.iv");
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
                        body: vec![Stmt::FunctionCall(FunctionCall {
                            name: "baz".to_string(),
                            args: vec![
                                Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 22..23
                                }),
                                Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 25..26
                                }),
                                Expression::Integer(IntegerLit {
                                    value: 3,
                                    span: 28..29
                                }),
                            ],
                            span: 18..30,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn function_call_with_expression_args() {
        let source = include_str!("../tests/identifiers/function_call_with_expression_args.iv");
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
                        body: vec![Stmt::FunctionCall(FunctionCall {
                            name: "calculate".to_string(),
                            args: vec![
                                Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 1,
                                        span: 28..29
                                    })),
                                    right: Box::new(Expression::BinaryExpression(
                                        BinaryExpression {
                                            left: Box::new(Expression::Integer(IntegerLit {
                                                value: 2,
                                                span: 33..34
                                            })),
                                            right: Box::new(Expression::Integer(IntegerLit {
                                                value: 5,
                                                span: 37..38
                                            })),
                                            operator: BinaryOperator::Add,
                                        }
                                    )),
                                    operator: BinaryOperator::Add,
                                }),
                                Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 3,
                                        span: 41..42
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 4,
                                        span: 45..46
                                    })),
                                    operator: BinaryOperator::Multiply,
                                }),
                            ],
                            span: 18..47,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn member_access() {
        let source = include_str!("../tests/identifiers/member_access.iv");
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
                        body: vec![Stmt::MemberAccess(MemberAccess {
                            base: Box::new(MemberExpressionBase::Identifier(Identifier {
                                name: "foo".to_string(),
                                span: 18..21,
                            })),
                            member: MemberExpressionMember::Identifier(Identifier {
                                name: "bar".to_string(),
                                span: 22..25,
                            }),
                            is_static: false,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn nested_member_access() {
        let source = include_str!("../tests/identifiers/nested_member_access.iv");
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
                        body: vec![Stmt::MemberAccess(MemberAccess {
                            base: Box::new(MemberExpressionBase::MemberAccess(MemberAccess {
                                base: Box::new(MemberExpressionBase::Identifier(Identifier {
                                    name: "foo".to_string(),
                                    span: 18..21,
                                })),
                                member: MemberExpressionMember::Identifier(Identifier {
                                    name: "bar".to_string(),
                                    span: 22..25,
                                }),
                                is_static: false,
                            })),
                            member: MemberExpressionMember::Identifier(Identifier {
                                name: "baz".to_string(),
                                span: 26..29,
                            }),
                            is_static: false,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn static_member_access_function_call() {
        let source = include_str!("../tests/identifiers/static_member_access_function_call.iv");
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
                        body: vec![Stmt::MemberAccess(MemberAccess {
                            base: Box::new(MemberExpressionBase::MemberAccess(MemberAccess {
                                base: Box::new(MemberExpressionBase::Identifier(Identifier {
                                    name: "foo".to_string(),
                                    span: 18..21,
                                })),
                                member: MemberExpressionMember::FunctionCall(FunctionCall {
                                    name: "bar".to_string(),
                                    span: 23..28,
                                    args: vec![]
                                }),
                                is_static: true,
                            })),
                            member: MemberExpressionMember::Identifier(Identifier {
                                name: "baz".to_string(),
                                span: 29..32,
                            }),
                            is_static: false,
                        })],
                    }),
                }],
            }
        );
    }
}
