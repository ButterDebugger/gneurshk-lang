use crate::codegen::Codegen;
use gneurshk_parser::{Stmt, UnaryOperator};
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn build_unary_expression(
        &mut self,
        value: Stmt,
        operator: UnaryOperator,
    ) -> Option<BasicValueEnum<'ctx>> {
        let operand = self.build_stmt(value)?;

        match operator {
            UnaryOperator::Not => self.build_not_expression(operand),
            UnaryOperator::Negative => self.build_negative_expression(operand),
        }
    }

    fn build_not_expression(
        &mut self,
        operand: BasicValueEnum<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> {
        match operand {
            BasicValueEnum::IntValue(int_val) => {
                let zero = self.context.i32_type().const_int(0, false);
                let is_zero = self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::EQ, int_val, zero, "is_zero")
                    .unwrap();
                let result = self
                    .builder
                    .build_int_z_extend(is_zero, self.context.i32_type(), "not_result")
                    .unwrap();
                Some(result.into())
            }
            _ => panic!("Unsupported operand type for unary not operator"),
        }
    }

    fn build_negative_expression(
        &mut self,
        operand: BasicValueEnum<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> {
        match operand {
            BasicValueEnum::IntValue(int_val) => {
                let result = self.builder.build_int_neg(int_val, "neg").unwrap();
                Some(result.into())
            }
            BasicValueEnum::FloatValue(float_val) => {
                let result = self.builder.build_float_neg(float_val, "fneg").unwrap();
                Some(result.into())
            }
            _ => panic!("Unsupported operand type for unary minus operator"),
        }
    }
}
