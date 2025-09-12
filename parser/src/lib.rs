use crate::block::parse_block;
use crate::expressions::parse_expression;
use crate::ifs::parse_if_statement;
use crate::imports::parse_import;
use crate::variables::parse_variable_declaration;
use funcs::parse_func_declaration;
use gneurshk_lexer::Scanner;
use gneurshk_lexer::tokens::Token;
use std::iter::Peekable;
mod block;
mod expressions;
mod funcs;
mod ifs;
mod imports;
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
    NotEqual,
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
    Block {
        body: Vec<Stmt>,
    },
    IfStatement {
        condition: Box<Stmt>,
        /// Should always be a block statement
        block: Box<Stmt>,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<Stmt>,
        return_type: String,
        /// Should always be a block statement
        block: Box<Stmt>,
    },
    FunctionParam {
        name: String,
        type_name: String,
        default_value: Option<Box<Stmt>>,
    },
    FunctionCall {
        name: String,
        args: Vec<Stmt>,
    },
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
    //     types: Vec<String>,
    // },
    ImportModule {
        module: String,
        alias: Option<String>,
    },
    ImportModules {
        modules: Vec<(String, Option<String>)>,
    },
    ImportEverything {
        module: String,
    },
    ImportCollection {
        module: String,
        items: Vec<(String, Option<String>)>,
    },
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
        Token::Import => {
            single_line = true;

            parse_import(tokens)
        }
        Token::OpenBrace => parse_block(tokens),
        _ => {
            println!("token: {token:?}");
            return Err("Unexpected token");
        }
    };

    // If the statement is single lined, expect a new line
    if single_line {
        match tokens.peek() {
            Some((Token::NewLine, _)) => {
                tokens.next(); // Consume the new line token
            }
            Some((Token::CloseBrace, _)) | None => {} // Ignore the end of block and the end of tokens
            _ => return Err("Expected new line"),
        }
    }

    // Return the parsed statement
    stmt
}
