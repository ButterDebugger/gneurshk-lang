use anyhow::{Result, anyhow};
use gneurshk_analyzer::program::{AnalyzedProgram, ProgramAnalyzer};
use gneurshk_compiler::output::{executable::compile_to_executable, ir::create_llvm_ir_file};
use gneurshk_lexer::{TokenStream, lex};
use gneurshk_parser::{Program, parse};
use indicatif::ProgressBar;
use std::path::PathBuf;

#[allow(clippy::boxed_local)]
pub(crate) fn tokenize(source: &str, pb: Box<ProgressBar>) -> Result<TokenStream<'_>> {
    // Create a iterable list of tokens
    pb.set_message("Tokenizing...");

    lex(source)
}

pub(crate) fn create_ast(source: &str, pb: Box<ProgressBar>) -> Result<Program> {
    // Tokenize the input
    let tokens = tokenize(source, pb.clone())?;

    // Parse the tokens to construct an AST
    pb.set_message("Parsing...");

    parse(&mut tokens.clone())
}

pub(crate) fn analyze_program(
    source: &str,
    pb: Box<ProgressBar>,
) -> Result<(Program, AnalyzedProgram)> {
    // Create the AST
    let ast = create_ast(source, pb.clone())?;

    // Analyze the AST
    pb.set_message("Analyzing...");

    let analyzed_program = ProgramAnalyzer::analyze(ast.clone());

    Ok((ast, analyzed_program))
}

pub(crate) fn build(source: &str, output_ir: bool, pb: Box<ProgressBar>) -> Result<PathBuf> {
    // Analyze the program
    let ast = match analyze_program(source, pb.clone()) {
        Ok((ast, analyzed)) => {
            // Cancel the build if there are any semantic errors
            let all_errors = analyzed.get_all_errors();

            if !all_errors.is_empty() {
                return Err(anyhow!("{:?}", all_errors));
            }

            // Print the warnings
            for warning in analyzed.warnings {
                pb.println(format!("Warning: {warning}"));
            }

            // Return the AST
            ast
        }
        Err(e) => {
            return Err(e);
        }
    };

    // Create the LLVM IR file
    if output_ir {
        pb.set_message("Creating LLVM IR file...");

        create_llvm_ir_file(ast.clone(), "output".as_ref())?;
    }

    // Create the executable
    pb.set_message("Compiling to executable...");

    let executable_path = compile_to_executable(ast.clone(), "output".as_ref())?;

    Ok(executable_path)
}
