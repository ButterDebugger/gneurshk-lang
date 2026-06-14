use crate::{errors::SematicError, function::FunctionAnalyzer};
use gneurshk_parser::{Expression, MemberExpressionBase, types::DataType};

impl<'a> FunctionAnalyzer<'a> {
    pub(crate) fn analyze_assignment(
        &mut self,
        member: MemberExpressionBase,
        value: Expression,
    ) -> Option<DataType> {
        let value_type = self.analyze_expression(value);

        match member {
            MemberExpressionBase::Identifier(identifier) => {
                let name = identifier.name;

                let variable = match self.scope.get_variable(&name) {
                    Some(v) => v,
                    None => {
                        self.program_analyzer
                            .errors
                            .push(SematicError::VariableNotFound(name));
                        return None;
                    }
                };

                if !variable.mutable {
                    self.program_analyzer
                        .errors
                        .push(SematicError::VariableUnmodifiable(name.clone()));
                }

                if let Some(value_type) = value_type
                    && value_type != variable.data_type
                {
                    self.program_analyzer
                        .errors
                        .push(SematicError::TypeMismatch(
                            variable.data_type.clone(),
                            value_type,
                        ));
                }

                if let Some(var_mut) = self.scope.get_mut_variable(&name) {
                    var_mut.initialized = true;
                }

                Some(variable.data_type)
            }
            MemberExpressionBase::FunctionCall(_) | MemberExpressionBase::MemberAccess(_) => {
                todo!()
            }
        }
    }
}
