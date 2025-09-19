use crate::codegen::scope::Scope;
use gneurshk_parser::Stmt;
use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use std::collections::HashMap;

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

    pub fn compile(&mut self, ast: Vec<Stmt>) {
        // Create main function
        let i32_type = self.context.i32_type();
        let main_type = i32_type.fn_type(&[], false);
        let main_function = self.module.add_function("main", main_type, None);
        let basic_block = self.context.append_basic_block(main_function, "entry");

        self.builder.position_at_end(basic_block);

        // Compile all statements
        for stmt in ast {
            self.compile_stmt(stmt);
        }

        // Return 0 from main
        let zero = i32_type.const_int(0, false);
        self.builder.build_return(Some(&zero)).unwrap();
    }

    fn compile_stmt(&mut self, stmt: Stmt) -> Option<BasicValueEnum<'ctx>> {
        match stmt {
            Stmt::Declaration {
                mutable: _,
                name,
                value,
            } => self.compile_declaration(name, value),
            Stmt::Block { body } => self.compile_block(body),
            Stmt::IfStatement {
                condition,
                block,
                else_block,
            } => self.compile_if_statement(*condition, *block, else_block.map(|b| *b)),
            Stmt::FunctionDeclaration {
                name,
                params,
                return_type,
                block,
            } => self.compile_function_declaration(name, params, return_type, *block),
            Stmt::FunctionCall { name, args } => self.compile_function_call(name, args),
            Stmt::BinaryExpression {
                left,
                right,
                operator,
            } => self.compile_binary_expression(*left, *right, operator),
            Stmt::Identifier { name } => self.compile_identifier(name),
            Stmt::Literal { value } => self.compile_literal(value),
            Stmt::ReturnStatement { value } => self.compile_return_statement(value),
            _ => {
                // TODO: Handle other statements
                None
            }
        }
    }
}
