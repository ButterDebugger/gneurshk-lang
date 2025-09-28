use gneurshk_parser::{FunctionParam, types::DataType};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Scope {
    parent: Option<Box<Scope>>,

    variables: HashMap<String, Variable>,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub(crate) data_type: DataType,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub(crate) return_type: DataType,
    pub(crate) params: Vec<FunctionParam>,
}

impl Scope {
    pub fn new(parent: Option<Box<Scope>>) -> Self {
        Self {
            parent,

            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, variable: Variable) {
        self.variables.insert(name, variable);
    }

    pub fn get_variable(&self, name: &String) -> Option<Variable> {
        self.variables.get(name).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_variable(name))
        })
    }
}
