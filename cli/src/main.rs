use colored::Colorize;
use gneurshk_analyzer::Analyzer;
use gneurshk_compiler::{compile_to_executable, create_llvm_ir_file};
use gneurshk_lexer::{TokenStream, lex};
use gneurshk_parser::{Program, parse};
use indicatif::{ProgressBar, ProgressStyle};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{env::args, fs::read_to_string, path::Path, time::Duration};

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

                let pb = create_progress_bar();

                match build_cmd(&source, pb.clone()) {
                    Ok(_) => {
                        pb.finish_with_message("Successfully built executable");
                    }
                    Err(e) => {
                        pb.finish_and_clear();

                        println!("Error: {e}");
                    }
                };
            }
            "lex" => {
                let input = args().nth(2).expect("Argument 2 needs to be a string");
                let path = Path::new(&input);
                let source = read_to_string(path).expect("Failed to read file");

                let pb = create_progress_bar();

                match tokenize(&source, pb.clone()) {
                    Ok(tokens) => {
                        pb.finish_with_message("Finished lexing");

                        for (token, range) in tokens {
                            println!("{}..{}\t{:?}", range.start, range.end, token);
                        }
                    }
                    Err(e) => {
                        pb.finish_and_clear();

                        println!("Error: {e}")
                    }
                }
            }
            "parse" => {
                let input = args().nth(2).expect("Argument 2 needs to be a string");
                let path = Path::new(&input);
                let source = read_to_string(path).expect("Failed to read file");

                let pb = create_progress_bar();

                match create_ast(&source, pb.clone()) {
                    Ok(ast) => {
                        pb.finish_with_message("Finished parsing");

                        println!("AST: {ast:#?}")
                    }
                    Err(e) => {
                        pb.finish_and_clear();

                        println!("Error: {e}")
                    }
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

fn build_cmd(input: &str, pb: Box<ProgressBar>) -> Result<(), String> {
    let ast = match analyze_program(input, pb.clone()) {
        Ok((ast, analyzer)) => {
            // Cancel the build if there are any semantic errors
            if !analyzer.errors.is_empty() {
                return Err(format!("{:?}", analyzer.errors));
            }

            // TODO: Print warnings if there are any

            // Return the AST
            ast
        }
        Err(e) => {
            return Err(e);
        }
    };

    pb.set_message("Creating LLVM IR file...");

    create_llvm_ir_file(ast.clone(), "output")?;

    pb.set_message("Compiling to executable...");

    compile_to_executable(ast.clone(), "output")?;

    pb.finish_with_message("Done");

    Ok(())
}

fn check_cmd(path: &Path) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    fn check(path: &Path) {
        let source = read_to_string(path).expect("Failed to read file");

        let pb = create_progress_bar();

        match analyze_program(&source, pb.clone()) {
            Ok((_ast, analyzer)) => {
                pb.finish_and_clear();

                if analyzer.errors.is_empty() && analyzer.warnings.is_empty() {
                    println!("✅");
                } else {
                    for error in analyzer.errors {
                        println!("❗ {}", error);
                    }

                    for warning in analyzer.warnings {
                        println!("⚠️  {}", warning);
                    }
                }
            }
            Err(error) => {
                pb.finish_and_clear();

                println!("❌ Error: {:?}", error);
            }
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

#[allow(clippy::boxed_local)]
fn tokenize(input: &str, pb: Box<ProgressBar>) -> Result<TokenStream<'_>, String> {
    // Create a iterable list of tokens
    pb.set_message("Tokenizing...");

    let tokens = match lex(input) {
        Ok(result) => result,
        Err(e) => return Err(e.to_owned()),
    };

    Ok(tokens)
}

fn create_ast(input: &str, pb: Box<ProgressBar>) -> Result<Program, String> {
    // Tokenize the input
    let tokens = tokenize(input, pb.clone())?;

    // Parse the tokens to construct an AST
    pb.set_message("Parsing...");

    let ast = match parse(&mut tokens.clone()) {
        Ok(result) => result,
        Err(e) => return Err(e.to_owned()),
    };

    Ok(ast)
}

fn analyze_program(input: &str, pb: Box<ProgressBar>) -> Result<(Program, Analyzer), String> {
    // Create the AST
    let ast = create_ast(input, pb.clone())?;

    // Analyze the AST
    pb.set_message("Analyzing...");

    let mut analyzer = Analyzer::new();

    if let Err(e) = analyzer.analyze(ast.clone()) {
        return Err(e.to_owned());
    }

    Ok((ast, analyzer))
}

fn create_progress_bar() -> Box<ProgressBar> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "⠁", "⠂", "⠄", "⡀", "⡈", "⡐", "⡠", "⣀", "⣁", "⣂", "⣄", "⣌", "⣔", "⣤", "⣥", "⣦",
                "⣮", "⣶", "⣷", "⣿", "⡿", "⠿", "⢟", "⠟", "⡛", "⠛", "⠫", "⢋", "⠋", "⠍", "⡉", "⠉",
                "⠑", "⠡", "⢁", "✔",
            ]),
    );
    pb.set_message("Initializing...");

    Box::new(pb)
}
