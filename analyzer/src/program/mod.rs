use crate::{
    errors::{SematicError, SematicWarning},
    function::{AnalyzedFunction, FunctionAnalyzer},
    scope::{Function, Scope},
};
use gneurshk_parser::{FunctionDeclaration, Program};
use std::collections::HashMap;

#[derive(Debug)]
pub struct AnalyzedProgram {
    pub analyzed_functions: Vec<AnalyzedFunction>,

    pub errors: Vec<SematicError>,
    pub warnings: Vec<SematicWarning>,
}

impl AnalyzedProgram {
    pub fn get_all_errors(&self) -> Vec<SematicError> {
        let mut errors = self.errors.clone();

        for function in &self.analyzed_functions {
            errors.append(&mut function.errors.clone());
        }

        errors
    }

    pub fn get_all_warnings(&self) -> Vec<SematicWarning> {
        let mut warnings = self.warnings.clone();

        for function in &self.analyzed_functions {
            warnings.append(&mut function.warnings.clone());
        }

        warnings
    }
}

#[derive(Debug)]
pub struct ProgramAnalyzer {
    pub(crate) scope: Box<Scope>,
    pub(crate) functions: HashMap<String, Function>,

    pub(crate) errors: Vec<SematicError>,
    pub(crate) warnings: Vec<SematicWarning>,
}

impl ProgramAnalyzer {
    pub fn analyze(program: Program) -> AnalyzedProgram {
        // Create an analyzer instance
        let mut analyzer = ProgramAnalyzer {
            scope: Box::new(Scope::new(None)),
            functions: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Register all function signatures
        for function in program.functions.clone() {
            let FunctionDeclaration {
                name,
                params,
                return_type,
                ..
            } = function;

            analyzer.functions.insert(
                name,
                Function {
                    return_type,
                    params,
                },
            );
        }

        // Analyze each function
        let analyzed_functions: Vec<AnalyzedFunction> = program
            .functions
            .into_iter()
            .map(|function| FunctionAnalyzer::analyze(&mut analyzer, function))
            .collect();

        // Check for unused variables in the global scope
        for variable in analyzer.scope.get_unused_variables() {
            analyzer
                .warnings
                .push(SematicWarning::UnusedVariable(variable.name));
        }

        // Return a static analyzed program
        AnalyzedProgram {
            analyzed_functions,
            errors: analyzer.errors,
            warnings: analyzer.warnings,
        }
    }
}
