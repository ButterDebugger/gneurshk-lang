use crate::codegen::Codegen;
use gneurshk_parser::Program;
use inkwell::OptimizationLevel;
use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use std::path::{Path, PathBuf};

/// Creates object files (.o) from the AST
///
/// # Returns
/// The path to the object file
pub fn create_object_file(ast: Program, output_path: &Path) -> Result<PathBuf, String> {
    let context = Context::create();
    let mut codegen = Codegen::new(&context, "main");

    codegen.compile(ast);

    // Initialize LLVM targets
    Target::initialize_all(&InitializationConfig::default());

    // Get the default target triple
    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple)
        .map_err(|e| format!("Failed to create target: {}", e))?;

    // Create target machine
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .ok_or("Failed to create target machine")?;

    // Write object file
    let module = codegen.get_module();
    let obj_path = output_path.with_extension("o");

    target_machine
        .write_to_file(module, FileType::Object, &obj_path)
        .map_err(|e| format!("Failed to write object file: {}", e))?;

    // Return the path to the object file
    Ok(obj_path)
}
