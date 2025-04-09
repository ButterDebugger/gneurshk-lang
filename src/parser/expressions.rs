use super::{Operator, StatementResult, Stmt, TokenStream};
use crate::lexer::tokens::Token;

/// Parses a binary expression based on operator priority
pub fn parse_expression(tokens: &mut TokenStream) -> StatementResult {
    parse_comparison(tokens)
}

/// Parses comparison operators (lowest priority)
fn parse_comparison(tokens: &mut TokenStream) -> StatementResult {
    let mut left = parse_addition_subtraction(tokens)?; // Parse the next priority level first

    // Continuously parse the given operators on this priority level until there are no more
    while let Some(operator) = match tokens.peek() {
        Some((Token::GreaterThan, _)) => Some(Operator::GreaterThan),
        Some((Token::GreaterThanEqual, _)) => Some(Operator::GreaterThanEqual),
        Some((Token::Equal, _)) => Some(Operator::Equal), // Assuming Token::Equal is for '==' comparison
        Some((Token::LessThanEqual, _)) => Some(Operator::LessThanEqual),
        Some((Token::LessThan, _)) => Some(Operator::LessThan),
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
        Some((Token::Plus, _)) => Some(Operator::Add),
        Some((Token::Minus, _)) => Some(Operator::Subtract),
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
        Some((Token::Multiply, _)) => Some(Operator::Multiply),
        Some((Token::Divide, _)) => Some(Operator::Divide),
        Some((Token::Modulus, _)) => Some(Operator::Modulus),
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
        Some((Token::Integer(_), _)) => parse_literal(tokens),
        Some((Token::Word(_), _)) => parse_identifier_or_function_call(tokens),
        Some(_) => Err("Unexpected token in expression"),
        None => Err("Unexpected end of tokens in expression"),
    }
}

fn parse_literal(tokens: &mut TokenStream) -> StatementResult {
    match tokens.next() {
        Some((Token::Integer(value), _)) => Ok(Stmt::Literal { value }),
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
                            )
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
    use super::*;
    use crate::lexer;
    use crate::parser::Stmt::{BinaryExpression, Literal};
    use crate::parser::{parse, Operator};

    /// Helper function for testing the parse_expression function
    fn lex_then_parse(input: &'static str) -> Vec<Stmt> {
        let tokens = lexer::lex(input).expect("Failed to lex");

        println!("tokens {:?}", tokens);

        match parse(&mut tokens.clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {}", e),
        }
    }

    #[test]
    #[should_panic]
    fn repeated_identifiers() {
        lex_then_parse("chicken chicken chicken chicken chicken chicken chicken chicken");
    }

    #[test]
    #[should_panic]
    fn repeated_numbers() {
        lex_then_parse("1 2 3 4 5 6 7 8 9 10");
    }

    #[test]
    fn single_identifier() {
        lex_then_parse("chicken");
    }

    #[test]
    fn single_number() {
        lex_then_parse("42");
    }

    #[test]
    fn basic_expression() {
        let stmt = lex_then_parse("1 + 7 * (3 - 4) / 5");

        assert_eq!(
            stmt,
            vec![BinaryExpression {
                left: Box::new(Literal { value: 1 }),
                right: Box::new(BinaryExpression {
                    left: Box::new(BinaryExpression {
                        left: Box::new(Literal { value: 7 }),
                        right: Box::new(BinaryExpression {
                            left: Box::new(Literal { value: 3 }),
                            right: Box::new(Literal { value: 4 }),
                            operator: Operator::Subtract,
                        }),
                        operator: Operator::Multiply,
                    }),
                    right: Box::new(Literal { value: 5 }),
                    operator: Operator::Divide,
                }),
                operator: Operator::Add
            }]
        )
    }

    #[test]
    fn function_call_no_args() {
        let stmt = lex_then_parse("foo()");

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
        let stmt = lex_then_parse("bar(42)");

        assert_eq!(
            stmt,
            vec![Stmt::FunctionCall {
                name: "bar".to_string(),
                args: vec![Stmt::Literal { value: 42 }],
            }]
        );
    }

    #[test]
    fn function_call_multiple_args() {
        let stmt = lex_then_parse("baz(1, 2, 3)");

        assert_eq!(
            stmt,
            vec![Stmt::FunctionCall {
                name: "baz".to_string(),
                args: vec![
                    Stmt::Literal { value: 1 },
                    Stmt::Literal { value: 2 },
                    Stmt::Literal { value: 3 },
                ],
            }]
        );
    }

    #[test]
    fn function_call_with_expression_args() {
        let stmt = lex_then_parse("calculate(1 + (2 + 5), 3 * 4)");

        assert_eq!(
            stmt,
            vec![Stmt::FunctionCall {
                name: "calculate".to_string(),
                args: vec![
                    Stmt::BinaryExpression {
                        left: Box::new(Stmt::Literal { value: 1 }),
                        right: Box::new(Stmt::BinaryExpression {
                            left: Box::new(Stmt::Literal { value: 2 }),
                            right: Box::new(Stmt::Literal { value: 5 }),
                            operator: Operator::Add,
                        }),
                        operator: Operator::Add,
                    },
                    Stmt::BinaryExpression {
                        left: Box::new(Stmt::Literal { value: 3 }),
                        right: Box::new(Stmt::Literal { value: 4 }),
                        operator: Operator::Multiply,
                    },
                ],
            }]
        );
    }
}
