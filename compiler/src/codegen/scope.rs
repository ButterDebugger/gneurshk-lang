use crate::codegen::Codegen;
use inkwell::values::{FunctionValue, PointerValue};
use std::collections::HashMap;
use std::convert::AsRef;

#[derive(Clone, Debug)]
pub struct Scope<'ctx> {
    parent: Option<Box<Scope<'ctx>>>,

    variables: HashMap<String, PointerValue<'ctx>>,
    functions: HashMap<String, FunctionValue<'ctx>>,
}

impl<'ctx> Scope<'ctx> {
    pub fn new(parent: Option<Box<Scope<'ctx>>>) -> Self {
        Self {
            parent,

            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, id: impl AsRef<str>, pointer: PointerValue<'ctx>) {
        self.variables.insert(id.as_ref().into(), pointer);
    }

    pub fn get_variable(&self, id: impl AsRef<str>) -> Option<PointerValue<'ctx>> {
        self.variables.get(id.as_ref()).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_variable(id))
        })
    }

    pub fn set_function(&mut self, id: impl AsRef<str>, function: FunctionValue<'ctx>) {
        self.functions.insert(id.as_ref().into(), function);
    }

    pub fn get_function(&self, id: impl AsRef<str>) -> Option<FunctionValue<'ctx>> {
        self.functions.get(id.as_ref()).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_function(id))
        })
    }
}

impl<'ctx> Codegen<'ctx> {
    pub fn enter_new_scope(&mut self) {
        let parent = self.scope.to_owned();
        let new_scope = Scope::new(Some(parent));

        self.scope = Box::new(new_scope);
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scope.parent.to_owned() {
            self.scope = parent;
        }
    }
}
