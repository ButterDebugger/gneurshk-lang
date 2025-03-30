use crate::tokens::Token;
use std::iter::Peekable;
use std::slice::Iter;

/// An alias for the result of parsing a single statement
type StatementResult = Result<Stmt, &'static str>;
/// An alias for the result of parsing multiple statements
type MultiStatementResult = Result<Vec<Stmt>, &'static str>;
/// An alias for a peekable iterator of tokens
type TokenStream<'a> = Peekable<Iter<'a, Token>>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    GreaterThan,
    GreaterThanEqual,
    Equal,
    LessThanEqual,
    LessThan,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Stmt {
    Declaration {
        mutable: bool,
        name: String,
        value: Option<Box<Stmt>>,
    },
    IfStatement {
        condition: Box<Stmt>,
        body: Vec<Stmt>,
    },
    // FunctionDeclaration {
    //     name: String,
    //     body: Vec<Stmt>,
    // },
    // FunctionCall {},
    BinaryExpression {
        left: Box<Stmt>,
        right: Box<Stmt>,
        operator: Operator,
    },
    Literal {
        value: isize,
    },
    // TypeAlias {
    //     name: String,
    //     value: Box<Stmt>,
    // },
}

/// Parses statements that appear directly after an new line and or indentation
pub fn parse(tokens: &mut TokenStream) -> MultiStatementResult {
    let mut tokens = tokens;
    let mut stmts = vec![];

    while let Some(&token) = tokens.peek() {
        match token {
            Token::NewLine => {
                tokens.next(); // Consume new line token
                continue;
            }
            _ => {}
        }

        let statement = parse_statement(&mut tokens);

        // Append statements or catch and throw errors
        match statement {
            Ok(stmt) => stmts.push(stmt),
            Err(e) => return Err(e),
        }
    }

    Ok(stmts)
}

fn parse_statement(tokens: &mut TokenStream) -> StatementResult {
    let mut tokens = tokens;

    let token = match tokens.peek() {
        Some(e) => e,
        _ => todo!("Unexpected end of tokens"),
    };

    match token {
        Token::Var => parse_variable_declaration(&mut tokens),
        Token::Const => parse_variable_declaration(&mut tokens),
        Token::If => parse_if_statement(&mut tokens),
        Token::Integer(_) => parse_expression(&mut tokens),
        Token::OpenParen => parse_expression(&mut tokens),
        _ => {
            todo!("Unexpected token: {:#?}", token)
        }
    }
}

fn parse_variable_declaration(tokens: &mut TokenStream) -> StatementResult {
    let mutable = match tokens.next() {
        Some(Token::Var) => false,
        Some(Token::Const) => true,
        _ => return Err("Expected variable declaration"),
    };

    // Read variable name
    let name = match tokens.next() {
        Some(Token::Word(name)) => name,
        _ => return Err("Expected variable name"),
    };

    let has_value = match tokens.peek() {
        Some(Token::Equal) => true,
        _ => false,
    };

    if has_value {
        tokens.next(); // Consume token

        let value = match parse_expression(tokens) {
            Ok(e) => e,
            _ => return Err("Expected an expression"),
        };

        return Ok(Stmt::Declaration {
            mutable,
            name: name.to_string(),
            value: Some(Box::new(value)),
        });
    }

    Ok(Stmt::Declaration {
        mutable,
        name: name.to_string(),
        value: None,
    })
}

/// Parses a binary expression based on operator priority
fn parse_expression(tokens: &mut TokenStream) -> StatementResult {
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
            // TODO: handle identifiers and function calls
            Some(_) => Err("Unexpected token in expression"),
            None => Err("Unexpected end of tokens"),
        }
    }

    parse_comparison(tokens)
}

fn parse_literal(tokens: &mut TokenStream) -> StatementResult {
    return match tokens.next() {
        Some(Token::Integer(val)) => Ok(Stmt::Literal { value: val.clone() }),
        _ => Err("Expected literal"),
    };
}

fn parse_if_statement(tokens: &mut TokenStream) -> StatementResult {
    // Consume the If token
    match tokens.next() {
        Some(Token::If) => {}
        _ => return Err("Expected if statement"),
    }

    // Parse the condition
    let condition = match parse_expression(tokens) {
        Ok(e) => e,
        _ => return Err("Expected expression"),
    };

    // Expect a colon after the condition
    match tokens.next() {
        Some(Token::Colon) => {}
        _ => return Err("Expected colon after if condition"),
    }

    // Parse the body of the if statement
    let body = parse_indented_body(tokens)?;

    Ok(Stmt::IfStatement {
        condition: Box::new(condition),
        body,
    })
}

fn parse_indented_body(tokens: &mut TokenStream) -> MultiStatementResult {
    // Consume the Indent token
    match tokens.next() {
        Some(Token::Indent) => {}
        _ => return Err("Expected line indent"),
    }

    let mut body = vec![];

    // Keep appending statements until a Dedent token is encountered
    while let Some(&token) = tokens.peek() {
        match token {
            Token::NewLine => {
                tokens.next(); // Consume the token
                continue;
            }
            Token::Dedent => {
                tokens.next(); // Consume the token
                break; // End of the block
            }
            _ => {}
        }

        let statement = parse_statement(tokens)?;
        body.push(statement);
    }

    Ok(body)
}
