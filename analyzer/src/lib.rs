use core::panic;
use gneurshk_parser::{Program, Stmt, types::DataType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct State {
    functions: HashMap<String, DataType>,
}

pub fn analyze(program: Program) -> Result<(), String> {
    let mut state = Box::new(State {
        functions: HashMap::new(),
    });

    for function in program.functions {
        match function {
            Stmt::FunctionDeclaration {
                name,
                params: _,
                return_type,
                block: _,
            } => {
                state.functions.insert(name, return_type);
            }
            _ => return Err("Expected function declaration".to_string()),
        }
    }

    for statement in program.body {
        match analyze_statement(statement, &mut state) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn analyze_statement(statement: Stmt, state: &mut Box<State>) -> Result<DataType, String> {
    match statement {
        Stmt::BinaryExpression {
            left,
            operator: _,
            right,
        } => {
            let left_type = analyze_statement(*left, state)?;
            let right_type = analyze_statement(*right, state)?;

            if left_type != right_type {
                panic!("Type mismatch: {:?} != {:?}", left_type, right_type);
            }

            Ok(left_type)
        }
        Stmt::String { value: _ } => Ok(DataType::String),
        Stmt::Integer { value: _ } => Ok(DataType::Int32),
        Stmt::Float { value: _ } => Ok(DataType::Float32),
        Stmt::Boolean { value: _ } => Ok(DataType::Boolean),
        Stmt::FunctionCall { name, args: _ } => {
            if let Some(function_type) = state.functions.get(&name) {
                Ok(function_type.clone())
            } else {
                Err(format!("Function {} not found", name))
            }
        }
        _ => {
            todo!();
        }
    }
}
