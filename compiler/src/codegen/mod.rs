use crate::codegen::scope::Scope;
use anyhow::{Result, anyhow};
use gneurshk_parser::{
    Assignment, BinaryExpression, BooleanLit, Expression, FloatLit, FunctionDeclaration,
    IfStatement, IntegerLit, Program, Return, Stmt, StringLit, UnaryExpression,
};
use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::BasicValueEnum;
use std::collections::HashMap;

mod assignment;
mod binary_expression;
mod block;
mod declaration;
mod function_call;
mod function_declaration;
mod identifier;
mod if_statement;
mod literal;
mod return_statement;
mod scope;
mod unary_expression;

pub struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    scope: Box<Scope<'ctx>>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let mut codegen = Self {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),

            scope: Box::new(Scope::new(None)),
        };

        // Add built-in functions
        codegen.add_builtin_functions();
        codegen
    }

    fn add_builtin_functions(&mut self) {
        // Add printf function for println
        let i8_ptr_type = self.context.ptr_type(AddressSpace::default());
        let i32_type = self.context.i32_type();

        let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
        let printf_function = self.module.add_function("printf", printf_type, None);
        self.scope.set_function("printf", printf_function);
    }

    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    pub fn compile(&mut self, program: Program) -> Result<()> {
        // Prebuild all function declarations so they can reference each other
        let mut functions = HashMap::new();

        for function in program.functions.clone() {
            let FunctionDeclaration { name, params, .. } = function;

            functions.insert(name.clone(), self.build_function_declaration(name, params));
        }

        // Check if the program has an entry point
        if !functions.contains_key("main") {
            return Err(anyhow!(
                "No program entry point found. Please define a main function."
            ));
        }

        // Build all function bodies
        for function in program.functions {
            let FunctionDeclaration {
                name,
                params,
                return_type,
                block,
                ..
            } = function;

            let function = functions.remove(&name).unwrap();

            self.build_function_body(function, params, return_type, *block);
        }

        Ok(())
    }

    fn build_stmt(&mut self, stmt: Stmt) -> Option<BasicValueEnum<'ctx>> {
        match stmt {
            Stmt::VariableDeclaration(variable) => self.build_declaration(variable),
            Stmt::Assignment(Assignment { member, value }) => self.build_assignment(member, value),
            Stmt::Block(block) => self.build_block(block),
            Stmt::IfStatement(IfStatement {
                condition,
                if_block: block,
                else_statement: else_block,
            }) => self.build_if_statement(*condition, *block, else_block.map(|b| *b)),
            Stmt::Identifier(identifier) => self.build_identifier(identifier),
            Stmt::FunctionCall(function_call) => self.build_function_call(function_call),
            Stmt::MemberAccess(_) => todo!(),
            Stmt::BinaryExpression(BinaryExpression {
                left,
                right,
                operator,
            }) => self.build_binary_expression(*left, *right, operator),
            Stmt::UnaryExpression(UnaryExpression { value, operator }) => {
                self.build_unary_expression(*value, operator)
            }
            Stmt::Integer(IntegerLit { value, .. }) => self.build_integer(value),
            Stmt::Float(FloatLit { value, .. }) => self.build_float(value),
            Stmt::String(StringLit { value, .. }) => self.build_global_string(value),
            Stmt::Boolean(BooleanLit { value, .. }) => self.build_boolean(value),
            Stmt::Return(Return { value }) => self.build_return_statement(value),
        }
    }

    fn build_expression(&mut self, expr: Expression) -> Option<BasicValueEnum<'ctx>> {
        match expr {
            Expression::Identifier(identifier) => self.build_identifier(identifier),
            Expression::FunctionCall(function_call) => self.build_function_call(function_call),
            Expression::MemberAccess(_) => todo!(),
            Expression::BinaryExpression(BinaryExpression {
                left,
                right,
                operator,
            }) => self.build_binary_expression(*left, *right, operator),
            Expression::UnaryExpression(UnaryExpression { value, operator }) => {
                self.build_unary_expression(*value, operator)
            }
            Expression::Integer(IntegerLit { value, .. }) => self.build_integer(value),
            Expression::Float(FloatLit { value, .. }) => self.build_float(value),
            Expression::String(StringLit { value, .. }) => self.build_global_string(value),
            Expression::Boolean(BooleanLit { value, .. }) => self.build_boolean(value),
            _ => {
                // TODO: Handle other expressions
                None
            }
        }
    }
}
