use crate::lexer::tokens::Token;
use crate::lexer::Scanner;
use crate::parser::expressions::parse_expression;
use crate::parser::ifs::parse_if_statement;
use crate::parser::variables::parse_variable_declaration;
use funcs::parse_func_declaration;
use std::iter::Peekable;
mod expressions;
mod funcs;
mod ifs;
mod variables;

/// An alias for the result of parsing a single statement
pub type StatementResult = Result<Stmt, &'static str>;
/// An alias for the result of parsing multiple statements
pub type MultiStatementResult = Result<Vec<Stmt>, &'static str>;
/// An alias for a peekable iterator of tokens
pub type TokenStream<'a> = Peekable<Scanner<'a>>;

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
    FunctionDeclaration {
        name: String,
        params: Vec<Stmt>,
        return_type: String,
        body: Vec<Stmt>,
    },
    FunctionParam {
        name: String,
        type_name: String,
        default_value: Option<Box<Stmt>>,
    },
    // FunctionCall {},
    BinaryExpression {
        left: Box<Stmt>,
        right: Box<Stmt>,
        operator: Operator,
    },
    Identifier {
        name: String,
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
    let mut stmts = vec![];

    while let Some((token, _)) = tokens.peek() {
        if token == &Token::NewLine {
            tokens.next(); // Consume new line token
            continue;
        }

        let statement = parse_statement(tokens);

        // Append statements or catch and throw errors
        match statement {
            Ok(stmt) => stmts.push(stmt),
            Err(e) => return Err(e),
        }
    }

    Ok(stmts)
}

fn parse_statement(tokens: &mut TokenStream) -> StatementResult {
    // Peek at the next token
    let (token, _) = match tokens.peek() {
        Some(e) => e,
        _ => return Err("Unexpected end of tokens at beginning of line"),
    };

    // Parse the statement
    let mut single_line = false;
    let stmt = match token {
        Token::Var | Token::Const => {
            single_line = true;

            parse_variable_declaration(tokens)
        }
        Token::If => parse_if_statement(tokens),
        Token::Integer(_) | Token::OpenParen | Token::Word(_) => {
            single_line = true;

            parse_expression(tokens)
        }
        Token::Func => parse_func_declaration(tokens),
        _ => return Err("Unexpected token"),
    };

    // If the statement is single lined, expect a new line
    if single_line {
        match tokens.peek() {
            Some((Token::NewLine, _)) => {
                tokens.next(); // Consume the new line token
            }
            Some((Token::Indent, _)) | Some((Token::Dedent, _)) | None => {} // Ignore indentation and the end of tokens
            _ => return Err("Expected new line"),
        }
    }

    // Return the parsed statement
    stmt
}

fn parse_literal(tokens: &mut TokenStream) -> StatementResult {
    match tokens.next() {
        Some((Token::Integer(value), _)) => Ok(Stmt::Literal { value }),
        _ => Err("Expected literal"),
    }
}

fn parse_identifier(tokens: &mut TokenStream) -> StatementResult {
    match tokens.next() {
        Some((Token::Word(val), _)) => Ok(Stmt::Identifier { name: val.clone() }),
        _ => Err("Expected identifier"),
    }
}

fn parse_indented_body(tokens: &mut TokenStream) -> MultiStatementResult {
    // Consume the Indent token
    match tokens.next() {
        Some((Token::Indent, _)) => {}
        _ => return Err("Expected line indent"),
    }

    let mut body = vec![];

    // Keep appending statements until a Dedent token is encountered
    loop {
        match tokens.peek() {
            Some((Token::Dedent, _)) => {
                tokens.next(); // Consume the token
                break; // End of the block
            }
            None => return Err("Unexpected end of tokens in indented block"),
            _ => {}
        }

        let statement = parse_statement(tokens)?;
        body.push(statement);
    }

    Ok(body)
}
