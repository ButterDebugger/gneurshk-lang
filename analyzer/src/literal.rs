use crate::Analyzer;
use gneurshk_parser::types::DataType;

impl Analyzer {
    pub(crate) fn analyze_string(&mut self) -> Option<DataType> {
        Some(DataType::String)
    }

    pub(crate) fn analyze_integer(&mut self) -> Option<DataType> {
        Some(DataType::Int32)
    }

    pub(crate) fn analyze_float(&mut self) -> Option<DataType> {
        Some(DataType::Float32)
    }

    pub(crate) fn analyze_boolean(&mut self) -> Option<DataType> {
        Some(DataType::Boolean)
    }
}
