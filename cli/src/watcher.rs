use console::style;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

pub(crate) fn run_with_flags<F>(path: &Path, callback: F, is_watching: bool)
where
    F: Fn(),
{
    if is_watching {
        if let Err(e) = watch(path, callback) {
            eprintln!("Watcher error: {e}");
        }
    } else {
        callback();
    }
}

pub(crate) fn watch<F>(path: &Path, callback: F) -> notify::Result<()>
where
    F: Fn(),
{
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default().with_compare_contents(true))?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    println!("{} Process has started.", style("Watcher").green().bright());

    callback();

    let mut restart_count = 0;

    for res in rx {
        // Clear the screen
        clearscreen::clear().unwrap();

        restart_count += 1;

        match res {
            Ok(event) => {
                // Print a restarting message
                if let Some(path) = event.paths.first() {
                    println!(
                        "{} Restarting! File change detected: \"{}\" {}",
                        style("Watcher").green().bright(),
                        match std::env::current_dir() {
                            Ok(cwd) => path.strip_prefix(&cwd).unwrap_or(path).display(),
                            Err(_) => path.display(),
                        },
                        style(format!("x{}", restart_count)).dim(),
                    );
                } else {
                    println!(
                        "{} Restarting! File change detected",
                        style("Watcher").green().bright()
                    );
                }

                // Restart the process
                callback();

                // Once the process has finished, print a finishing message
                println!(
                    "{} Process has finished. Restarting on file change...",
                    style("Watcher").green().bright()
                );
            }
            Err(error) => eprintln!(
                "{} Encountered an error: {}",
                style("Watcher").green().bright(),
                error
            ),
        }
    }

    Ok(())
}
