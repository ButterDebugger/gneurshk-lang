use crate::codegen::Codegen;
use anyhow::{Result, anyhow};
use gneurshk_parser::Program;
use inkwell::OptimizationLevel;
use inkwell::context::Context;
#[cfg(windows)]
use inkwell::targets::TargetTriple;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target};
use std::path::{Path, PathBuf};

/// Creates object files (.o) from the AST
///
/// # Returns
/// The path to the object file
pub fn create_object_file(ast: Program, output_path: &Path) -> Result<PathBuf> {
    let context = Context::create();
    let mut codegen = Codegen::new(&context, "main");

    codegen.compile(ast)?;

    // Initialize LLVM targets
    Target::initialize_all(&InitializationConfig::default());

    // Get the appropriate target triple
    let target_triple = get_host_target_triple();
    let target = Target::from_triple(&target_triple)
        .map_err(|e| anyhow!("Failed to create target: {}", e))?;

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
        .ok_or(anyhow!("Failed to create target machine"))?;

    // Write object file
    let module = codegen.get_module();
    let obj_path = output_path.with_extension("o");

    target_machine
        .write_to_file(module, FileType::Object, &obj_path)
        .map_err(|e| anyhow!("Failed to write object file: {}", e))?;

    // Return the path to the object file
    Ok(obj_path)
}

fn get_host_target_triple() -> TargetTriple {
    use std::env::consts::{ARCH, OS};

    // NOTE: ideally I should be using `target_lexicon::HOST` but i've hardcoded using gcc, so we can't do that yet

    let triple = match (OS, ARCH) {
        ("windows", "x86_64") => "x86_64-pc-windows-gnu",
        // TODO: test theses other platforms
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        (os, arch) => panic!("Unsupported host platform: {os}-{arch}"),
    };

    TargetTriple::create(triple)
}
