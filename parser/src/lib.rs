use crate::block::parse_block;
use crate::expressions::parse_expression;
use crate::ifs::parse_if_statement;
use crate::imports::parse_import;
use crate::returns::parse_return_statement;
use crate::types::DataType;
use crate::variables::parse_variable_declaration;
use funcs::parse_func_declaration;
use gneurshk_lexer::TokenStream;
use gneurshk_lexer::tokens::Token;
use std::ops::Range;

mod block;
mod expressions;
mod funcs;
mod ifs;
mod imports;
mod returns;
pub mod types;
mod variables;

/// An alias for the result of parsing a single statement
pub type StatementResult = Result<Stmt, &'static str>;
/// An alias for the result of parsing multiple statements
pub type MultiStatementResult = Result<Vec<Stmt>, &'static str>;
/// An alias for the result of parsing a program
pub type ProgramResult = Result<Program, &'static str>;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub imports: Vec<Stmt>,
    pub functions: Vec<Stmt>,
    pub body: Vec<Stmt>,
}

/// A binary operator which takes in two operands
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
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
    And,
    Or,
}

/// A unary operator which takes in one operand
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
    Negative,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParam {
    pub name: String,
    pub data_type: DataType,
    pub default_value: Option<Box<Stmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Annotation {
    pub name: String,
    pub args: Vec<Stmt>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Declaration {
        mutable: bool,
        name: String,
        data_type: Option<DataType>,
        value: Option<Box<Stmt>>,
    },
    Block {
        body: Vec<Stmt>,
    },
    IfStatement {
        condition: Box<Stmt>,
        /// Should always be a block statement
        block: Box<Stmt>,
        else_block: Option<Box<Stmt>>,
    },
    FunctionDeclaration {
        annotations: Vec<Annotation>,
        name: String,
        params: Vec<FunctionParam>,
        return_type: DataType,
        /// Should always be a block statement
        block: Box<Stmt>,
    },
    FunctionCall {
        name: String,
        args: Vec<Stmt>,
        span: Range<usize>,
    },
    BinaryExpression {
        left: Box<Stmt>,
        right: Box<Stmt>,
        operator: BinaryOperator,
    },
    UnaryExpression {
        value: Box<Stmt>,
        operator: UnaryOperator,
    },
    Identifier {
        name: String,
        span: Range<usize>,
    },
    Integer {
        value: u64,
        span: Range<usize>,
    },
    Float {
        value: f64,
        span: Range<usize>,
    },
    Boolean {
        value: bool,
        span: Range<usize>,
    },
    String {
        value: String,
        span: Range<usize>,
    },
    ReturnStatement {
        value: Option<Box<Stmt>>,
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
pub fn parse(tokens: &mut TokenStream) -> ProgramResult {
    let mut imports = vec![];
    let mut functions = vec![];
    let mut stmts = vec![];

    while let Some((token, _)) = tokens.peek() {
        if token == &Token::NewLine {
            tokens.next(); // Consume new line token
            continue;
        }

        let statement = parse_statement(tokens);

        // Append statements or catch and throw errors
        match statement {
            Ok(stmt) => match stmt {
                Stmt::ImportModule { .. }
                | Stmt::ImportModules { .. }
                | Stmt::ImportEverything { .. }
                | Stmt::ImportCollection { .. } => {
                    imports.push(stmt);
                }
                Stmt::FunctionDeclaration { .. } => {
                    functions.push(stmt);
                }
                _ => {
                    stmts.push(stmt);
                }
            },
            Err(e) => return Err(e),
        }
    }

    Ok(Program {
        imports,
        functions,
        body: stmts,
    })
}

fn parse_statement(tokens: &mut TokenStream) -> StatementResult {
    // Peek at the next token
    let (token, _) = match tokens.peek() {
        Some(e) => e,
        _ => return Err("Unexpected end of tokens at beginning of line"),
    };

    // Parse the statement
    let stmt = match token {
        Token::Var | Token::Const => parse_variable_declaration(tokens),
        Token::If => parse_if_statement(tokens),
        Token::Integer(_)
        | Token::Float(_)
        | Token::Boolean(_)
        | Token::String(_)
        | Token::OpenParen
        | Token::Word(_)
        | Token::Minus
        | Token::Not => parse_expression(tokens),
        Token::Annotation(_) | Token::Func => parse_func_declaration(tokens),
        Token::Import => parse_import(tokens),
        Token::OpenBrace => parse_block(tokens),
        Token::Return => parse_return_statement(tokens),
        _ => {
            println!("token: {token:?}");
            return Err("Unexpected token");
        }
    };

    // Consume a NewLine token if its present
    if let Some((Token::NewLine, _)) = tokens.peek() {
        tokens.next(); // Consume the new line token
    }

    // Return the parsed statement
    stmt
}
