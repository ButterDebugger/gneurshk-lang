use super::{
    BinaryExpression, BinaryOperator, BooleanLit, CastExpression, FloatLit, IntegerLit, StringLit,
    TokenStream, UnaryExpression, UnaryOperator,
};
use crate::{Expression, identifiers::parse_member_expression_base, types::parse_type};
use anyhow::{Result, anyhow};
use gneurshk_lexer::tokens::Token;

/// Parses a binary expression based on operator priority
pub fn parse_expression(tokens: &mut TokenStream) -> Result<Expression> {
    parse_logical_or(tokens)
}

/// Parses logical or (lowest priority)
fn parse_logical_or(tokens: &mut TokenStream) -> Result<Expression> {
    let mut left = parse_logical_and(tokens)?; // Parse the next priority level first

    // Continuously parse the given operators on this level until there are no more
    while let Some(operator) = match tokens.peek() {
        Some((Token::Or, _)) => Some(BinaryOperator::Or),
        _ => None, // Stop parsing this level
    } {
        tokens.next(); // Consume the operator token

        // With the next lowest priority, parse the right operand
        let right = parse_logical_and(tokens)?;
        left = Expression::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });
    }

    Ok(left)
}

/// Parses logical and
fn parse_logical_and(tokens: &mut TokenStream) -> Result<Expression> {
    let mut left = parse_comparison(tokens)?; // Parse the next priority level first

    // Continuously parse the given operators on this level until there are no more
    while let Some(operator) = match tokens.peek() {
        Some((Token::And, _)) => Some(BinaryOperator::And),
        _ => None, // Stop parsing this level
    } {
        tokens.next(); // Consume the operator token

        // With the next lowest priority, parse the right operand
        let right = parse_comparison(tokens)?;
        left = Expression::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });
    }

    Ok(left)
}

/// Parses comparison operators
fn parse_comparison(tokens: &mut TokenStream) -> Result<Expression> {
    let mut left = parse_addition_subtraction(tokens)?; // Parse the next priority level first

    // Continuously parse the given operators on this level until there are no more
    while let Some(operator) = match tokens.peek() {
        Some((Token::GreaterThan, _)) => Some(BinaryOperator::GreaterThan),
        Some((Token::GreaterThanEqual, _)) => Some(BinaryOperator::GreaterThanEqual),
        Some((Token::EqualEqual, _)) => Some(BinaryOperator::Equal),
        Some((Token::NotEqual, _)) => Some(BinaryOperator::NotEqual),
        Some((Token::LessThanEqual, _)) => Some(BinaryOperator::LessThanEqual),
        Some((Token::LessThan, _)) => Some(BinaryOperator::LessThan),
        _ => None, // Stop parsing this level
    } {
        tokens.next(); // Consume the operator token

        // With the next lowest priority, parse the right operand
        let right = parse_addition_subtraction(tokens)?;
        left = Expression::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });
    }

    Ok(left)
}

/// Parses addition and subtraction
fn parse_addition_subtraction(tokens: &mut TokenStream) -> Result<Expression> {
    let mut left = parse_multiplication_division(tokens)?; // Parse the next priority level first

    // Continuously parse the given operators on this level until there are no more
    while let Some(operator) = match tokens.peek() {
        Some((Token::Plus, _)) => Some(BinaryOperator::Add),
        Some((Token::Minus, _)) => Some(BinaryOperator::Subtract),
        _ => None, // Stop parsing this level
    } {
        tokens.next(); // Consume the operator token

        // With the next lowest priority, parse the right operand
        let right = parse_multiplication_division(tokens)?;
        left = Expression::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });
    }
    Ok(left)
}

/// Parses multiplication, division, and modulus
fn parse_multiplication_division(tokens: &mut TokenStream) -> Result<Expression> {
    let mut left = parse_cast(tokens)?; // Parse the next priority level first

    // Continuously parse the given operators on this level until there are no more
    while let Some(operator) = match tokens.peek() {
        Some((Token::Multiply, _)) => Some(BinaryOperator::Multiply),
        Some((Token::Divide, _)) => Some(BinaryOperator::Divide),
        Some((Token::Modulus, _)) => Some(BinaryOperator::Modulus),
        _ => None, // Stop parsing this level
    } {
        tokens.next(); // Consume the operator token

        // With the next lowest priority, parse the right operand
        let right = parse_term(tokens)?;
        left = Expression::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });
    }
    Ok(left)
}

