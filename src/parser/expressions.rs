use super::{parse_identifier, parse_literal, Operator, StatementResult, Stmt, TokenStream};
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
        Some(Token::GreaterThan) => Some(Operator::GreaterThan),
        Some(Token::GreaterThanEqual) => Some(Operator::GreaterThanEqual),
        Some(Token::Equal) => Some(Operator::Equal), // Assuming Token::Equal is for '==' comparison
        Some(Token::LessThanEqual) => Some(Operator::LessThanEqual),
        Some(Token::LessThan) => Some(Operator::LessThan),
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
        Some(Token::Plus) => Some(Operator::Add),
        Some(Token::Minus) => Some(Operator::Subtract),
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
        Some(Token::Multiply) => Some(Operator::Multiply),
        Some(Token::Divide) => Some(Operator::Divide),
        Some(Token::Modulus) => Some(Operator::Modulus),
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
        Some(Token::OpenParen) => {
            tokens.next(); // Consume the '(' token

            // Recursively parse the inner expression
            let expression = parse_expression(tokens)?;

            // Consume the ')' token and return the expression
            match tokens.next() {
                Some(Token::CloseParen) => Ok(expression),
                _ => Err("Expected a closing parenthesis"),
            }
        }
        Some(Token::Integer(_)) => parse_literal(tokens),
        Some(Token::Word(_)) => parse_identifier(tokens),
        // TODO: handle function calls
        Some(_) => Err("Unexpected token in expression"),
        None => Err("Unexpected end of tokens"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;
    use crate::parser::Operator;
    use crate::parser::Stmt::{BinaryExpression, Literal};

    /// Helper function for testing the parse_expression function
    fn lex_then_parse(input: &str) -> Stmt {
        let tokens = lexer::lex(input).expect("Failed to lex");

        match parse_expression(&mut tokens.iter().peekable().clone()) {
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

        match stmt {
            // TODO: make this more readable, maybe with assert matches instead
            BinaryExpression {
                left,
                right,
                operator,
            } => {
                assert_eq!(left, Box::new(Literal { value: 1 }));
                assert_eq!(
                    right,
                    Box::new(BinaryExpression {
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
                    })
                );
                assert_eq!(operator, Operator::Add);
            }
            _ => unreachable!(),
        };
    }
}
