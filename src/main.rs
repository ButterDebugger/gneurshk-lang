use parser::Stmt;
mod parser;
mod tokenize;

fn main() {
    // Read the input from the command line
    // let input = env::args()
    //     .nth(1)
    //     .expect("Expected expression argument (e.g. `1 + 7 * (3 - 4) / 5`)");

    // Compile the input
    compile("var e = 2\n\n");
}

fn compile(input: &str) -> Vec<Stmt> {
    // Create a iterable list of tokens
    let tokens = tokenize::lex(&input);

    // Parse the tokens to construct an AST
    let ast = match parser::parse(&mut tokens.iter().peekable().clone()) {
        Ok(result) => {
            println!("AST {:#?}", result);
            result
        }
        Err(e) => panic!("Parse error: {}", e),
    };

    ast
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_variable_declaration() {
        compile("var var extra_extra = 0");
    }

    #[test]
    #[should_panic]
    fn unfinished_variable_declaration() {
        compile("var");
    }

    #[test]
    #[should_panic]
    fn unfinished_constant_declaration() {
        compile("const");
    }

    #[test]
    fn blank_variable_declaration() {
        compile("var apple");
    }

    #[test]
    fn blank_constant_declaration() {
        compile("const apple");
    }

    #[test]
    fn literal_variable_declaration() {
        compile("var green_beans = 2");
    }

    #[test]
    fn expression_variable_declaration() {
        compile("var green_beans = 2 + 5");
    }
}
