use crate::output::object::create_object_file;
use gneurshk_parser::Program;
use std::process::Command;

/// Compiles the AST into an executable
///
/// # Returns
/// The path to the executable
pub fn compile_to_executable(ast: Program, output_path: &str) -> Result<String, String> {
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

    // Return the path to the executable
    #[cfg(windows)]
    let executable_path = format!("{}.exe", output_path);
    #[cfg(not(windows))]
    let executable_path = output_path.to_string();

    Ok(executable_path)
}