/// Parses casting
fn parse_cast(tokens: &mut TokenStream) -> Result<Expression> {
    let mut value = parse_term(tokens)?; // Parse the next priority level first

    // Continuously parse casts on this level until there are no more
    while let Some((Token::As, _)) = tokens.peek() {
        tokens.next(); // Consume the 'as' token

        // Parse the target type
        let data_type = match parse_type(tokens)? {
            Some(data_type) => data_type,
            None => return Err(anyhow!("Expected a type after the 'as' keyword")),
        };

        value = Expression::Cast(CastExpression {
            value: Box::new(value),
            data_type,
        });
    }

    Ok(value)
}

/// Parses literals and parenthesized expressions (highest priority)
fn parse_term(tokens: &mut TokenStream) -> Result<Expression> {
    match tokens.peek() {
        Some((Token::OpenParen, _)) => {
            tokens.next(); // Consume the '(' token

            // Recursively parse the inner expression
            let expression = parse_expression(tokens)?;

            // Consume the ')' token and return the expression
            match tokens.next() {
                Some((Token::CloseParen, _)) => Ok(expression),
                _ => Err(anyhow!("Expected a closing parenthesis")),
            }
        }
        Some((Token::Minus, _)) => {
            tokens.next(); // Consume the '-' token

            // Parse the operand
            let operand = parse_term(tokens)?;

            Ok(Expression::UnaryExpression(UnaryExpression {
                value: Box::new(operand),
                operator: UnaryOperator::Negative,
            }))
        }
        Some((Token::Not, _)) => {
            tokens.next(); // Consume the 'not' token

            // Parse the operand
            let operand = parse_term(tokens)?;

            Ok(Expression::UnaryExpression(UnaryExpression {
                value: Box::new(operand),
                operator: UnaryOperator::Not,
            }))
        }
        Some((Token::Integer(_), _))
        | Some((Token::Float(_), _))
        | Some((Token::Boolean(_), _))
        | Some((Token::String(_), _)) => parse_literal(tokens),
        Some((Token::Word(_), _)) => Ok(parse_member_expression_base(tokens)?.into()),
        Some(_) => Err(anyhow!("Unexpected token in expression")),
        None => Err(anyhow!("Unexpected end of tokens in expression")),
    }
}

