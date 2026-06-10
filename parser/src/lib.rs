use crate::assignments::parse_assignment;
use crate::block::parse_block;
use crate::expressions::parse_expression;
use crate::ifs::parse_if_statement;
use crate::imports::parse_import;
use crate::returns::parse_return_statement;
use crate::types::DataType;
use crate::variables::parse_variable_declaration;
use anyhow::{Result, anyhow};
use funcs::parse_func_declaration;
use gneurshk_lexer::TokenStream;
use gneurshk_lexer::tokens::Token;
use std::ops::Range;

mod assignments;
mod block;
mod expressions;
mod funcs;
mod identifiers;
mod ifs;
mod imports;
mod returns;
pub mod types;
mod variables;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub imports: Vec<ImportStmt>,
    pub functions: Vec<FunctionDeclaration>,
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
    pub default_value: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Annotation {
    pub name: String,
    pub args: Vec<Expression>,
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

impl From<MemberExpressionBase> for Expression {
    fn from(val: MemberExpressionBase) -> Self {
        match val {
            MemberExpressionBase::Identifier(identifier) => Expression::Identifier(identifier),
            MemberExpressionBase::FunctionCall(function_call) => {
                Expression::FunctionCall(function_call)
            }
            MemberExpressionBase::MemberAccess(member_access) => {
                Expression::MemberAccess(member_access)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expression>,
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
    Integer(IntegerLit),
    Float(FloatLit),
    Boolean(BooleanLit),
    String(StringLit),
    Identifier(Identifier),
    FunctionCall(FunctionCall),
    MemberAccess(MemberAccess),
}

impl From<Expression> for Stmt {
    fn from(val: Expression) -> Self {
        match val {
            Expression::Block(block) => Stmt::Block(block),
            Expression::BinaryExpression(binary_expression) => {
                Stmt::BinaryExpression(binary_expression)
            }
            Expression::UnaryExpression(unary_expression) => {
                Stmt::UnaryExpression(unary_expression)
            }
            Expression::IfStatement(if_statement) => Stmt::IfStatement(if_statement),
            Expression::Integer(integer_lit) => Stmt::Integer(integer_lit),
            Expression::Float(float_lit) => Stmt::Float(float_lit),
            Expression::Boolean(boolean_lit) => Stmt::Boolean(boolean_lit),
            Expression::String(string_lit) => Stmt::String(string_lit),
            Expression::Identifier(identifier) => Stmt::Identifier(identifier),
            Expression::FunctionCall(function_call) => Stmt::FunctionCall(function_call),
            Expression::MemberAccess(member_access) => Stmt::MemberAccess(member_access),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub if_block: Box<Block>,
    pub else_statement: Option<Box<Stmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLit {
    pub value: u64,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FloatLit {
    pub value: f64,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanLit {
    pub value: bool,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLit {
    pub value: String,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: BinaryOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    pub value: Box<Expression>,
    pub operator: UnaryOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    pub member: MemberExpressionBase,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub annotations: Vec<Annotation>,
    pub name: String,
    pub params: Vec<FunctionParam>,
    pub return_type: Option<DataType>,
    pub block: Box<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableDeclaration {
    Mutable {
        name: String,
        data_type: Option<DataType>,
        value: Option<Expression>,
    },
    Constant {
        name: String,
        data_type: Option<DataType>,
        value: Expression,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub value: Option<Expression>,
}

/// Anything that can be declared at the top level of a program
#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    // TypeAlias {
    //     name: String,
    //     types: Vec<String>,
    // },
    Import(ImportStmt),
    Function(FunctionDeclaration),
}

/// Anything that can be declared inside a block
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Assignment(Assignment),
    VariableDeclaration(VariableDeclaration),
    Block(Block),
    IfStatement(IfStatement),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    Identifier(Identifier),
    FunctionCall(FunctionCall),
    MemberAccess(MemberAccess),
    Integer(IntegerLit),
    Float(FloatLit),
    Boolean(BooleanLit),
    String(StringLit),
    Return(Return),
}

/// Parses statements that appear directly after an new line and or indentation
pub fn parse(tokens: &mut TokenStream) -> Result<Program> {
    let mut imports = vec![];
    let mut functions = vec![];

    while let Some((token, _)) = tokens.peek() {
        if token == &Token::NewLine {
            tokens.next(); // Consume new line token
            continue;
        }

        // Append statements or catch and throw errors
        match parse_declaration(tokens) {
            Ok(stmt) => match stmt {
                Declaration::Import(import) => {
                    imports.push(import);
                }
                Declaration::Function(func) => {
                    functions.push(func);
                }
            },
            Err(e) => return Err(e),
        }
    }

    Ok(Program { imports, functions })
}

fn parse_declaration(tokens: &mut TokenStream) -> Result<Declaration> {
    // Peek at the next token
    let (token, _) = match tokens.peek() {
        Some(e) => e,
        _ => return Err(anyhow!("Unexpected end of tokens at beginning of line")),
    };

    // Parse the statement
    let stmt = match token {
        Token::Annotation(_) | Token::Func => {
            Declaration::Function(parse_func_declaration(tokens)?)
        }
        Token::Import => Declaration::Import(parse_import(tokens)?),
        _ => {
            println!("token: {token:?}");
            return Err(anyhow!("Unexpected token"));
        }
    };

    // Consume a NewLine token if its present
    if let Some((Token::NewLine, _)) = tokens.peek() {
        tokens.next(); // Consume the new line token
    }

    // Return the parsed statement
    Ok(stmt)
}

fn parse_statement(tokens: &mut TokenStream) -> Result<Stmt> {
    // Peek at the next token
    let (token, _) = match tokens.peek() {
        Some(e) => e,
        _ => return Err(anyhow!("Unexpected end of tokens at beginning of line")),
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
        | Token::Minus
        | Token::Not => Ok(parse_expression(tokens)?.into()),
        Token::Word(_) => {
            let mut lookahead_tokens = tokens.clone();

            lookahead_tokens.next(); // Consume the word token

            // Check if its an assignment
            match lookahead_tokens.peek() {
                Some((Token::Equal, _))
                | Some((Token::PlusEqual, _))
                | Some((Token::MinusEqual, _))
                | Some((Token::MultiplyEqual, _))
                | Some((Token::DivideEqual, _))
                | Some((Token::ModulusEqual, _)) => parse_assignment(tokens),
                // Otherwise, its an expression
                _ => Ok(parse_expression(tokens)?.into()),
            }
        }
        Token::OpenBrace => Ok(Stmt::Block(parse_block(tokens)?)),
        Token::Return => parse_return_statement(tokens),
        _ => {
            println!("token: {token:?}");
            return Err(anyhow!("Unexpected token"));
        }
    };

    // Consume a NewLine token if its present
    if let Some((Token::NewLine, _)) = tokens.peek() {
        tokens.next(); // Consume the new line token
    }

    // Return the parsed statement
    stmt
}
