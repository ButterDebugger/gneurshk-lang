use crate::strings::parse_string;

use super::{BinaryOperator, StatementResult, Stmt, TokenStream, UnaryOperator};
use gneurshk_lexer::tokens::Token;

/// Parses a binary expression based on operator priority
pub fn parse_expression(tokens: &mut TokenStream) -> StatementResult {
    parse_logical_or(tokens)
}

/// Parses logical or (lowest priority)
fn parse_logical_or(tokens: &mut TokenStream) -> StatementResult {
    let mut left = parse_logical_and(tokens)?; // Parse the next priority level first

    // Continuously parse the given operators on this priority level until there are no more
    while let Some(operator) = match tokens.peek() {
        Some((Token::Or, _)) => Some(BinaryOperator::Or),
        _ => None, // Stop parsing this level
    } {
        tokens.next(); // Consume the operator token

        // With the next lowest priority, parse the right operand
        let right = parse_logical_and(tokens)?;
        left = Stmt::BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

/// Parses logical and
fn parse_logical_and(tokens: &mut TokenStream) -> StatementResult {
    let mut left = parse_comparison(tokens)?; // Parse the next priority level first

    // Continuously parse the given operators on this priority level until there are no more
    while let Some(operator) = match tokens.peek() {
        Some((Token::And, _)) => Some(BinaryOperator::And),
        _ => None, // Stop parsing this level
    } {
        tokens.next(); // Consume the operator token

        // With the next lowest priority, parse the right operand
        let right = parse_comparison(tokens)?;
        left = Stmt::BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

/// Parses comparison operators
fn parse_comparison(tokens: &mut TokenStream) -> StatementResult {
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
        left = Stmt::BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

/// Parses addition and subtraction
fn parse_addition_subtraction(tokens: &mut TokenStream) -> StatementResult {
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
        left = Stmt::BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }
    Ok(left)
}

/// Parses multiplication, division, and modulus
fn parse_multiplication_division(tokens: &mut TokenStream) -> StatementResult {
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
        left = Stmt::BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }
    Ok(left)
}

/// Parses literals and parenthesized expressions (highest priority)
fn parse_term(tokens: &mut TokenStream) -> StatementResult {
    match tokens.peek() {
        Some((Token::OpenParen, _)) => {
            tokens.next(); // Consume the '(' token

            // Recursively parse the inner expression
            let expression = parse_expression(tokens)?;

            // Consume the ')' token and return the expression
            match tokens.next() {
                Some((Token::CloseParen, _)) => Ok(expression),
                _ => Err("Expected a closing parenthesis"),
            }
        }
        Some((Token::Minus, _)) => {
            tokens.next(); // Consume the '-' token

            // Parse the operand
            let operand = parse_term(tokens)?;

            Ok(Stmt::UnaryExpression {
                value: Box::new(operand),
                operator: UnaryOperator::Negative,
            })
        }
        Some((Token::Not, _)) => {
            tokens.next(); // Consume the 'not' token

            // Parse the operand
            let operand = parse_term(tokens)?;

            Ok(Stmt::UnaryExpression {
                value: Box::new(operand),
                operator: UnaryOperator::Not,
            })
        }
        Some((Token::Integer(_), _)) | Some((Token::Float(_), _)) => parse_literal(tokens),
        Some((Token::Word(_), _)) => parse_identifier_or_function_call(tokens),
        Some((Token::String(_), _)) => parse_string(tokens),
        Some(_) => Err("Unexpected token in expression"),
        None => Err("Unexpected end of tokens in expression"),
    }
}

fn parse_literal(tokens: &mut TokenStream) -> StatementResult {
    match tokens.next() {
        Some((Token::Integer(value), _)) => Ok(Stmt::Integer { value }),
        Some((Token::Float(value), _)) => Ok(Stmt::Float { value }),
        _ => Err("Expected literal"),
    }
}

fn parse_identifier_or_function_call(tokens: &mut TokenStream) -> StatementResult {
    match tokens.next() {
        Some((Token::Word(name), _)) => {
            if let Some((Token::OpenParen, _)) = tokens.peek() {
                tokens.next(); // Consume the opening parenthesis

                // Parse the arguments
                let mut args = Vec::new();

                // Handle empty argument list
                if let Some((Token::CloseParen, _)) = tokens.peek() {
                    tokens.next(); // Consume the closing parenthesis
                    return Ok(Stmt::FunctionCall { name, args });
                }

                // Loop while there are still arguments to parse
                loop {
                    // Parse the argument as an expression
                    let arg = parse_expression(tokens)?;
                    args.push(arg);

                    // Check for comma or closing parenthesis
                    match tokens.peek() {
                        Some((Token::Comma, _)) => {
                            tokens.next(); // Consume the comma
                        }
                        Some((Token::CloseParen, _)) => {
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

                Ok(Stmt::FunctionCall { name, args })
            } else {
                Ok(Stmt::Identifier { name })
            }
        }
        _ => Err("Expected identifier"),
    }
}

#[cfg(test)]
mod tests {
    use crate::Stmt::{self, BinaryExpression, Identifier, Integer};
    use crate::{BinaryOperator, Program, UnaryOperator, parse};
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
    fn repeated_identifiers() {
        let stmt =
            lex_then_parse("chicken chicken chicken chicken chicken chicken chicken chicken").body;

        assert_eq!(
            stmt,
            vec![
                Identifier {
                    name: "chicken".to_string()
                },
                Identifier {
                    name: "chicken".to_string()
                },
                Identifier {
                    name: "chicken".to_string()
                },
                Identifier {
                    name: "chicken".to_string()
                },
                Identifier {
                    name: "chicken".to_string()
                },
                Identifier {
                    name: "chicken".to_string()
                },
                Identifier {
                    name: "chicken".to_string()
                },
                Identifier {
                    name: "chicken".to_string()
                },
            ]
        );
    }

    #[test]
    fn repeated_numbers() {
        let stmt = lex_then_parse("1 2 3 + 4 == 5 6 7 8 9 10").body;

        assert_eq!(
            stmt,
            vec![
                Integer { value: 1 },
                Integer { value: 2 },
                BinaryExpression {
                    left: Box::new(BinaryExpression {
                        left: Box::new(Integer { value: 3 }),
                        right: Box::new(Integer { value: 4 }),
                        operator: BinaryOperator::Add,
                    }),
                    right: Box::new(Integer { value: 5 }),
                    operator: BinaryOperator::Equal,
                },
                Integer { value: 6 },
                Integer { value: 7 },
                Integer { value: 8 },
                Integer { value: 9 },
                Integer { value: 10 },
            ]
        );
    }

    #[test]
    fn single_identifier() {
        let stmt = lex_then_parse("chicken").body;

        assert_eq!(
            stmt,
            vec![Identifier {
                name: "chicken".to_string()
            }]
        );
    }

    #[test]
    fn single_integer() {
        let stmt = lex_then_parse("42").body;

        assert_eq!(stmt, vec![Stmt::Integer { value: 42 }]);
    }

    #[test]
    fn basic_expression() {
        let stmt = lex_then_parse("1 + 7 * (3 - 4) / 5").body;

        assert_eq!(
            stmt,
            vec![BinaryExpression {
                left: Box::new(Integer { value: 1 }),
                right: Box::new(BinaryExpression {
                    left: Box::new(BinaryExpression {
                        left: Box::new(Integer { value: 7 }),
                        right: Box::new(BinaryExpression {
                            left: Box::new(Integer { value: 3 }),
                            right: Box::new(Integer { value: 4 }),
                            operator: BinaryOperator::Subtract,
                        }),
                        operator: BinaryOperator::Multiply,
                    }),
                    right: Box::new(Integer { value: 5 }),
                    operator: BinaryOperator::Divide,
                }),
                operator: BinaryOperator::Add
            }]
        )
    }

    #[test]
    fn and_or_logical_expression() {
        let stmt = lex_then_parse("1 < 2 && 3 > 4 || 5 == 6").body;

        assert_eq!(
            stmt,
            vec![BinaryExpression {
                left: Box::new(BinaryExpression {
                    left: Box::new(BinaryExpression {
                        left: Box::new(Integer { value: 1 }),
                        right: Box::new(Integer { value: 2 }),
                        operator: BinaryOperator::LessThan,
                    }),
                    right: Box::new(BinaryExpression {
                        left: Box::new(Integer { value: 3 }),
                        right: Box::new(Integer { value: 4 }),
                        operator: BinaryOperator::GreaterThan,
                    }),
                    operator: BinaryOperator::And,
                }),
                right: Box::new(BinaryExpression {
                    left: Box::new(Integer { value: 5 }),
                    right: Box::new(Integer { value: 6 }),
                    operator: BinaryOperator::Equal,
                }),
                operator: BinaryOperator::Or,
            }]
        );
    }

    #[test]
    fn or_and_logical_expression() {
        let stmt = lex_then_parse("1 < 2 || 3 > 4 && 5 == 6").body;

        assert_eq!(
            stmt,
            vec![BinaryExpression {
                left: Box::new(BinaryExpression {
                    left: Box::new(Integer { value: 1 }),
                    right: Box::new(Integer { value: 2 }),
                    operator: BinaryOperator::LessThan,
                }),
                right: Box::new(BinaryExpression {
                    left: Box::new(BinaryExpression {
                        left: Box::new(Integer { value: 3 }),
                        right: Box::new(Integer { value: 4 }),
                        operator: BinaryOperator::GreaterThan,
                    }),
                    right: Box::new(BinaryExpression {
                        left: Box::new(Integer { value: 5 }),
                        right: Box::new(Integer { value: 6 }),
                        operator: BinaryOperator::Equal,
                    }),
                    operator: BinaryOperator::And,
                }),
                operator: BinaryOperator::Or,
            }]
        );
    }

    #[test]
    fn and_or_and_logical_expression() {
        let stmt = lex_then_parse("1 < 2 && 3 > 4 || 5 == 6 && 7 != 8").body;

        assert_eq!(
            stmt,
            vec![BinaryExpression {
                left: Box::new(BinaryExpression {
                    left: Box::new(BinaryExpression {
                        left: Box::new(Integer { value: 1 }),
                        right: Box::new(Integer { value: 2 }),
                        operator: BinaryOperator::LessThan,
                    }),
                    right: Box::new(BinaryExpression {
                        left: Box::new(Integer { value: 3 }),
                        right: Box::new(Integer { value: 4 }),
                        operator: BinaryOperator::GreaterThan,
                    }),
                    operator: BinaryOperator::And,
                }),
                right: Box::new(BinaryExpression {
                    left: Box::new(BinaryExpression {
                        left: Box::new(Integer { value: 5 }),
                        right: Box::new(Integer { value: 6 }),
                        operator: BinaryOperator::Equal,
                    }),
                    right: Box::new(BinaryExpression {
                        left: Box::new(Integer { value: 7 }),
                        right: Box::new(Integer { value: 8 }),
                        operator: BinaryOperator::NotEqual,
                    }),
                    operator: BinaryOperator::And,
                }),
                operator: BinaryOperator::Or,
            }]
        );
    }

    #[test]
    fn or_and_or_logical_expression() {
        let stmt = lex_then_parse("1 < 2 || 3 > 4 && 5 == 6 || 7 != 8").body;

        assert_eq!(
            stmt,
            vec![BinaryExpression {
                left: Box::new(BinaryExpression {
                    left: Box::new(BinaryExpression {
                        left: Box::new(Integer { value: 1 }),
                        right: Box::new(Integer { value: 2 }),
                        operator: BinaryOperator::LessThan,
                    }),
                    right: Box::new(BinaryExpression {
                        left: Box::new(BinaryExpression {
                            left: Box::new(Integer { value: 3 }),
                            right: Box::new(Integer { value: 4 }),
                            operator: BinaryOperator::GreaterThan,
                        }),
                        right: Box::new(BinaryExpression {
                            left: Box::new(Integer { value: 5 }),
                            right: Box::new(Integer { value: 6 }),
                            operator: BinaryOperator::Equal,
                        }),
                        operator: BinaryOperator::And,
                    }),
                    operator: BinaryOperator::Or,
                }),
                right: Box::new(BinaryExpression {
                    left: Box::new(Integer { value: 7 }),
                    right: Box::new(Integer { value: 8 }),
                    operator: BinaryOperator::NotEqual,
                }),
                operator: BinaryOperator::Or,
            }]
        );
    }

    #[test]
    fn function_call_no_args() {
        let stmt = lex_then_parse("foo()").body;

        assert_eq!(
            stmt,
            vec![Stmt::FunctionCall {
                name: "foo".to_string(),
                args: vec![],
            }]
        );
    }

    #[test]
    fn function_call_single_arg() {
        let stmt = lex_then_parse("bar(42)").body;

        assert_eq!(
            stmt,
            vec![Stmt::FunctionCall {
                name: "bar".to_string(),
                args: vec![Stmt::Integer { value: 42 }],
            }]
        );
    }

    #[test]
    fn function_call_multiple_args() {
        let stmt = lex_then_parse("baz(1, 2, 3)").body;

        assert_eq!(
            stmt,
            vec![Stmt::FunctionCall {
                name: "baz".to_string(),
                args: vec![
                    Stmt::Integer { value: 1 },
                    Stmt::Integer { value: 2 },
                    Stmt::Integer { value: 3 },
                ],
            }]
        );
    }

    #[test]
    fn function_call_with_expression_args() {
        let stmt = lex_then_parse("calculate(1 + (2 + 5), 3 * 4)").body;

        assert_eq!(
            stmt,
            vec![Stmt::FunctionCall {
                name: "calculate".to_string(),
                args: vec![
                    Stmt::BinaryExpression {
                        left: Box::new(Stmt::Integer { value: 1 }),
                        right: Box::new(Stmt::BinaryExpression {
                            left: Box::new(Stmt::Integer { value: 2 }),
                            right: Box::new(Stmt::Integer { value: 5 }),
                            operator: BinaryOperator::Add,
                        }),
                        operator: BinaryOperator::Add,
                    },
                    Stmt::BinaryExpression {
                        left: Box::new(Stmt::Integer { value: 3 }),
                        right: Box::new(Stmt::Integer { value: 4 }),
                        operator: BinaryOperator::Multiply,
                    },
                ],
            }]
        );
    }

    #[test]
    fn negative_number() {
        let stmt = lex_then_parse("-1").body;

        assert_eq!(
            stmt,
            vec![Stmt::UnaryExpression {
                value: Box::new(Stmt::Integer { value: 1 }),
                operator: UnaryOperator::Negative,
            }]
        );
    }

    #[test]
    fn negative_expression() {
        let stmt = lex_then_parse("-(1 + 2)").body;

        assert_eq!(
            stmt,
            vec![Stmt::UnaryExpression {
                value: Box::new(Stmt::BinaryExpression {
                    left: Box::new(Stmt::Integer { value: 1 }),
                    right: Box::new(Stmt::Integer { value: 2 }),
                    operator: BinaryOperator::Add,
                }),
                operator: UnaryOperator::Negative,
            }]
        );
    }

    #[test]
    fn not_expression() {
        let stmt = lex_then_parse("not (1 == 2)").body;

        assert_eq!(
            stmt,
            vec![Stmt::UnaryExpression {
                value: Box::new(Stmt::BinaryExpression {
                    left: Box::new(Stmt::Integer { value: 1 }),
                    right: Box::new(Stmt::Integer { value: 2 }),
                    operator: BinaryOperator::Equal,
                }),
                operator: UnaryOperator::Not,
            }]
        );
    }

    #[test]
    fn single_float() {
        let stmt = lex_then_parse("1.0").body;

        assert_eq!(stmt, vec![Stmt::Float { value: 1.0 }]);
    }

    #[test]
    fn float_and_integer_expression() {
        let stmt = lex_then_parse("1 + 2.0").body;

        assert_eq!(
            stmt,
            vec![Stmt::BinaryExpression {
                left: Box::new(Stmt::Integer { value: 1 }),
                right: Box::new(Stmt::Float { value: 2.0 }),
                operator: BinaryOperator::Add,
            }]
        );
    }

    #[test]
    fn float_and_float_expression() {
        let stmt = lex_then_parse("1.0 + 2.0").body;

        assert_eq!(
            stmt,
            vec![Stmt::BinaryExpression {
                left: Box::new(Stmt::Float { value: 1.0 }),
                right: Box::new(Stmt::Float { value: 2.0 }),
                operator: BinaryOperator::Add,
            }]
        );
    }
}
