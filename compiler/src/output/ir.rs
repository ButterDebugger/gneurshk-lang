use crate::codegen::Codegen;
use gneurshk_parser::Program;
use inkwell::context::Context;

/// Creates LLVM IR files (.ll) from the AST
///
/// # Returns
/// The path to the LLVM IR file
pub fn create_llvm_ir_file(ast: Program, output_path: &str) -> Result<String, String> {
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
