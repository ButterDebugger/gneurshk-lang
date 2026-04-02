use crate::Analyzer;
use gneurshk_parser::{Literal, types::DataType};

impl Analyzer {
    pub(crate) fn analyze_literal(&mut self, literal: Literal) -> Option<DataType> {
        match literal {
            Literal::String(_) => Some(DataType::String),
            Literal::Integer(_) => Some(DataType::Int32),
            Literal::Float(_) => Some(DataType::Float32),
            Literal::Boolean(_) => Some(DataType::Boolean),
        }
    }
}
