use crate::{errors::SematicWarning, function::FunctionAnalyzer};
use gneurshk_parser::{Block, types::DataType};

impl<'a> FunctionAnalyzer<'a> {
    pub(crate) fn analyze_block(&mut self, block: Block) -> Option<DataType> {
        self.enter_new_scope();

        let mut last_value = None;
        for stmt in block.body {
            last_value = self.analyze_statement(stmt);
        }

        // Check for unused variables before exiting the scope
        for variable in self.scope.get_unused_variables() {
            self.warnings
                .push(SematicWarning::UnusedVariable(variable.name));
        }

        self.exit_scope();

        last_value
    }
}
