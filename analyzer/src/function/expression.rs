use crate::function::FunctionAnalyzer;
use gneurshk_parser::{BinaryExpression, Expression, FunctionCall, Identifier, types::DataType};

impl<'a> FunctionAnalyzer<'a> {
    pub(crate) fn analyze_expression(&mut self, expr: Expression) -> Option<DataType> {
        match expr {
            Expression::BinaryExpression(BinaryExpression {
                left,
                right,
                operator,
            }) => self.analyze_binary_expression(*left, *right, operator),
            Expression::String(..) => self.analyze_string(),
            Expression::Integer(..) => self.analyze_integer(),
            Expression::Float(..) => self.analyze_float(),
            Expression::Boolean(..) => self.analyze_boolean(),
            Expression::Identifier(Identifier { name, .. }) => self.analyze_identifier(name),
            Expression::FunctionCall(FunctionCall { name, args, .. }) => {
                self.analyze_function_call(name, args)
            }
            Expression::UnaryExpression(unary_expr) => self.analyze_unary_expression(unary_expr),
            _ => {
                println!("expression: {expr:?}");

                todo!();
            }
        }
    }
}
