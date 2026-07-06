use crate::{
    steps::{analyze_program, build, create_ast, tokenize},
    watcher::run_with_flags,
};
use clap::{
    Arg, ArgAction, Command,
    builder::{
        Styles,
        styling::{AnsiColor, Color, Style},
    },
};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::read_to_string, path::Path, time::Duration};

mod steps;
mod watcher;

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
    let matches = Command::new("gneurshk")
        .about(format!(
            "{} is an awesome programming language",
            style("Gneurshk").magenta().bright()
        ))
        .version("0.1.0")
        .styles(CLAP_STYLING)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("Builds and runs a file")
                .arg(
                    Arg::new("file")
                        .help("The file to run")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("watch")
                        .help("Restarts on file changes")
                        .required(false)
                        .action(ArgAction::SetTrue)
                        .long("watch")
                        .short('w'),
                )
                .arg(
                    Arg::new("output-ir")
                        .help("Outputs the LLVM IR file")
                        .required(false)
                        .action(ArgAction::SetTrue)
                        .long("ir"),
                ),
        )
        .subcommand(
            Command::new("build")
                .about("Compiles a file into an executable")
                .arg(
                    Arg::new("file")
                        .help("The file to build")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("output-ir")
                        .help("Outputs the LLVM IR file")
                        .required(false)
                        .action(ArgAction::SetTrue)
                        .long("ir"),
                ),
        )
        .subcommand(
            Command::new("lex")
                .about("Lexes a file and prints the tokens")
                .arg(
                    Arg::new("file")
                        .help("The file to lex")
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
                        .help("The file to parse")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("check")
                .about("Type check a file")
                .arg(
                    Arg::new("file")
                        .help("The file to check")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("watch")
                        .help("Rechecks on file changes")
                        .required(false)
                        .action(ArgAction::SetTrue)
                        .long("watch")
                        .short('w'),
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

            // Get the flags from the arguments
            let is_watching = query_matches.get_flag("watch");
            let output_ir = query_matches.get_flag("output-ir");

            // Run the build command with the command flags
            run_with_flags(
                path,
                || {
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
                    match build(&source, output_ir, pb.clone()) {
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
                    }
                },
                is_watching,
            );
        }
        Some(("build", query_matches)) => {
            // Get the path from the arguments
            let path = query_matches
                .get_one::<String>("file")
                .expect("Argument 'file' is required");
            let path: &Path = path.as_ref();

            // Get the flags from the arguments
            let output_ir = query_matches.get_flag("output-ir");

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
            match build(&source, output_ir, pb.clone()) {
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

            // Get the watching flag from the arguments
            let is_watching = query_matches.get_flag("watch");

            // Check the source code for errors
            run_with_flags(
                path,
                || {
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
                        Ok((_ast, analyzed)) => {
                            pb.finish_and_clear();

                            let errors = analyzed.get_all_errors();
                            let warnings = analyzed.get_all_warnings();

                            // Print the errors and warnings
                            if errors.is_empty() && warnings.is_empty() {
                                println!("✅");
                            } else {
                                for error in errors {
                                    eprintln!("❗ {}", error);
                                }

                                for warning in warnings {
                                    eprintln!("⚠️  {}", warning);
                                }
                            }
                        }
                        Err(error) => {
                            pb.finish_and_clear();

                            eprintln!("❌ Error: {:?}", error);
                        }
                    }
                },
                is_watching,
            )
        }
        // If all subcommands are defined above, anything else is unreachable
        _ => unreachable!(),
    }
}

fn create_progress_bar() -> Box<ProgressBar> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            // Spinner was sourced from
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
