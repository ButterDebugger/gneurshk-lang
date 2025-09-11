use crate::codegen::Codegen;
use gneurshk_parser::Stmt;
use inkwell::context::Context;
mod codegen;

pub fn compile(ast: Vec<Stmt>) {
    let context = Context::create();

    let mut codegen = Codegen::new(&context, "main");

    codegen.compile(ast);
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
