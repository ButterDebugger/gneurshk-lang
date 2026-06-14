use crate::{errors::SematicError, function::FunctionAnalyzer};
use gneurshk_parser::types::DataType;

impl<'a> FunctionAnalyzer<'a> {
    pub(crate) fn analyze_identifier(&mut self, name: String) -> Option<DataType> {
        if let Some(variable) = self.scope.get_mut_variable(&name) {
            variable.used = true;

            Some(variable.data_type.clone())
        } else {
            self.program_analyzer.errors.push(SematicError::VariableNotFound(name));

            None
        }
    }
}
