use parser::Stmt;
mod lexer;
mod parser;

fn main() {
    // Read the input from the command line
    // let input = env::args()
    //     .nth(1)
    //     .expect("Expected expression argument (e.g. `1 + 7 * (3 - 4) / 5`)");

    // Compile the input
    build("1 + 7 * (3 - 4) / 5");
}

fn build(input: &str) -> Vec<Stmt> {
    // Create a iterable list of tokens
    let tokens = lexer::lex(&input);

    println!("Tokens: {:#?}", tokens);

    // Parse the tokens to construct an AST
    let ast = match parser::parse(&mut tokens.iter().peekable().clone()) {
        Ok(result) => result,
        Err(e) => panic!("Parse error: {}", e),
    };

    println!("AST {:#?}", ast);

    ast
}
