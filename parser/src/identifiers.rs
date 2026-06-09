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
        let stmt = lex_then_parse(
            r#"
func main() {
    chicken
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
                        body: vec![Stmt::Identifier(Identifier {
                            name: "chicken".to_string(),
                            span: 19..26,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn repeated_identifiers() {
        let stmt = lex_then_parse(
            r#"
func main() {
    chicken chicken chicken chicken chicken chicken chicken chicken
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
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 19..26,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 27..34,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 35..42,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 43..50,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 51..58,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 59..66,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 67..74,
                            }),
                            Stmt::Identifier(Identifier {
                                name: "chicken".to_string(),
                                span: 75..82,
                            }),
                        ],
                    }),
                }],
            }
        );
    }

    #[test]
    fn function_call_no_args() {
        let stmt = lex_then_parse(
            r#"
func main() {
    foo()
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
                        body: vec![Stmt::FunctionCall(FunctionCall {
                            name: "foo".to_string(),
                            args: vec![],
                            span: 19..24,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn function_call_single_arg() {
        let stmt = lex_then_parse(
            r#"
func main() {
    bar(42)
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
                        body: vec![Stmt::FunctionCall(FunctionCall {
                            name: "bar".to_string(),
                            args: vec![Expression::Integer(IntegerLit {
                                value: 42,
                                span: 23..25
                            })],
                            span: 19..26,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn function_call_multiple_args() {
        let stmt = lex_then_parse(
            r#"
func main() {
    baz(1, 2, 3)
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
                        body: vec![Stmt::FunctionCall(FunctionCall {
                            name: "baz".to_string(),
                            args: vec![
                                Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 23..24
                                }),
                                Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 26..27
                                }),
                                Expression::Integer(IntegerLit {
                                    value: 3,
                                    span: 29..30
                                }),
                            ],
                            span: 19..31,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn function_call_with_expression_args() {
        let stmt = lex_then_parse(
            r#"
func main() {
    calculate(1 + (2 + 5), 3 * 4)
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
                        body: vec![Stmt::FunctionCall(FunctionCall {
                            name: "calculate".to_string(),
                            args: vec![
                                Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 1,
                                        span: 29..30
                                    })),
                                    right: Box::new(Expression::BinaryExpression(
                                        BinaryExpression {
                                            left: Box::new(Expression::Integer(IntegerLit {
                                                value: 2,
                                                span: 34..35
                                            })),
                                            right: Box::new(Expression::Integer(IntegerLit {
                                                value: 5,
                                                span: 38..39
                                            })),
                                            operator: BinaryOperator::Add,
                                        }
                                    )),
                                    operator: BinaryOperator::Add,
                                }),
                                Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 3,
                                        span: 42..43
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 4,
                                        span: 46..47
                                    })),
                                    operator: BinaryOperator::Multiply,
                                }),
                            ],
                            span: 19..48,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn member_access() {
        let stmt = lex_then_parse(
            r#"
func main() {
    foo.bar
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
                        body: vec![Stmt::MemberAccess(MemberAccess {
                            base: Box::new(MemberExpressionBase::Identifier(Identifier {
                                name: "foo".to_string(),
                                span: 19..22,
                            })),
                            member: MemberExpressionMember::Identifier(Identifier {
                                name: "bar".to_string(),
                                span: 23..26,
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
        let stmt = lex_then_parse(
            r#"
func main() {
    foo.bar.baz
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
                        body: vec![Stmt::MemberAccess(MemberAccess {
                            base: Box::new(MemberExpressionBase::MemberAccess(MemberAccess {
                                base: Box::new(MemberExpressionBase::Identifier(Identifier {
                                    name: "foo".to_string(),
                                    span: 19..22,
                                })),
                                member: MemberExpressionMember::Identifier(Identifier {
                                    name: "bar".to_string(),
                                    span: 23..26,
                                }),
                                is_static: false,
                            })),
                            member: MemberExpressionMember::Identifier(Identifier {
                                name: "baz".to_string(),
                                span: 27..30,
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
        let stmt = lex_then_parse(
            r#"
func main() {
    foo::bar().baz
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
                        body: vec![Stmt::MemberAccess(MemberAccess {
                            base: Box::new(MemberExpressionBase::MemberAccess(MemberAccess {
                                base: Box::new(MemberExpressionBase::Identifier(Identifier {
                                    name: "foo".to_string(),
                                    span: 19..22,
                                })),
                                member: MemberExpressionMember::FunctionCall(FunctionCall {
                                    name: "bar".to_string(),
                                    span: 24..29,
                                    args: vec![]
                                }),
                                is_static: true,
                            })),
                            member: MemberExpressionMember::Identifier(Identifier {
                                name: "baz".to_string(),
                                span: 30..33,
                            }),
                            is_static: false,
                        })],
                    }),
                }],
            }
        );
    }
}
