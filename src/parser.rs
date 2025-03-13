use crate::tokenize::Token;
use std::iter::Peekable;
use std::slice::Iter;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
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
}

/// Parses statements that appear directly after an new line and or indentation
pub fn parse(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Vec<Stmt>, &'static str> {
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

fn parse_statement(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Stmt, &'static str> {
    let mut tokens = tokens;

    let token = match tokens.peek() {
        Some(e) => e,
        _ => todo!("Unexpected end of tokens"),
    };

    match token {
        Token::Var => parse_variable_declaration(&mut tokens),
        Token::Const => parse_variable_declaration(&mut tokens),
        Token::If => parse_if_statement(&mut tokens),
        _ => {
            todo!("Unexpected token: {:#?}", token)
        }
    }
}

fn parse_variable_declaration(
    tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<Stmt, &'static str> {
    let mutable = match tokens.next() {
        Some(Token::Var) => false,
        Some(Token::Const) => true,
        _ => return Err("Expected variable declaration".into()),
    };

    // Read variable name
    let name = match tokens.next() {
        Some(Token::Word(name)) => name,
        _ => return Err("Expected variable name".into()),
    };

    let has_value = match tokens.peek() {
        Some(Token::Equal) => true,
        _ => false,
    };

    if has_value {
        tokens.next(); // Consume token

        let value = match parse_expression(tokens) {
            Ok(e) => e,
            _ => return Err("Expected expression".into()),
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

fn parse_expression(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Stmt, &'static str> {
    let left = match parse_literal(tokens) {
        Ok(e) => e,
        _ => return Err("Expected left expression".into()),
    };

    let operator: Option<Operator> = match tokens.peek() {
        Some(Token::Plus) => Some(Operator::Add),
        Some(Token::Minus) => Some(Operator::Sub),
        Some(Token::Multiply) => Some(Operator::Mul),
        Some(Token::Divide) => Some(Operator::Div),
        _ => None,
    };

    // If no operator, return literal
    if operator.is_none() {
        return Ok(left);
    } else {
        tokens.next(); // Consume operator token
    }

    let right = match parse_literal(tokens) {
        Ok(e) => e,
        _ => return Err("Expected right expression".into()),
    };

    Ok(Stmt::BinaryExpression {
        left: Box::new(left),
        right: Box::new(right),
        operator: operator.unwrap(),
    })
}

fn parse_literal(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Stmt, &'static str> {
    let token = match tokens.next() {
        Some(x) => x,
        _ => return Err("Expected literal".into()),
    };

    return match token {
        Token::Integer(val) => Ok(Stmt::Literal { value: val.clone() }),
        _ => Err("Expected literal".into()),
    };
}

fn parse_if_statement(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Stmt, &'static str> {
    // Consume the If token
    match tokens.next() {
        Some(Token::If) => {}
        _ => return Err("Expected if statement".into()),
    }

    // Parse the condition
    let condition = match parse_expression(tokens) {
        Ok(e) => e,
        _ => return Err("Expected expression".into()),
    };

    // Expect a colon after the condition
    match tokens.next() {
        Some(Token::Colon) => {}
        _ => return Err("Expected colon after if condition".into()),
    }

    // Parse the body of the if statement
    let body = parse_indented_body(tokens)?;

    Ok(Stmt::IfStatement {
        condition: Box::new(condition),
        body,
    })
}

fn parse_indented_body(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Vec<Stmt>, &'static str> {
    // Consume the Indent token
    match tokens.next() {
        Some(Token::Indent) => {}
        _ => return Err("Expected line indent".into()),
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
