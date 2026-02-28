use crate::output::object::create_object_file;
use gneurshk_parser::Program;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Compiles the AST into an executable
///
/// # Returns
/// The path to the executable
pub fn compile_to_executable(ast: Program, output_path: &Path) -> Result<PathBuf, String> {
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

    // Add the correct extension for the executable
    #[cfg(windows)]
    let executable_path = output_path.with_extension("exe");
    #[cfg(not(windows))]
    let executable_path = output_path.to_path_buf();

    // Return the path to the executable
    Ok(executable_path)
}

#[cfg(test)]
mod tests {
    use crate::output::executable::compile_to_executable;
    use std::path::PathBuf;

    fn compile_and_run(source: &str, output_name: &str) -> Result<String, String> {
        let output_path = PathBuf::from(format!("tests/{}", output_name));
        let output_path = output_path.as_path();

        // Create parent directory if it doesn't exist
        std::fs::create_dir_all(output_path.parent().unwrap())
            .map_err(|e| format!("Failed to create parent directory: {}", e))?;

        // Compile the source code to an executable
        let executable_path = compile_to_executable(
            gneurshk_parser::parse(&mut gneurshk_lexer::lex(source).unwrap()).unwrap(),
            output_path,
        )?;

        // Run the executable
        let path = std::path::absolute(&executable_path).unwrap();

        let output = std::process::Command::new(&path)
            .output()
            .map_err(|e| format!("Failed to run executable: {}", e))?;

        // Return an error if the executable failed
        if !output.status.success() {
            return Err(format!("Executable failed with status: {}", output.status));
        }

        // Return the output
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    #[test]
    fn hello_world() {
        let source = r#"println("Hello, World!")"#;

        let output = compile_and_run(source, "hello_world").unwrap();

        assert_eq!(output.trim(), "Hello, World!");
    }

    #[test]
    fn arithmetic() {
        let source = r#"println(1 + 3 * 4)"#;

        let output = compile_and_run(source, "arithmetic").unwrap();

        assert_eq!(output.trim(), "13");
    }

    #[test]
    fn add_two_numbers_with_function() {
        let source = r#"
            func add(a: Int32, b: Int32) -> Int32 {
                return a + b
            }

            println(add(2, 3))
        "#;

        let output = compile_and_run(source, "add_two_numbers_with_function").unwrap();

        assert_eq!(output.trim(), "5");
    }

    #[test]
    fn multiple_println_statements() {
        let source = r#"
            println(1)
            println(2)
            println(3)
        "#;

        let output = compile_and_run(source, "multiple_println_statements").unwrap();

        assert_eq!(output.trim(), "1\r\n2\r\n3");
    }

    #[test]
    fn print_multiple_values() {
        let source = r#"println(1, 2, 3)"#;

        let output = compile_and_run(source, "print_multiple_values").unwrap();

        assert_eq!(output.trim(), "1 2 3");
    }

    #[test]
    fn if_statement() {
        let source = r#"
            if true {
                println("if")
            }
        "#;

        let output = compile_and_run(source, "if_statement").unwrap();

        assert_eq!(output.trim(), "if");
    }

    #[test]
    fn if_else_statement() {
        let source = r#"
            if true {
                println("if")
            } else {
                println("else")
            }
        "#;

        let output = compile_and_run(source, "if_else_statement").unwrap();

        assert_eq!(output.trim(), "if");
    }

    #[test]
    fn pass_if_finally_statement() {
        let source = r#"
            if false {
                println("if")
            }

            println("finally")
        "#;

        let output = compile_and_run(source, "pass_if_finally_statement").unwrap();

        assert_eq!(output.trim(), "finally");
    }

    #[test]
    fn if_finally_statement() {
        let source = r#"
            if true {
                println("if")
            }

            println("finally")
        "#;

        let output = compile_and_run(source, "if_finally_statement").unwrap();

        assert_eq!(output.trim(), "if\r\nfinally");
    }

    #[test]
    fn if_else_finally_statement() {
        let source = r#"
            if false {
                println("if")
            } else {
                println("else")
            }

            println("finally")
        "#;

        let output = compile_and_run(source, "if_else_finally_statement").unwrap();

        assert_eq!(output.trim(), "else\r\nfinally");
    }

    #[test]
    fn fibonacci() {
        let source = r#"
            func fib(n: Int32) -> Int32 {
                if n <= 1 {
                    return n
                }

                return fib(n - 1) + fib(n - 2)
            }

            println(fib(42))
        "#;

        let output = compile_and_run(source, "fibonacci").unwrap();

        assert_eq!(output.trim(), "267914296");
    }

    #[test]
    fn factorial() {
        let source = r#"
            func factorial(n: Int32) -> Int32 {
                if n == 0 {
                    return 1
                }

                return n * factorial(n - 1)
            }

            println(factorial(12))
        "#;

        let output = compile_and_run(source, "factorial").unwrap();

        assert_eq!(output.trim(), "479001600");
    }
}
