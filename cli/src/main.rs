use clap::{
    Arg, ArgAction, Command,
    builder::{
        Styles,
        styling::{AnsiColor, Color, Style},
    },
};
use colored::Colorize;
use gneurshk_analyzer::Analyzer;
use gneurshk_compiler::output::{executable::compile_to_executable, ir::create_llvm_ir_file};
use gneurshk_lexer::{TokenStream, lex};
use gneurshk_parser::{Program, parse};
use indicatif::{ProgressBar, ProgressStyle};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    time::Duration,
};

pub const CLAP_STYLING: Styles = Styles::styled()
    .header(Style::new().bold())
    .usage(Style::new().bold())
    .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue))))
    .placeholder(Style::new().dimmed())
    .error(
        Style::new()
            .bold()
            .fg_color(Some(Color::Ansi(AnsiColor::Red))),
    )
    .valid(
        Style::new()
            .bold()
            .fg_color(Some(Color::Ansi(AnsiColor::BrightGreen))),
    )
    .invalid(
        Style::new()
            .bold()
            .fg_color(Some(Color::Ansi(AnsiColor::Red))),
    );

fn main() {
    let matches = Command::new("pacman")
        .about(format!(
            "{} is an awesome programming language",
            "Gneurshk".bright_magenta()
        ))
        .version("0.1.0")
        .styles(CLAP_STYLING)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run").about("Builds and runs a file").arg(
                Arg::new("file")
                    .help("The file to run")
                    .required(true)
                    .action(ArgAction::Set)
                    .num_args(1),
            ),
        )
        .subcommand(
            Command::new("build")
                .about("Compiles a file into an executable")
                .arg(
                    Arg::new("file")
                        .help("The file to run")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("lex")
                .about("Lexes a file and prints the tokens")
                .arg(
                    Arg::new("file")
                        .help("The file to run")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("parse")
                .about("Parses a file and prints the AST")
                .arg(
                    Arg::new("file")
                        .help("The file to run")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("check")
                .about("Watches a file for changes and checks code validity")
                .arg(
                    Arg::new("file")
                        .help("The file to run")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", query_matches)) => {
            // Get the path from the arguments
            let path = query_matches
                .get_one::<String>("file")
                .expect("Argument 'file' is required");
            let path: &Path = path.as_ref();

            // Read the file
            let source = match read_to_string(path) {
                Ok(source) => source,
                Err(e) => {
                    eprintln!("Error: {e}");
                    return;
                }
            };

            // Create the progress bar
            let pb = create_progress_bar();

            // Build the source code
            match build_cmd(&source, pb.clone()) {
                Ok(executable_path) => {
                    pb.finish_with_message("Running executable");

                    // Run the executable
                    let path = std::path::absolute(executable_path).unwrap();

                    let mut child = std::process::Command::new(&path)
                        .stdout(std::process::Stdio::inherit())
                        .stderr(std::process::Stdio::inherit())
                        .spawn()
                        .unwrap();

                    child.wait().unwrap();
                }
                Err(e) => {
                    pb.finish_and_clear();

                    eprintln!("Error: {e}");
                }
            };
        }
        Some(("build", query_matches)) => {
            // Get the path from the arguments
            let path = query_matches
                .get_one::<String>("file")
                .expect("Argument 'file' is required");
            let path: &Path = path.as_ref();

            // Read the file
            let source = match read_to_string(path) {
                Ok(source) => source,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            // Create the progress bar
            let pb = create_progress_bar();

            // Build the source code
            match build_cmd(&source, pb.clone()) {
                Ok(_) => {
                    pb.finish_with_message("Successfully built executable");
                }
                Err(e) => {
                    pb.finish_and_clear();

                    eprintln!("Error: {e}");
                }
            };
        }

        Some(("lex", query_matches)) => {
            // Get the path from the arguments
            let path = query_matches
                .get_one::<String>("file")
                .expect("Argument 'file' is required");
            let path: &Path = path.as_ref();

            // Read the file
            let source = match read_to_string(path) {
                Ok(source) => source,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            // Create the progress bar
            let pb = create_progress_bar();

            // Tokenize the source code
            match tokenize(&source, pb.clone()) {
                Ok(tokens) => {
                    pb.finish_with_message("Finished lexing");

                    for (token, range) in tokens {
                        println!("{}..{}\t{:?}", range.start, range.end, token);
                    }
                }
                Err(e) => {
                    pb.finish_and_clear();

                    eprintln!("Error: {e}")
                }
            }
        }
        Some(("parse", query_matches)) => {
            // Get the path from the arguments
            let path = query_matches
                .get_one::<String>("file")
                .expect("Argument 'file' is required");
            let path: &Path = path.as_ref();

            // Read the file
            let source = match read_to_string(path) {
                Ok(source) => source,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            // Create the progress bar
            let pb = create_progress_bar();

            // Parse the source code
            match create_ast(&source, pb.clone()) {
                Ok(ast) => {
                    pb.finish_with_message("Finished parsing");

                    println!("AST: {ast:#?}")
                }
                Err(e) => {
                    pb.finish_and_clear();

                    eprintln!("Error: {e}")
                }
            }
        }
        Some(("check", query_matches)) => {
            // Get the path from the arguments
            let path = query_matches
                .get_one::<String>("file")
                .expect("Argument 'file' is required");
            let path: &Path = path.as_ref();

            // Check the source code for errors
            if let Err(error) = check_cmd(path) {
                eprintln!("Error: {error:?}");
            }
        }
        // If all subcommands are defined above, anything else is unreachable
        _ => unreachable!(),
    }
}

fn build_cmd(input: &str, pb: Box<ProgressBar>) -> Result<PathBuf, String> {
    // Analyze the program
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

    // Create the LLVM IR file
    pb.set_message("Creating LLVM IR file...");

    create_llvm_ir_file(ast.clone(), "output".as_ref())?;

    // Create the executable
    pb.set_message("Compiling to executable...");

    let executable_path = compile_to_executable(ast.clone(), "output".as_ref())?;

    Ok(executable_path)
}

fn check_cmd(path: &Path) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    fn check(path: &Path) {
        // Read the file
        let source = match read_to_string(path) {
            Ok(source) => source,
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };

        // Create the progress bar
        let pb = create_progress_bar();

        // Analyze the program
        match analyze_program(&source, pb.clone()) {
            Ok((_ast, analyzer)) => {
                pb.finish_and_clear();

                // Print the errors and warnings
                if analyzer.errors.is_empty() && analyzer.warnings.is_empty() {
                    println!("✅");
                } else {
                    for error in analyzer.errors {
                        eprintln!("❗ {}", error);
                    }

                    for warning in analyzer.warnings {
                        eprintln!("⚠️  {}", warning);
                    }
                }
            }
            Err(error) => {
                pb.finish_and_clear();

                eprintln!("❌ Error: {:?}", error);
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
            Err(error) => eprintln!(
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
