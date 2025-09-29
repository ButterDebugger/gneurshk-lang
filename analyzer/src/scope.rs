use gneurshk_parser::{FunctionParam, types::DataType};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Scope {
    parent: Option<Box<Scope>>,

    variables: HashMap<String, Variable>,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub(crate) name: String,
    pub(crate) data_type: DataType,
    pub(crate) mutable: bool,
    pub(crate) used: bool,
    pub(crate) initialized: bool,
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

    pub fn get_mut_variable(&mut self, name: &String) -> Option<&mut Variable> {
        self.variables.get_mut(name).or_else(|| {
            self.parent
                .as_mut()
                .and_then(|parent| parent.get_mut_variable(name))
        })
    }

    pub fn get_unused_variables(&self) -> Vec<Variable> {
        let mut unused = self
            .variables
            .iter()
            .filter(|(_, variable)| !variable.used)
            .map(|(_, var)| var.clone())
            .collect::<Vec<Variable>>();

        if let Some(parent) = self.parent.as_ref() {
            unused.extend(parent.get_unused_variables());
        }

        unused
    }
}
