use crate::{Analyzer, errors::SematicError, scope::Variable};
use gneurshk_parser::{Expression, types::DataType};

impl Analyzer {
    pub(crate) fn analyze_declaration(
        &mut self,
        mutable: bool,
        name: String,
        data_type: Option<DataType>,
        value: Option<Expression>,
    ) -> Option<DataType> {
        let var_type = if let Some(dt) = data_type {
            dt
        } else if let Some(val) = value.clone() {
            self.analyze_expression(val)?
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
}
