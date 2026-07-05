use crate::function::FunctionAnalyzer;
use gneurshk_parser::{
    Assignment, BinaryExpression, FunctionCall, Identifier, Stmt, types::DataType,
};

impl<'a> FunctionAnalyzer<'a> {
    pub(crate) fn analyze_statement(&mut self, statement: Stmt) -> Option<DataType> {
        match statement {
            Stmt::BinaryExpression(BinaryExpression {
                left,
                right,
                operator,
            }) => self.analyze_binary_expression(*left, *right, operator),
            Stmt::String(..) => self.analyze_string(),
            Stmt::Integer(..) => self.analyze_integer(),
            Stmt::Float(..) => self.analyze_float(),
            Stmt::Boolean(..) => self.analyze_boolean(),
            Stmt::Identifier(Identifier { name, .. }) => self.analyze_identifier(name),
            Stmt::FunctionCall(FunctionCall { name, args, .. }) => {
                self.analyze_function_call(name, args)
            }
            Stmt::VariableDeclaration(variable) => self.analyze_variable_declaration(variable),
            Stmt::Assignment(Assignment { member, value }) => {
                self.analyze_assignment(member, value)
            }
            Stmt::Block(block) => self.analyze_block(block),
            Stmt::IfStatement(return_stmt) => self.analyze_if(return_stmt),
            Stmt::Return(return_stmt) => self.analyze_return(return_stmt),
            Stmt::Loop(loop_stmt) => self.analyze_loop(loop_stmt),
            Stmt::Break => {
                if self.loop_stack.is_empty() {
                    self.errors
                        .push(crate::errors::SematicError::BreakOutsideLoop);
                }

                None
            }
            Stmt::Continue => {
                if self.loop_stack.is_empty() {
                    self.errors
                        .push(crate::errors::SematicError::ContinueOutsideLoop);
                }

                None
            }
            _ => {
                println!("statement: {statement:?}");

                todo!();
            }
        }
    }
}
