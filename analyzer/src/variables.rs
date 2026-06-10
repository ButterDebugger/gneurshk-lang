use crate::{Analyzer, errors::SematicError, scope::Variable};
use gneurshk_parser::{VariableDeclaration, types::DataType};

impl Analyzer {
    pub(crate) fn analyze_variable_declaration(
        &mut self,
        variable: VariableDeclaration,
    ) -> Option<DataType> {
        // Get values from the variable declaration
        let (mutable, name, data_type, value) = match variable {
            VariableDeclaration::Mutable {
                name,
                value,
                data_type,
                ..
            } => (true, name, data_type, value),
            VariableDeclaration::Constant {
                name,
                value,
                data_type,
                ..
            } => (false, name, data_type, Some(value)),
        };

        // Analyze data type
        let var_type = if let Some(dt) = data_type {
            dt
        } else if let Some(val) = value.clone() {
            self.analyze_expression(val)?
        } else {
            self.errors.push(SematicError::NoTypeOrValueProvided);

            return None;
        };

        // Store variable in scope
        let variable = Variable {
            name: name.clone(),
            data_type: var_type.clone(),
            mutable,
            used: false,
            initialized: value.is_some(),
        };

        self.scope.set_variable(name, variable);

        None
    }
}