fn parse_literal(tokens: &mut TokenStream) -> Result<Expression> {
    match tokens.next() {
        Some((Token::Integer(value), span)) => Ok(Expression::Integer(IntegerLit { value, span })),
        Some((Token::Float(value), span)) => Ok(Expression::Float(FloatLit { value, span })),
        Some((Token::Boolean(value), span)) => Ok(Expression::Boolean(BooleanLit { value, span })),
        Some((Token::String(value), span)) => Ok(Expression::String(StringLit { value, span })),
        _ => Err(anyhow!("Expected literal")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BinaryOperator, Block, CastExpression, DataType, Expression, FunctionDeclaration,
        Identifier, IntegerLit, MemberAccess, MemberExpressionBase, MemberExpressionMember,
        Program, Stmt, UnaryOperator, VariableDeclaration, parse,
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
    fn repeated_numbers() {
        let source = include_str!("../tests/expressions/repeated_numbers.iv");
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
                            Stmt::Integer(IntegerLit {
                                value: 1,
                                span: 18..19
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 2,
                                span: 20..21
                            }),
                            Stmt::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 3,
                                        span: 22..23
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 4,
                                        span: 26..27
                                    })),
                                    operator: BinaryOperator::Add,
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 31..32
                                })),
                                operator: BinaryOperator::Equal,
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 6,
                                span: 33..34
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 7,
                                span: 35..36
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 8,
                                span: 37..38
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 9,
                                span: 39..40
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 10,
                                span: 41..43
                            }),
                        ],
                    }),
                }],
            }
        );
    }

    #[test]
    fn single_integer() {
        let source = include_str!("../tests/expressions/single_integer.iv");
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
                        body: vec![Stmt::Integer(IntegerLit {
                            value: 42,
                            span: 18..20
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn basic_expression() {
        let source = include_str!("../tests/expressions/basic_expression.iv");
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
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::Integer(IntegerLit {
                                value: 1,
                                span: 18..19
                            })),
                            right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 7,
                                        span: 22..23
                                    })),
                                    right: Box::new(Expression::BinaryExpression(
                                        BinaryExpression {
                                            left: Box::new(Expression::Integer(IntegerLit {
                                                value: 3,
                                                span: 27..28
                                            })),
                                            right: Box::new(Expression::Integer(IntegerLit {
                                                value: 4,
                                                span: 31..32
                                            })),
                                            operator: BinaryOperator::Subtract,
                                        }
                                    )),
                                    operator: BinaryOperator::Multiply,
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 36..37
                                })),
                                operator: BinaryOperator::Divide,
                            })),
                            operator: BinaryOperator::Add
                        })],
                    }),
                }],
            }
        )
    }

    #[test]
    fn and_or_logical_expression() {
        let source = include_str!("../tests/expressions/and_or_logical_expression.iv");
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
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 1,
                                        span: 18..19
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 2,
                                        span: 22..23
                                    })),
                                    operator: BinaryOperator::LessThan,
                                })),
                                right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 3,
                                        span: 27..28
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 4,
                                        span: 31..32
                                    })),
                                    operator: BinaryOperator::GreaterThan,
                                })),
                                operator: BinaryOperator::And,
                            })),
                            right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 36..37
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 6,
                                    span: 41..42
                                })),
                                operator: BinaryOperator::Equal,
                            })),
                            operator: BinaryOperator::Or,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn or_and_logical_expression() {
        let source = include_str!("../tests/expressions/or_and_logical_expression.iv");
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
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 18..19
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 22..23
                                })),
                                operator: BinaryOperator::LessThan,
                            })),
                            right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 3,
                                        span: 27..28
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 4,
                                        span: 31..32
                                    })),
                                    operator: BinaryOperator::GreaterThan,
                                })),
                                right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 5,
                                        span: 36..37
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 6,
                                        span: 41..42
                                    })),
                                    operator: BinaryOperator::Equal,
                                })),
                                operator: BinaryOperator::And,
                            })),
                            operator: BinaryOperator::Or,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn and_or_and_logical_expression() {
        let source = include_str!("../tests/expressions/and_or_and_logical_expression.iv");
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
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 1,
                                        span: 18..19
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 2,
                                        span: 22..23
                                    })),
                                    operator: BinaryOperator::LessThan,
                                })),
                                right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 3,
                                        span: 27..28
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 4,
                                        span: 31..32
                                    })),
                                    operator: BinaryOperator::GreaterThan,
                                })),
                                operator: BinaryOperator::And,
                            })),
                            right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 5,
                                        span: 36..37
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 6,
                                        span: 41..42
                                    })),
                                    operator: BinaryOperator::Equal,
                                })),
                                right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 7,
                                        span: 46..47
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 8,
                                        span: 51..52
                                    })),
                                    operator: BinaryOperator::NotEqual,
                                })),
                                operator: BinaryOperator::And,
                            })),
                            operator: BinaryOperator::Or,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn or_and_or_logical_expression() {
        let source = include_str!("../tests/expressions/or_and_or_logical_expression.iv");
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
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 1,
                                        span: 18..19
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 2,
                                        span: 22..23
                                    })),
                                    operator: BinaryOperator::LessThan,
                                })),
                                right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::BinaryExpression(
                                        BinaryExpression {
                                            left: Box::new(Expression::Integer(IntegerLit {
                                                value: 3,
                                                span: 27..28
                                            })),
                                            right: Box::new(Expression::Integer(IntegerLit {
                                                value: 4,
                                                span: 31..32
                                            })),
                                            operator: BinaryOperator::GreaterThan,
                                        }
                                    )),
                                    right: Box::new(Expression::BinaryExpression(
                                        BinaryExpression {
                                            left: Box::new(Expression::Integer(IntegerLit {
                                                value: 5,
                                                span: 36..37
                                            })),
                                            right: Box::new(Expression::Integer(IntegerLit {
                                                value: 6,
                                                span: 41..42
                                            })),
                                            operator: BinaryOperator::Equal,
                                        }
                                    )),
                                    operator: BinaryOperator::And,
                                })),
                                operator: BinaryOperator::Or,
                            })),
                            right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 7,
                                    span: 46..47
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 8,
                                    span: 51..52
                                })),
                                operator: BinaryOperator::NotEqual,
                            })),
                            operator: BinaryOperator::Or,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn negative_number() {
        let source = include_str!("../tests/expressions/negative_number.iv");
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
                        body: vec![Stmt::UnaryExpression(UnaryExpression {
                            value: Box::new(Expression::Integer(IntegerLit {
                                value: 1,
                                span: 19..20
                            })),
                            operator: UnaryOperator::Negative,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn negative_expression() {
        let source = include_str!("../tests/expressions/negative_expression.iv");
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
                        body: vec![Stmt::UnaryExpression(UnaryExpression {
                            value: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 20..21
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 24..25
                                })),
                                operator: BinaryOperator::Add,
                            })),
                            operator: UnaryOperator::Negative,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn not_expression() {
        let source = include_str!("../tests/expressions/not_expression.iv");
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
                        body: vec![Stmt::UnaryExpression(UnaryExpression {
                            value: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 23..24
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 28..29
                                })),
                                operator: BinaryOperator::Equal,
                            })),
                            operator: UnaryOperator::Not,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn single_float() {
        let source = include_str!("../tests/expressions/single_float.iv");
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
                        body: vec![Stmt::Float(FloatLit {
                            value: 1.0,
                            span: 18..21
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn float_and_integer_expression() {
        let source = include_str!("../tests/expressions/float_and_integer_expression.iv");
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
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::Integer(IntegerLit {
                                value: 1,
                                span: 18..19
                            })),
                            right: Box::new(Expression::Float(FloatLit {
                                value: 2.0,
                                span: 22..25
                            })),
                            operator: BinaryOperator::Add,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn float_and_float_expression() {
        let source = include_str!("../tests/expressions/float_and_float_expression.iv");
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
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::Float(FloatLit {
                                value: 1.0,
                                span: 18..21
                            })),
                            right: Box::new(Expression::Float(FloatLit {
                                value: 2.0,
                                span: 24..27
                            })),
                            operator: BinaryOperator::Add,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn single_string() {
        let source = include_str!("../tests/expressions/single_string.iv");
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
                        body: vec![Stmt::String(StringLit {
                            value: "i love you".to_string(),
                            span: 18..30,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn cast_basic() {
        let source = include_str!("../tests/expressions/cast_basic.iv");
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
                        body: vec![Stmt::Cast(CastExpression {
                            value: Box::new(Expression::Integer(IntegerLit {
                                value: 5,
                                span: 18..19
                            })),
                            data_type: DataType::Float32,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn cast_precedence_addition() {
        let source = include_str!("../tests/expressions/cast_precedence_addition.iv");
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
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::Integer(IntegerLit {
                                value: 1,
                                span: 18..19
                            })),
                            right: Box::new(Expression::Cast(CastExpression {
                                value: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 22..23
                                })),
                                data_type: DataType::Float32,
                            })),
                            operator: BinaryOperator::Add,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn cast_parenthesized() {
        let source = include_str!("../tests/expressions/cast_parenthesized.iv");
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
                        body: vec![Stmt::Cast(CastExpression {
                            value: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 19..20
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 23..24
                                })),
                                operator: BinaryOperator::Add,
                            })),
                            data_type: DataType::Float32,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn cast_with_multiplication() {
        let source = include_str!("../tests/expressions/cast_with_multiplication.iv");
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
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::Cast(CastExpression {
                                value: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 18..19
                                })),
                                data_type: DataType::Float32,
                            })),
                            right: Box::new(Expression::Integer(IntegerLit {
                                value: 3,
                                span: 33..34
                            })),
                            operator: BinaryOperator::Multiply,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn cast_in_variable() {
        let source = include_str!("../tests/expressions/cast_in_variable.iv");
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
                        body: vec![Stmt::VariableDeclaration(VariableDeclaration::Mutable {
                            name: "x".to_string(),
                            data_type: None,
                            value: Some(Expression::Cast(CastExpression {
                                value: Box::new(Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 26..27
                                })),
                                data_type: DataType::Float32,
                            })),
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn cast_custom_type() {
        let source = include_str!("../tests/expressions/cast_custom_type.iv");
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
                        body: vec![Stmt::Cast(CastExpression {
                            value: Box::new(Expression::Integer(IntegerLit {
                                value: 5,
                                span: 18..19
                            })),
                            data_type: DataType::Custom("CustomType".to_string()),
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn cast_identifier() {
        let source = include_str!("../tests/expressions/cast_identifier.iv");
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
                        body: vec![Stmt::Cast(CastExpression {
                            value: Box::new(Expression::Identifier(Identifier {
                                name: "foo".to_string(),
                                span: 18..21
                            })),
                            data_type: DataType::Int32,
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    #[should_panic]
    fn cast_missing_type() {
        let source = include_str!("../tests/expressions/cast_missing_type.iv");
        let _ = lex_then_parse(source);
    }

    #[test]
    fn cast_member_access() {
        let source = include_str!("../tests/expressions/cast_member_access.iv");
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
                        body: vec![Stmt::Cast(CastExpression {
                            value: Box::new(Expression::MemberAccess(MemberAccess {
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
                            data_type: DataType::Int32,
                        })],
                    }),
                }],
            }
        );
    }
}
