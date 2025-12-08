use crate::{
    errors::{SematicError, SematicWarning},
    scope::{Function, Scope, Variable},
};
use gneurshk_parser::{BinaryOperator, Program, Stmt, types::DataType};
use std::collections::HashMap;

mod errors;
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
                    annotations: _,
                    name,
                    params,
                    return_type,
                    block: _,
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
            Stmt::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let left_type = self.analyze_statement(*left)?;
                let right_type = self.analyze_statement(*right)?;

                match operator {
                    BinaryOperator::Equal
                    | BinaryOperator::NotEqual
                    | BinaryOperator::GreaterThan
                    | BinaryOperator::GreaterThanEqual
                    | BinaryOperator::LessThan
                    | BinaryOperator::LessThanEqual
                    | BinaryOperator::And
                    | BinaryOperator::Or => {
                        return Some(DataType::Boolean);
                    }
                    _ => (),
                }

                match (left_type.clone(), right_type.clone()) {
                    (DataType::Int32, DataType::Int32) => Some(DataType::Int32),
                    (DataType::Float32, DataType::Float32) => Some(DataType::Float32),
                    _ => {
                        self.errors
                            .push(SematicError::TypeMismatch(left_type, right_type));

                        None
                    }
                }
            }
            Stmt::String { .. } => Some(DataType::String),
            Stmt::Integer { .. } => Some(DataType::Int32),
            Stmt::Float { .. } => Some(DataType::Float32),
            Stmt::Boolean { .. } => Some(DataType::Boolean),
            Stmt::FunctionCall { name, args, span } => {
                // Handle built-in functions
                if matches!(name.as_str(), "println" | "print") {
                    // Analyze arguments and ignore types for these functions
                    for arg in args {
                        self.analyze_statement(arg);
                    }
                    return Some(DataType::Void);
                }

                if let Some(function) = self.functions.get(&name).cloned() {
                    // Check for correct number of arguments
                    if args.len() != function.params.len() {
                        self.errors
                            .push(SematicError::FunctionCallArgumentCountMismatch(
                                name.clone(),
                                function.params.len(),
                                args.len(),
                            ));
                    }
                    // Check for correct types of arguments
                    else {
                        let mut arg_types = Vec::with_capacity(args.len());
                        for arg in args {
                            arg_types.push(self.analyze_statement(arg)?);
                        }

                        let expected_types = function
                            .params
                            .iter()
                            .map(|param| param.data_type.clone())
                            .collect::<Vec<_>>();

                        for (i, (expected, actual)) in
                            expected_types.iter().zip(arg_types.iter()).enumerate()
                        {
                            if expected != actual {
                                self.errors.push(SematicError::FunctionCallArgumentMismatch(
                                    name.clone(),
                                    i + 1,
                                    expected.clone(),
                                    actual.clone(),
                                ));
                            }
                        }
                    }

                    Some(function.return_type.clone())
                } else {
                    self.errors.push(SematicError::FunctionNotFound(name));

                    // TODO: analyze arguments anyway

                    None
                }
            }
            Stmt::Identifier { name, .. } => {
                if let Some(variable) = self.scope.get_mut_variable(&name) {
                    variable.used = true;

                    Some(variable.data_type.clone())
                } else {
                    self.errors.push(SematicError::VariableNotFound(name));

                    None
                }
            }
            Stmt::Declaration {
                mutable,
                name,
                data_type,
                value,
            } => {
                let var_type = if let Some(dt) = data_type {
                    dt
                } else if let Some(val) = value.clone() {
                    self.analyze_statement(*val)?
                } else {
                    self.errors.push(SematicError::NoTypeOrValueProvided);

                    return None;
                };

                let variable = Variable {
                    name: name.clone(),
                    data_type: var_type.clone(),
                    mutable,
                    used: false,
                    initialized: value.is_some(),
                };

                self.scope.set_variable(name, variable);

                Some(var_type)
            }
            _ => {
                println!("statement: {statement:?}");
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
