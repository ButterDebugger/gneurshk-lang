use gneurshk_parser::{Operator, Stmt};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

pub struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        Self {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),
        }
    }

    pub fn compile(&mut self, ast: Vec<Stmt>) {
        for stmt in ast {
            self.compile_stmt(stmt);
        }
    }

    fn compile_stmt(&mut self, expression: Stmt) {
        match expression {
            _ => todo!(),
        }
    }
}
