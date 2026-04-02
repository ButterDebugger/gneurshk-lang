use crate::{
    errors::{SematicError, SematicWarning},
    scope::{Function, Scope},
};
use gneurshk_parser::{
    BinaryExpression, Expression, FunctionCall, Identifier, Program, Stmt, types::DataType,
};
use std::collections::HashMap;

mod binary_expression;
mod declaration;
mod errors;
mod function_call;
mod identifier;
mod literal;
mod scope;

#[derive(Debug, Clone)]
pub struct Analyzer {
    scope: Box<Scope>,

    functions: HashMap<String, Function>,

    pub errors: Vec<SematicError>,
    pub warnings: Vec<SematicWarning>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            scope: Box::new(Scope::new(None)),

            functions: HashMap::new(),

            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn analyze(&mut self, program: Program) -> Result<(), String> {
        for function in program.functions {
            match function {
                Stmt::FunctionDeclaration {
                    name,
                    params,
                    return_type,
                    ..
                } => {
                    self.functions.insert(
                        name,
                        Function {
                            return_type,
                            params,
                        },
                    );
                }
                _ => return Err("Expected function declaration".to_string()),
            }
        }

        for statement in program.body {
            self.analyze_statement(statement);
        }

        // Check for unused variables
        for variable in self.scope.get_unused_variables() {
            self.warnings
                .push(SematicWarning::UnusedVariable(variable.name));
        }

        Ok(())
    }

    fn analyze_statement(&mut self, statement: Stmt) -> Option<DataType> {
        match statement {
            Stmt::BinaryExpression(BinaryExpression {
                left,
                operator,
                right,
            }) => self.analyze_binary_expression(left, right, operator),
            Stmt::String(..) => self.analyze_string(),
            Stmt::Integer(..) => self.analyze_integer(),
            Stmt::Float(..) => self.analyze_float(),
            Stmt::Boolean(..) => self.analyze_boolean(),
            Stmt::Identifier(Identifier { name, .. }) => self.analyze_identifier(name),
            Stmt::FunctionCall(FunctionCall { name, args, .. }) => {
                self.analyze_function_call(name, args)
            }
            Stmt::Declaration {
                mutable,
                name,
                data_type,
                value,
            } => self.analyze_declaration(mutable, name, data_type, value),
            _ => {
                println!("statement: {statement:?}");

                todo!();
            }
        }
    }

    fn analyze_expression(&mut self, expr: Expression) -> Option<DataType> {
        match expr {
            Expression::BinaryExpression(BinaryExpression {
                left,
                right,
                operator,
            }) => self.analyze_binary_expression(left, right, operator),
            Expression::String(..) => self.analyze_string(),
            Expression::Integer(..) => self.analyze_integer(),
            Expression::Float(..) => self.analyze_float(),
            Expression::Boolean(..) => self.analyze_boolean(),
            Expression::Identifier(Identifier { name, .. }) => self.analyze_identifier(name),
            Expression::FunctionCall(FunctionCall { name, args, .. }) => {
                self.analyze_function_call(name, args)
            }
            _ => {
                println!("expression: {expr:?}");

                todo!();
            }
        }
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}
