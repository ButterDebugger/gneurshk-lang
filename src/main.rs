use parser::Stmt;
mod lexer;
mod parser;
mod tokens;

fn main() {
    // Read the input from the command line
    // let input = env::args()
    //     .nth(1)
    //     .expect("Expected expression argument (e.g. `1 + 7 * (3 - 4) / 5`)");

    // Compile the input
    compile("var green_beans = 2 + 5");
}

fn compile(input: &str) -> Vec<Stmt> {
    // Create a iterable list of tokens
    let tokens = lexer::lex(&input);

    println!("Tokens: {:#?}", tokens);

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

    #[test]
    fn large_indented_block() {
        compile(
            r"
if 10 + 10:
    var apple = 2








    var green = 5

var borg = 5
",
        );
    }
}
