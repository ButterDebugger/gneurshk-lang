use crate::codegen::Codegen;
use gneurshk_parser::{BinaryOperator, Stmt};
use inkwell::IntPredicate;
use inkwell::values::BasicValueEnum;

impl<'ctx> Codegen<'ctx> {
    pub(crate) fn compile_binary_expression(
        &mut self,
        left: Stmt,
        right: Stmt,
        operator: BinaryOperator,
    ) -> Option<BasicValueEnum<'ctx>> {
        let left_value = self.compile_stmt(left)?.into_int_value();
        let right_value = self.compile_stmt(right)?.into_int_value();

        let result = match operator {
            BinaryOperator::Add => self
                .builder
                .build_int_add(left_value, right_value, "add")
                .unwrap(),
            BinaryOperator::Subtract => self
                .builder
                .build_int_sub(left_value, right_value, "sub")
                .unwrap(),
            BinaryOperator::Multiply => self
                .builder
                .build_int_mul(left_value, right_value, "mul")
                .unwrap(),
            BinaryOperator::Divide => self
                .builder
                .build_int_signed_div(left_value, right_value, "div")
                .unwrap(),
            BinaryOperator::Modulus => self
                .builder
                .build_int_signed_rem(left_value, right_value, "rem")
                .unwrap(),
            BinaryOperator::GreaterThan => {
                let cmp = self
                    .builder
                    .build_int_compare(IntPredicate::SGT, left_value, right_value, "gt")
                    .unwrap();
                self.builder
                    .build_int_z_extend(cmp, self.context.i32_type(), "gt_ext")
                    .unwrap()
            }
            BinaryOperator::GreaterThanEqual => {
                let cmp = self
                    .builder
                    .build_int_compare(IntPredicate::SGE, left_value, right_value, "gte")
                    .unwrap();
                self.builder
                    .build_int_z_extend(cmp, self.context.i32_type(), "gte_ext")
                    .unwrap()
            }
            BinaryOperator::Equal => {
                let cmp = self
                    .builder
                    .build_int_compare(IntPredicate::EQ, left_value, right_value, "eq")
                    .unwrap();
                self.builder
                    .build_int_z_extend(cmp, self.context.i32_type(), "eq_ext")
                    .unwrap()
            }
            BinaryOperator::NotEqual => {
                let cmp = self
                    .builder
                    .build_int_compare(IntPredicate::NE, left_value, right_value, "ne")
                    .unwrap();
                self.builder
                    .build_int_z_extend(cmp, self.context.i32_type(), "ne_ext")
                    .unwrap()
            }
            BinaryOperator::LessThanEqual => {
                let cmp = self
                    .builder
                    .build_int_compare(IntPredicate::SLE, left_value, right_value, "lte")
                    .unwrap();
                self.builder
                    .build_int_z_extend(cmp, self.context.i32_type(), "lte_ext")
                    .unwrap()
            }
            BinaryOperator::LessThan => {
                let cmp = self
                    .builder
                    .build_int_compare(IntPredicate::SLT, left_value, right_value, "lt")
                    .unwrap();
                self.builder
                    .build_int_z_extend(cmp, self.context.i32_type(), "lt_ext")
                    .unwrap()
            }
            BinaryOperator::And => self
                .builder
                .build_and(left_value, right_value, "and")
                .unwrap(),
            BinaryOperator::Or => self
                .builder
                .build_or(left_value, right_value, "or")
                .unwrap(),
        };

        Some(result.into())
    }
}
