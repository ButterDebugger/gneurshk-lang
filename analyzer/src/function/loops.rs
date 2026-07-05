use crate::function::{FunctionAnalyzer, LoopContext};
use gneurshk_parser::{LoopStmt, types::DataType};

// TODO: If the loop never ends,
//   return an warning that any code after the loop is unreachable
//   or if the function the loop is inside has a return type,
//     return an error that the function will never return

// TODO: Add warnings for code unreachable after a terminator (break/continue/return)

impl<'a> FunctionAnalyzer<'a> {
    pub(crate) fn analyze_loop(&mut self, loop_stmt: LoopStmt) -> Option<DataType> {
        self.loop_stack.push(LoopContext {});

        self.analyze_block(*loop_stmt.block);

        self.loop_stack.pop();

        None // NOTE: Loops will have a return type in the future
    }
}
