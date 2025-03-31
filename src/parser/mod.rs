use crate::lexer::tokens::Token;
use crate::parser::expressions::parse_expression;
use crate::parser::ifs::parse_if_statement;
use crate::parser::variables::parse_variable_declaration;
use std::iter::Peekable;
use std::slice::Iter;
mod expressions;
mod ifs;
mod variables;

/// An alias for the result of parsing a single statement
pub type StatementResult = Result<Stmt, &'static str>;
/// An alias for the result of parsing multiple statements
pub type MultiStatementResult = Result<Vec<Stmt>, &'static str>;
/// An alias for a peekable iterator of tokens
pub type TokenStream<'a> = Peekable<Iter<'a, Token>>;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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

fn parse_literal(tokens: &mut TokenStream) -> StatementResult {
    return match tokens.next() {
        Some(Token::Integer(val)) => Ok(Stmt::Literal { value: val.clone() }),
        _ => Err("Expected literal"),
    };
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

#[cfg(test)]
mod tests {
    use super::{parse, Stmt};
    use crate::lexer;

    /// Helper function for testing the parse function
    fn lex_then_parse(input: &str) -> Vec<Stmt> {
        let tokens = lexer::lex(input);

        match parse(&mut tokens.iter().peekable().clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {}", e),
        }
    }

    #[test]
    fn large_indented_if_block() {
        lex_then_parse(
            r"
if 10 + 10:
    var apple = 2








    var green = 5

var borg = 5
",
        );
    }
}
