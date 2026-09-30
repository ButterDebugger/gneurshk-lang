use crate::{
    Assignment, BinaryExpression, BinaryOperator, Expression, Stmt, expressions::parse_expression,
    identifiers::parse_member_expression_base,
};
use anyhow::{Result, anyhow};
use gneurshk_lexer::{TokenStream, tokens::Token};

pub fn parse_assignment(tokens: &mut TokenStream) -> Result<Stmt> {
    // Read the variable
    let member = parse_member_expression_base(tokens)?;

    // Read the expression
    let expr = match tokens.next() {
        Some((Token::Equal, _)) => parse_expression(tokens)?,
        Some((Token::PlusEqual, _)) => Expression::BinaryExpression(BinaryExpression {
            left: Box::new(member.clone().into()),
            right: Box::new(parse_expression(tokens)?),
            operator: BinaryOperator::Add,
        }),
        Some((Token::MinusEqual, _)) => Expression::BinaryExpression(BinaryExpression {
            left: Box::new(member.clone().into()),
            right: Box::new(parse_expression(tokens)?),
            operator: BinaryOperator::Subtract,
        }),
        Some((Token::MultiplyEqual, _)) => Expression::BinaryExpression(BinaryExpression {
            left: Box::new(member.clone().into()),
            right: Box::new(parse_expression(tokens)?),
            operator: BinaryOperator::Multiply,
        }),
        Some((Token::DivideEqual, _)) => Expression::BinaryExpression(BinaryExpression {
            left: Box::new(member.clone().into()),
            right: Box::new(parse_expression(tokens)?),
            operator: BinaryOperator::Divide,
        }),
        Some((Token::ModulusEqual, _)) => Expression::BinaryExpression(BinaryExpression {
            left: Box::new(member.clone().into()),
            right: Box::new(parse_expression(tokens)?),
            operator: BinaryOperator::Modulus,
        }),
        _ => return Err(anyhow!("Expected assignment operator")),
    };

    // Return the assignment
    Ok(Stmt::Assignment(Assignment {
        member,
        value: expr,
    }))
}

#[cfg(test)]
mod tests {
    use crate::{
        Assignment, BinaryExpression, BinaryOperator, Block, Expression, FunctionDeclaration,
        Identifier, IntegerLit, MemberExpressionBase, Program, Stmt, parse,
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
    fn regular_assignment() {
        let source = include_str!("../tests/assignments/regular_assignment.iv");
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
                        body: vec![Stmt::Assignment(Assignment {
                            member: MemberExpressionBase::Identifier(Identifier {
                                name: "a".to_string(),
                                span: 18..19
                            }),
                            value: Expression::Integer(IntegerLit {
                                value: 2,
                                span: 22..23
                            })
                        })],
                    }),
                }],
            }
        )
    }

    #[test]
    fn plus_equal_assignment() {
        let source = include_str!("../tests/assignments/plus_equal_assignment.iv");
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
                        body: vec![Stmt::Assignment(Assignment {
                            member: MemberExpressionBase::Identifier(Identifier {
                                name: "a".to_string(),
                                span: 18..19
                            }),
                            value: Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Identifier(Identifier {
                                    name: "a".to_string(),
                                    span: 18..19
                                })),
                                right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 2,
                                        span: 23..24
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 3,
                                        span: 27..28
                                    })),
                                    operator: BinaryOperator::Add
                                })),
                                operator: BinaryOperator::Add
                            })
                        })],
                    }),
                }],
            }
        )
    }

    #[test]
    fn minus_equal_assignment() {
        let source = include_str!("../tests/assignments/minus_equal_assignment.iv");
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
                        body: vec![Stmt::Assignment(Assignment {
                            member: MemberExpressionBase::Identifier(Identifier {
                                name: "b".to_string(),
                                span: 18..19
                            }),
                            value: Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Identifier(Identifier {
                                    name: "b".to_string(),
                                    span: 18..19
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 23..24
                                })),
                                operator: BinaryOperator::Subtract
                            })
                        })],
                    }),
                }],
            }
        )
    }

    #[test]
    fn multiply_equal_assignment() {
        let source = include_str!("../tests/assignments/multiply_equal_assignment.iv");
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
                        body: vec![Stmt::Assignment(Assignment {
                            member: MemberExpressionBase::Identifier(Identifier {
                                name: "c".to_string(),
                                span: 18..19
                            }),
                            value: Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Identifier(Identifier {
                                    name: "c".to_string(),
                                    span: 18..19
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 4,
                                    span: 23..24
                                })),
                                operator: BinaryOperator::Multiply
                            })
                        })],
                    }),
                }],
            }
        )
    }

    #[test]
    fn divide_equal_assignment() {
        let source = include_str!("../tests/assignments/divide_equal_assignment.iv");
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
                        body: vec![Stmt::Assignment(Assignment {
                            member: MemberExpressionBase::Identifier(Identifier {
                                name: "d".to_string(),
                                span: 18..19
                            }),
                            value: Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Identifier(Identifier {
                                    name: "d".to_string(),
                                    span: 18..19
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 23..24
                                })),
                                operator: BinaryOperator::Divide
                            })
                        })],
                    }),
                }],
            }
        )
    }

    #[test]
    fn modulus_equal_assignment() {
        let source = include_str!("../tests/assignments/modulus_equal_assignment.iv");
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
                        body: vec![Stmt::Assignment(Assignment {
                            member: MemberExpressionBase::Identifier(Identifier {
                                name: "e".to_string(),
                                span: 18..19
                            }),
                            value: Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Identifier(Identifier {
                                    name: "e".to_string(),
                                    span: 18..19
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 3,
                                    span: 23..24
                                })),
                                operator: BinaryOperator::Modulus
                            })
                        })],
                    }),
                }],
            }
        )
    }
}
