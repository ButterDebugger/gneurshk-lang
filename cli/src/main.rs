use colored::Colorize;
use gneurshk_compiler::compile;
use gneurshk_lexer::lex;
use gneurshk_parser::{Stmt, parse};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{env::args, fs::read_to_string, path::Path};

fn main() {
    // Read the input from the command line
    match args().nth(1) {
        Some(input) => match input.to_lowercase().as_str() {
            "run" => {
                todo!();
            }
            "build" => {
                let path = args().nth(2).expect("Argument 2 needs to be a path");
                let path: &Path = path.as_ref();
                let source = read_to_string(path).expect("Failed to read file");

                build_cmd(&source);
            }
            "lex" => {
                let input = args().nth(2).expect("Argument 2 needs to be a string");
                let path = Path::new(&input);
                let source = read_to_string(path).expect("Failed to read file");

                match lex(&source) {
                    Ok(tokens) => {
                        for (token, range) in tokens {
                            println!("{}..{}\t{:?}", range.start, range.end, token);
                        }
                    }
                    Err(e) => println!("Error: {e}"),
                }
            }
            "parse" => {
                let input = args().nth(2).expect("Argument 2 needs to be a string");
                let path = Path::new(&input);
                let source = read_to_string(path).expect("Failed to read file");

                match create_ast(&source) {
                    Ok(ast) => println!("AST: {ast:#?}"),
                    Err(e) => println!("Error: {e}"),
                }
            }
            "check" => {
                let path = args().nth(2).expect("Argument 2 needs to be a path");
                let path: &Path = path.as_ref();

                if let Err(error) = check_cmd(path) {
                    println!("Error: {error:?}");
                }
            }
            "help" => help_cmd(),
            _ => println!("Unknown command: {input}"),
        },
        None => help_cmd(),
    };
}

fn help_cmd() {
    // Print header message
    println!(
        "{} is an awesome programming language",
        "Gneurshk".bright_magenta()
    );
    println!();

    // Print command usage
    println!("Usage: gneurshk [<flags>] <command>");
    println!();

    // Print list of commands
    println!("Commands:");
    println!(
        "  {}    {}  Builds and runs a file",
        "run".blue(),
        "<file path>".dimmed()
    );
    println!(
        "  {}  {}  Compiles a file into an executable",
        "build".blue(),
        "<file path>".dimmed()
    );
    println!(
        "  {}    {}  Lexes a file and prints the tokens",
        "lex".blue(),
        "<file path>".dimmed()
    );
    println!(
        "  {}  {}  Parses a file and prints the AST",
        "parse".blue(),
        "<file path>".dimmed()
    );
    println!(
        "  {}  {}  Watches a file for changes and checks code validity",
        "check".blue(),
        "<file path>".dimmed()
    );
    println!("  {}                Prints a help message", "help".blue());
}

fn build_cmd(input: &str) {
    let ast = match create_ast(input) {
        Ok(ast) => ast,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };

    compile(ast);
}

fn check_cmd(path: &Path) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    fn check(path: &Path) {
        let source = read_to_string(path).expect("Failed to read file");

        match create_ast(&source) {
            Ok(_ast) => println!("✅"),
            Err(error) => println!("❌ Error: {error:?}"),
        }
    }

    let mut watcher = RecommendedWatcher::new(tx, Config::default().with_compare_contents(true))?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    println!("{} Process has started.", "Watcher".bright_green());

    check(path);

    for res in rx {
        // Clear the screen
        clearscreen::clear().unwrap();

        match res {
            Ok(event) => {
                // Print a restarting message
                if let Some(path) = event.paths.first() {
                    println!(
                        "{} Restarting! File change detected: \"{}\"",
                        "Watcher".bright_green(),
                        path.display()
                    );
                } else {
                    println!(
                        "{} Restarting! File change detected",
                        "Watcher".bright_green()
                    );
                }

                // Restart the process
                check(path);

                // Once the process has finished, print a finishing message
                println!(
                    "{} Process has finished. Restarting on file change...",
                    "Watcher".bright_green()
                );
            }
            Err(error) => println!(
                "{} Encountered an error: {}",
                "Watcher".bright_green(),
                error
            ),
        }
    }

    Ok(())
}

fn create_ast(input: &str) -> Result<Vec<Stmt>, String> {
    // Create a iterable list of tokens
    let tokens = lex(input)?;

    // println!(
    //     "Tokens: {:#?}",
    //     tokens.clone().map(|(token, _)| token).collect::<Vec<_>>()
    // );

    // Parse the tokens to construct an AST
    let ast = match parse(&mut tokens.clone()) {
        Ok(result) => result,
        Err(e) => return Err(e.to_owned()),
    };

    // println!("AST {:#?}", ast);

    Ok(ast)
}
