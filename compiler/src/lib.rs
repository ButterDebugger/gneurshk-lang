use crate::codegen::Codegen;
use gneurshk_parser::Stmt;
use inkwell::OptimizationLevel;
use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use std::path::Path;
use std::process::Command;

mod codegen;

/// Creates LLVM IR files (.ll) from the AST
///
/// # Returns
/// The path to the LLVM IR file
pub fn create_llvm_ir_file(ast: Vec<Stmt>, output_path: &str) -> Result<String, String> {
    let context = Context::create();
    let mut codegen = Codegen::new(&context, "main");

    codegen.compile(ast);

    // Write LLVM IR to file
    let module = codegen.get_module();
    let ir = module.print_to_string().to_string();
    let ir_path = format!("{}.ll", output_path);

    std::fs::write(&ir_path, ir).map_err(|e| format!("Failed to write LLVM IR file: {}", e))?;

    Ok(ir_path)
}

/// Creates object files (.o) from the AST
///
/// # Returns
/// The path to the object file
pub fn create_object_file(ast: Vec<Stmt>, output_path: &str) -> Result<String, String> {
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
    let obj_path = format!("{}.o", output_path);

    target_machine
        .write_to_file(module, FileType::Object, Path::new(&obj_path))
        .map_err(|e| format!("Failed to write object file: {}", e))?;

    Ok(obj_path)
}

/// Compiles the AST into an executable
pub fn compile_to_executable(ast: Vec<Stmt>, output_path: &str) -> Result<(), String> {
    // First create an object file
    let obj_path = create_object_file(ast, output_path)?;

    // Link the object file to create an executable
    let output = Command::new("gcc")
        .arg(&obj_path)
        .arg("-o")
        .arg(output_path)
        .output()
        .map_err(|e| format!("Failed to run linker: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Linker failed: {}", stderr));
    }

    // Clean up the object file
    std::fs::remove_file(&obj_path)
        .map_err(|e| format!("Failed to clean up object file: {}", e))?;

    Ok(())
}
