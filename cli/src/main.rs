use colored::Colorize;
use gneurshk_lexer::lex;
use gneurshk_parser::{Stmt, parse};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{env, path::Path};

fn main() {
    // Read the input from the command line
    match env::args().nth(1) {
        Some(input) => match input.to_lowercase().as_str() {
            "parse" => {
                let input = std::env::args()
                    .nth(2)
                    .expect("Argument 2 needs to be a string");
                let path = Path::new(&input);
                let source = std::fs::read_to_string(path).expect("Failed to read file");

                match create_ast(&source) {
                    Ok(ast) => println!("AST: {ast:#?}"),
                    Err(e) => println!("Error: {e}"),
                }
            }
            "check" => {
                let path = std::env::args()
                    .nth(2)
                    .expect("Argument 2 needs to be a path");
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

    // Compile the input
    // build("1 + 7 * (3 - 4) / 5");
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

fn check_cmd(path: &Path) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    fn check(path: &Path) {
        let source = std::fs::read_to_string(path).expect("Failed to read file");

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
