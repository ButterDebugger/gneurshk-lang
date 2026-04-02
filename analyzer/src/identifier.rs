use crate::{Analyzer, errors::SematicError};
use gneurshk_parser::types::DataType;

impl Analyzer {
    pub(crate) fn analyze_identifier(&mut self, name: String) -> Option<DataType> {
        if let Some(variable) = self.scope.get_mut_variable(&name) {
            variable.used = true;

            Some(variable.data_type.clone())
        } else {
            self.errors.push(SematicError::VariableNotFound(name));

            None
        }
    }
}
