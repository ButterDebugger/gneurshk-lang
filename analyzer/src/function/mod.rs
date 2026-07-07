use crate::{
    errors::{SematicError, SematicWarning},
    program::ProgramAnalyzer,
    scope::{Scope, Variable},
};
use gneurshk_parser::FunctionDeclaration;

mod assignment;
mod binary_expression;
mod block;
mod expression;
mod function_call;
mod identifier;
mod ifs;
mod literal;
mod loops;
mod returns;
mod statement;
mod unary_expression;
mod variables;

#[derive(Debug)]
pub struct AnalyzedFunction {
    pub errors: Vec<SematicError>,
    pub warnings: Vec<SematicWarning>,
}

#[derive(Debug)]
pub(crate) struct LoopContext {}

#[derive(Debug)]
pub(crate) struct FunctionAnalyzer<'a> {
    pub(crate) scope: Box<Scope>,
    pub(crate) program_analyzer: &'a mut ProgramAnalyzer,
    pub(crate) function_declaration: FunctionDeclaration,
    pub(crate) loop_stack: Vec<LoopContext>,

    pub(crate) errors: Vec<SematicError>,
    pub(crate) warnings: Vec<SematicWarning>,
}

impl<'a> FunctionAnalyzer<'a> {
    pub fn analyze(
        program_analyzer: &'a mut ProgramAnalyzer,
        function: FunctionDeclaration,
    ) -> AnalyzedFunction {
        // Create an analyzer instance
        let mut analyzer = FunctionAnalyzer {
            scope: Box::new(Scope::new(None)),
            // scope: Box::new(Scope::new(Some(program_analyzer.scope))),
            program_analyzer,
            function_declaration: function.clone(),
            errors: Vec::new(),
            warnings: Vec::new(),
            loop_stack: Vec::new(),
        };

        // Declare the params in the scope
        for param in function.params {
            analyzer.scope.set_variable(
                param.name.clone(),
                Variable {
                    name: param.name,
                    data_type: param.data_type,
                    mutable: param.mutable,
                    used: false,
                    initialized: true,
                },
            );
        }

        // Analyze function body
        let implicit_return = analyzer.analyze_block(*function.block);

        // Check if the implicit return doesn't match the expected return type
        if implicit_return != function.return_type {
            analyzer
                .errors
                .push(SematicError::FunctionReturnTypeMismatch(function.name));
        }

        // Check for unused variables before exiting the scope
        for variable in analyzer.scope.get_unused_variables() {
            analyzer
                .warnings
                .push(SematicWarning::UnusedVariable(variable.name));
        }

        // Return a static analyzed function
        AnalyzedFunction {
            errors: analyzer.errors,
            warnings: analyzer.warnings,
        }
    }
}

impl<'a> FunctionAnalyzer<'a> {
    pub fn enter_new_scope(&mut self) {
        let parent = self.scope.to_owned();
        let new_scope = Scope::new(Some(parent));

        *self.scope = new_scope;
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scope.parent.to_owned() {
            self.scope = parent;
        }
    }
}
