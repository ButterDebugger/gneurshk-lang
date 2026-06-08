use super::{
    BinaryExpression, BinaryOperator, BooleanLit, FloatLit, IntegerLit, StringLit, TokenStream,
    UnaryExpression, UnaryOperator,
};
use crate::{Expression, identifiers::parse_member_expression_base};
use anyhow::{Result, anyhow};
use gneurshk_lexer::tokens::Token;

/// Parses a binary expression based on operator priority
pub fn parse_expression(tokens: &mut TokenStream) -> Result<Expression> {
    parse_logical_or(tokens)
}

/// Parses logical or (lowest priority)
fn parse_logical_or(tokens: &mut TokenStream) -> Result<Expression> {
    let mut left = parse_logical_and(tokens)?; // Parse the next priority level first

    // Continuously parse the given operators on this priority level until there are no more
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

    // Continuously parse the given operators on this priority level until there are no more
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

    // Continuously parse the given operators on this priority level until there are no more
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

    // Continuously parse the given operators on this priority level until there are no more
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
    let mut left = parse_term(tokens)?; // Parse the next priority level first

    // Continuously parse the given operators on this priority level until there are no more
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
        BinaryOperator, Block, FunctionDeclaration, Program, Stmt, UnaryOperator, parse,
        types::DataType,
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
        let stmt = lex_then_parse(
            r#"
func main() {
    1 2 3 + 4 == 5 6 7 8 9 10
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![
                            Stmt::Integer(IntegerLit {
                                value: 1,
                                span: 19..20
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 2,
                                span: 21..22
                            }),
                            Stmt::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 3,
                                        span: 23..24
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 4,
                                        span: 27..28
                                    })),
                                    operator: BinaryOperator::Add,
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 32..33
                                })),
                                operator: BinaryOperator::Equal,
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 6,
                                span: 34..35
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 7,
                                span: 36..37
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 8,
                                span: 38..39
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 9,
                                span: 40..41
                            }),
                            Stmt::Integer(IntegerLit {
                                value: 10,
                                span: 42..44
                            }),
                        ],
                    }),
                }],
            }
        );
    }

    #[test]
    fn single_integer() {
        let stmt = lex_then_parse(
            r#"
func main() {
    42
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::Integer(IntegerLit {
                            value: 42,
                            span: 19..21
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn basic_expression() {
        let stmt = lex_then_parse(
            r#"
func main() {
    1 + 7 * (3 - 4) / 5
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::Integer(IntegerLit {
                                value: 1,
                                span: 19..20
                            })),
                            right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 7,
                                        span: 23..24
                                    })),
                                    right: Box::new(Expression::BinaryExpression(
                                        BinaryExpression {
                                            left: Box::new(Expression::Integer(IntegerLit {
                                                value: 3,
                                                span: 28..29
                                            })),
                                            right: Box::new(Expression::Integer(IntegerLit {
                                                value: 4,
                                                span: 32..33
                                            })),
                                            operator: BinaryOperator::Subtract,
                                        }
                                    )),
                                    operator: BinaryOperator::Multiply,
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 37..38
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
        let stmt = lex_then_parse(
            r#"
func main() {
    1 < 2 && 3 > 4 || 5 == 6
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 1,
                                        span: 19..20
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 2,
                                        span: 23..24
                                    })),
                                    operator: BinaryOperator::LessThan,
                                })),
                                right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 3,
                                        span: 28..29
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 4,
                                        span: 32..33
                                    })),
                                    operator: BinaryOperator::GreaterThan,
                                })),
                                operator: BinaryOperator::And,
                            })),
                            right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 37..38
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 6,
                                    span: 42..43
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
        let stmt = lex_then_parse(
            r#"
func main() {
    1 < 2 || 3 > 4 && 5 == 6
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 19..20
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 23..24
                                })),
                                operator: BinaryOperator::LessThan,
                            })),
                            right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 3,
                                        span: 28..29
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 4,
                                        span: 32..33
                                    })),
                                    operator: BinaryOperator::GreaterThan,
                                })),
                                right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 5,
                                        span: 37..38
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 6,
                                        span: 42..43
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
        let stmt = lex_then_parse(
            r#"
func main() {
    1 < 2 && 3 > 4 || 5 == 6 && 7 != 8
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 1,
                                        span: 19..20
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 2,
                                        span: 23..24
                                    })),
                                    operator: BinaryOperator::LessThan,
                                })),
                                right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 3,
                                        span: 28..29
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 4,
                                        span: 32..33
                                    })),
                                    operator: BinaryOperator::GreaterThan,
                                })),
                                operator: BinaryOperator::And,
                            })),
                            right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 5,
                                        span: 37..38
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 6,
                                        span: 42..43
                                    })),
                                    operator: BinaryOperator::Equal,
                                })),
                                right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 7,
                                        span: 47..48
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 8,
                                        span: 52..53
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
        let stmt = lex_then_parse(
            r#"
func main() {
    1 < 2 || 3 > 4 && 5 == 6 || 7 != 8
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::Integer(IntegerLit {
                                        value: 1,
                                        span: 19..20
                                    })),
                                    right: Box::new(Expression::Integer(IntegerLit {
                                        value: 2,
                                        span: 23..24
                                    })),
                                    operator: BinaryOperator::LessThan,
                                })),
                                right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                    left: Box::new(Expression::BinaryExpression(
                                        BinaryExpression {
                                            left: Box::new(Expression::Integer(IntegerLit {
                                                value: 3,
                                                span: 28..29
                                            })),
                                            right: Box::new(Expression::Integer(IntegerLit {
                                                value: 4,
                                                span: 32..33
                                            })),
                                            operator: BinaryOperator::GreaterThan,
                                        }
                                    )),
                                    right: Box::new(Expression::BinaryExpression(
                                        BinaryExpression {
                                            left: Box::new(Expression::Integer(IntegerLit {
                                                value: 5,
                                                span: 37..38
                                            })),
                                            right: Box::new(Expression::Integer(IntegerLit {
                                                value: 6,
                                                span: 42..43
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
                                    span: 47..48
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 8,
                                    span: 52..53
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
        let stmt = lex_then_parse(
            r#"
func main() {
    -1
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::UnaryExpression(UnaryExpression {
                            value: Box::new(Expression::Integer(IntegerLit {
                                value: 1,
                                span: 20..21
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
        let stmt = lex_then_parse(
            r#"
func main() {
    -(1 + 2)
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::UnaryExpression(UnaryExpression {
                            value: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 21..22
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 25..26
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
        let stmt = lex_then_parse(
            r#"
func main() {
    not (1 == 2)
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::UnaryExpression(UnaryExpression {
                            value: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 24..25
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 29..30
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
        let stmt = lex_then_parse(
            r#"
func main() {
    1.0
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::Float(FloatLit {
                            value: 1.0,
                            span: 19..22
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn float_and_integer_expression() {
        let stmt = lex_then_parse(
            r#"
func main() {
    1 + 2.0
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::Integer(IntegerLit {
                                value: 1,
                                span: 19..20
                            })),
                            right: Box::new(Expression::Float(FloatLit {
                                value: 2.0,
                                span: 23..26
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
        let stmt = lex_then_parse(
            r#"
func main() {
    1.0 + 2.0
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::Float(FloatLit {
                                value: 1.0,
                                span: 19..22
                            })),
                            right: Box::new(Expression::Float(FloatLit {
                                value: 2.0,
                                span: 25..28
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
        let stmt = lex_then_parse(
            r#"
func main() {
    "i love you"
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
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::String(StringLit {
                            value: "i love you".to_string(),
                            span: 19..31,
                        })],
                    }),
                }],
            }
        );
    }
}
