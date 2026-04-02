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
mod identifiers;
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
    pub imports: Vec<ImportStmt>,
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
    pub mutable: bool,
    pub data_type: DataType,
    pub default_value: Option<Box<Stmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Annotation {
    pub name: String,
    pub args: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MemberExpressionMember {
    Identifier(Identifier),
    FunctionCall(FunctionCall),
}

#[derive(Debug, PartialEq, Clone)]
pub enum MemberExpressionBase {
    Identifier(Identifier),
    FunctionCall(FunctionCall),
    MemberAccess(MemberAccess),
}

impl From<MemberExpressionBase> for Stmt {
    fn from(val: MemberExpressionBase) -> Self {
        match val {
            MemberExpressionBase::Identifier(identifier) => Stmt::Identifier(identifier),
            MemberExpressionBase::FunctionCall(function_call) => Stmt::FunctionCall(function_call),
            MemberExpressionBase::MemberAccess(member_access) => Stmt::MemberAccess(member_access),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Stmt>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemberAccess {
    pub base: Box<MemberExpressionBase>,
    pub member: MemberExpressionMember,
    pub is_static: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportStmt {
    Module(ImportModule),
    Modules(ImportModules),
    Everything(ImportEverything),
    Collection(ImportCollection),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportModule {
    module: String,
    alias: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportModules {
    modules: Vec<(String, Option<String>)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportEverything {
    module: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportCollection {
    module: String,
    items: Vec<(String, Option<String>)>,
}

#[derive(Debug, PartialEq, Clone)]
/// Represents anything that can be evaluated to a value
pub enum Expression {
    Block(Block),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    IfStatement(IfStatement),
    Literal(Literal),
    Identifier(Identifier),
    FunctionCall(FunctionCall),
    MemberAccess(MemberAccess),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub condition: Box<Stmt>,
    pub if_block: Box<Block>,
    pub else_statement: Option<Box<Stmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Integer(Integer),
    Float(Float),
    Boolean(Boolean),
    String(StringLiteral),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Integer {
    pub value: u64,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    pub value: f64,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Boolean {
    pub value: bool,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub left: Box<Stmt>,
    pub right: Box<Stmt>,
    pub operator: BinaryOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    pub value: Box<Stmt>,
    pub operator: UnaryOperator,
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
    Block(Block),
    IfStatement(IfStatement),
    FunctionDeclaration {
        annotations: Vec<Annotation>,
        name: String,
        params: Vec<FunctionParam>,
        return_type: DataType,
        block: Box<Block>,
    },
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    Identifier(Identifier),
    FunctionCall(FunctionCall),
    MemberAccess(MemberAccess),
    Literal(Literal),
    ReturnStatement {
        value: Option<Box<Stmt>>,
    },
    // TypeAlias {
    //     name: String,
    //     types: Vec<String>,
    // },
    Import(ImportStmt),
}

/// Parses statements that appear directly after an new line and or indentation
pub fn parse(tokens: &mut TokenStream) -> ProgramResult {
    let mut imports = vec![];
    let mut functions = vec![];
    let mut body = vec![];

    while let Some((token, _)) = tokens.peek() {
        if token == &Token::NewLine {
            tokens.next(); // Consume new line token
            continue;
        }

        let statement = parse_statement(tokens);

        // Append statements or catch and throw errors
        match statement {
            Ok(stmt) => match stmt {
                Stmt::Import(import) => {
                    imports.push(import);
                }
                Stmt::FunctionDeclaration { .. } => {
                    functions.push(stmt);
                }
                _ => {
                    body.push(stmt);
                }
            },
            Err(e) => return Err(e),
        }
    }

    Ok(Program {
        imports,
        functions,
        body,
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
        Token::OpenBrace => Ok(Stmt::Block(parse_block(tokens)?)),
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
