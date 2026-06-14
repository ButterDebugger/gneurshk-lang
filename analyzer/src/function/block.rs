use crate::function::FunctionAnalyzer;
use gneurshk_parser::{Block, types::DataType};

impl<'a> FunctionAnalyzer<'a> {
    pub(crate) fn analyze_block(&mut self, block: Block) -> Option<DataType> {
        self.enter_new_scope();

        let mut last_value = None;
        for stmt in block.body {
            last_value = self.analyze_statement(stmt);
        }

        self.exit_scope();

        last_value
    }
}
