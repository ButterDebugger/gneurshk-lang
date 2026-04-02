use crate::{
    FunctionCall, Identifier, MemberAccess, MemberExpressionBase, MemberExpressionMember,
    expressions::parse_expression,
};
use gneurshk_lexer::{TokenStream, tokens::Token};

// TODO: handle indexing

/// Parses function calls, identifiers, and member accessing
pub fn parse_member_expression_base(
    tokens: &mut TokenStream,
) -> Result<MemberExpressionBase, &'static str> {
    // Capture the initial word
    let (name, word_span) = match tokens.next() {
        Some((Token::Word(name), span)) => (name, span),
        _ => return Err(""),
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
                            return Err(
                                "Expected a comma or closing parenthesis in the function call",
                            );
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
            _ => return Err("Expected identifier after member access"),
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
                                return Err(
                                    "Expected a comma or closing parenthesis in the function call",
                                );
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
        BinaryExpression, BinaryOperator, Expression, FunctionCall, Identifier, IntegerLit,
        MemberAccess, MemberExpressionBase, MemberExpressionMember, Program, parse,
    };
    use gneurshk_lexer::lex;

    /// Helper function for testing the parse_expression function
    fn lex_then_parse(input: &'static str) -> Program {
        let tokens = lex(input).expect("Failed to lex");

        match parse(&mut tokens.clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {e}"),
        }
    }

    #[test]
    fn single_identifier() {
        let stmt = lex_then_parse("chicken").body;

        assert_eq!(
            stmt,
            vec![Stmt::Identifier(Identifier {
                name: "chicken".to_string(),
                span: 0..7,
            })]
        );
    }

    #[test]
    fn repeated_identifiers() {
        let stmt =
            lex_then_parse("chicken chicken chicken chicken chicken chicken chicken chicken").body;

        assert_eq!(
            stmt,
            vec![
                Stmt::Identifier(Identifier {
                    name: "chicken".to_string(),
                    span: 0..7,
                }),
                Stmt::Identifier(Identifier {
                    name: "chicken".to_string(),
                    span: 8..15,
                }),
                Stmt::Identifier(Identifier {
                    name: "chicken".to_string(),
                    span: 16..23,
                }),
                Stmt::Identifier(Identifier {
                    name: "chicken".to_string(),
                    span: 24..31,
                }),
                Stmt::Identifier(Identifier {
                    name: "chicken".to_string(),
                    span: 32..39,
                }),
                Stmt::Identifier(Identifier {
                    name: "chicken".to_string(),
                    span: 40..47,
                }),
                Stmt::Identifier(Identifier {
                    name: "chicken".to_string(),
                    span: 48..55,
                }),
                Stmt::Identifier(Identifier {
                    name: "chicken".to_string(),
                    span: 56..63,
                }),
            ]
        );
    }

    #[test]
    fn function_call_no_args() {
        let stmt = lex_then_parse("foo()").body;

        assert_eq!(
            stmt,
            vec![Stmt::FunctionCall(FunctionCall {
                name: "foo".to_string(),
                args: vec![],
                span: 0..5,
            })]
        );
    }

    #[test]
    fn function_call_single_arg() {
        let stmt = lex_then_parse("bar(42)").body;

        assert_eq!(
            stmt,
            vec![Stmt::FunctionCall(FunctionCall {
                name: "bar".to_string(),
                args: vec![Expression::Integer(IntegerLit {
                    value: 42,
                    span: 4..6
                })],
                span: 0..7,
            })]
        );
    }

    #[test]
    fn function_call_multiple_args() {
        let stmt = lex_then_parse("baz(1, 2, 3)").body;

        assert_eq!(
            stmt,
            vec![Stmt::FunctionCall(FunctionCall {
                name: "baz".to_string(),
                args: vec![
                    Expression::Integer(IntegerLit {
                        value: 1,
                        span: 4..5
                    }),
                    Expression::Integer(IntegerLit {
                        value: 2,
                        span: 7..8
                    }),
                    Expression::Integer(IntegerLit {
                        value: 3,
                        span: 10..11
                    }),
                ],
                span: 0..12,
            })]
        );
    }

    #[test]
    fn function_call_with_expression_args() {
        let stmt = lex_then_parse("calculate(1 + (2 + 5), 3 * 4)").body;

        assert_eq!(
            stmt,
            vec![Stmt::FunctionCall(FunctionCall {
                name: "calculate".to_string(),
                args: vec![
                    Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(Expression::Integer(IntegerLit {
                            value: 1,
                            span: 10..11
                        })),
                        right: Box::new(Expression::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::Integer(IntegerLit {
                                value: 2,
                                span: 15..16
                            })),
                            right: Box::new(Expression::Integer(IntegerLit {
                                value: 5,
                                span: 19..20
                            })),
                            operator: BinaryOperator::Add,
                        })),
                        operator: BinaryOperator::Add,
                    }),
                    Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(Expression::Integer(IntegerLit {
                            value: 3,
                            span: 23..24
                        })),
                        right: Box::new(Expression::Integer(IntegerLit {
                            value: 4,
                            span: 27..28
                        })),
                        operator: BinaryOperator::Multiply,
                    }),
                ],
                span: 0..29,
            })]
        );
    }

    #[test]
    fn member_access() {
        let stmt = lex_then_parse("foo.bar").body;

        assert_eq!(
            stmt,
            vec![Stmt::MemberAccess(MemberAccess {
                base: Box::new(MemberExpressionBase::Identifier(Identifier {
                    name: "foo".to_string(),
                    span: 0..3,
                })),
                member: MemberExpressionMember::Identifier(Identifier {
                    name: "bar".to_string(),
                    span: 4..7,
                }),
                is_static: false,
            })]
        );
    }

    #[test]
    fn nested_member_access() {
        let stmt = lex_then_parse("foo.bar.baz").body;

        assert_eq!(
            stmt,
            vec![Stmt::MemberAccess(MemberAccess {
                base: Box::new(MemberExpressionBase::MemberAccess(MemberAccess {
                    base: Box::new(MemberExpressionBase::Identifier(Identifier {
                        name: "foo".to_string(),
                        span: 0..3,
                    })),
                    member: MemberExpressionMember::Identifier(Identifier {
                        name: "bar".to_string(),
                        span: 4..7,
                    }),
                    is_static: false,
                })),
                member: MemberExpressionMember::Identifier(Identifier {
                    name: "baz".to_string(),
                    span: 8..11,
                }),
                is_static: false,
            })]
        );
    }

    #[test]
    fn static_member_access_function_call() {
        let stmt = lex_then_parse("foo::bar().baz").body;

        assert_eq!(
            stmt,
            vec![Stmt::MemberAccess(MemberAccess {
                base: Box::new(MemberExpressionBase::MemberAccess(MemberAccess {
                    base: Box::new(MemberExpressionBase::Identifier(Identifier {
                        name: "foo".to_string(),
                        span: 0..3,
                    })),
                    member: MemberExpressionMember::FunctionCall(FunctionCall {
                        name: "bar".to_string(),
                        span: 5..10,
                        args: vec![]
                    }),
                    is_static: true,
                })),
                member: MemberExpressionMember::Identifier(Identifier {
                    name: "baz".to_string(),
                    span: 11..14,
                }),
                is_static: false,
            })]
        );
    }
}
