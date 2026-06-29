use crate::codegen::Codegen;
use gneurshk_parser::{Expression, MemberExpressionBase};
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_assignment(
        &mut self,
        member: MemberExpressionBase,
        value: Expression,
    ) -> Option<BasicValueEnum<'ctx>> {
        match member {
            MemberExpressionBase::Identifier(identifier) => {
                let name = identifier.name;

                let new_value = self.build_expression(value)?;
                let pointer = self.scope.get_variable(&name)?.pointer;

                self.builder.build_store(pointer, new_value).unwrap();

                Some(new_value)
            }
            MemberExpressionBase::FunctionCall(_) | MemberExpressionBase::MemberAccess(_) => {
                todo!()
            }
        }
    }
}
