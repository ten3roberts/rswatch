use libcli::args;
use std::io::Write;
use std::sync::mpsc;
use std::{fs, path::Path, process, time};
#[macro_use]
mod macros;

pub fn run(config: args::Config) {
    let print = config.option("print").is_some();
    let watching = config.option("(unnamed)").unwrap_or_else(|| {
        error!("Failed to get files or directories to watch");
    });

    let interval: u64 = match config.option("interval") {
        Some(v) => match v[0].parse::<u64>() {
            Ok(v) => v,
            Err(_) => error!("Malformed interval, expected integer"),
        },
        None => 300,
    };

    let (sig_tx, sig_rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        sig_tx.send(true).unwrap_or_else(|_| {
            println!("Failed to transmit term signal");
            std::process::exit(1)
        })
    })
    .unwrap_or_else(|e| error!("Failed to setup handler for SIGINT {}", e));

    let exec_command = config.option("exec");

    // Represents a running process handle, if running
    let mut child_process: Option<process::Child> = None;

    // Switch options
    let verbose = config.option("verbose").is_some();
    let clear = config.option("clear").is_some();
    let exec_kill = config.option("kill").is_some();
    let mut exec_start = config.option("start").is_some();

    // logic
    let mut last_check = time::SystemTime::now();
    let mut changed = Vec::new();
    loop {
        // Check for SIGINT
        if let Ok(_) = sig_rx.try_recv() {
            println!("Child {:?}", child_process);
            match child_process {
                Some(child) => {
                    kill_or_wait_child(child, true, verbose);
                    child_process = None;
                    ()
                }
                None => {
                    if confirm_exit() {
                        break;
                    }
                }
            };

            // Clear channel from multiple ^C
            while let Ok(_) = sig_rx.try_recv() {}
        };

        // Check if child process has terminated
        if let Some(child) = &mut child_process {
            match child
                .try_wait()
                .unwrap_or_else(|e| error!("Failed to get status of child process {}", e))
            {
                Some(status) => {
                    verb_print!(
                        verbose,
                        "Child process exited with status {}",
                        status.code().unwrap_or_default()
                    );
                    child_process = None;
                }
                None => {}
            }
        }

        // Check for changed files
        for watch in watching {
            changed.append(&mut find_changed(last_check, Path::new(watch)));
        }

        if changed.len() > 0 || exec_start {
            if print {
                changed.iter().for_each(|path| println!("{}", path));
            }

            // Execute
            if let Some(command) = exec_command {
                child_process = exec_child(
                    child_process,
                    &substitute_args(&command, &changed),
                    exec_kill,
                    clear,
                    verbose,
                );
            };

            exec_start = false;
        }

        last_check = time::SystemTime::now();
        changed.clear();
        std::thread::sleep(time::Duration::from_millis(interval));
    }
}

fn confirm_exit() -> bool {
    let mut buf = String::new();
    print!("Are you sure you want to exit? [y/n]");
    std::io::stdout().flush().expect("Failed to flush stdout");

    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read stdin()");
    buf.to_lowercase().starts_with("y") || buf.is_empty()
}

fn clear_term() {
    print!("\x1B[2J\x1B[1;1H\n");
}

fn kill_or_wait_child(mut child: process::Child, kill: bool, verbose: bool) -> Option<()> {
    // Wait for process if already running
    // Tell process to exit

    if kill {
        verb_print!(verbose, "Killing child process");
        match child.kill() {
            Ok(()) => {}
            Err(msg) => {
                eprintln!("Failed to kill child process {}", msg);
                return None;
            }
        };
    }
    verb_print!(verbose, "Waiting for child process to exit");
    match child.wait() {
        Ok(status) => {
            if verbose {
                println!(
                    "Child process exited with status {}",
                    status.code().unwrap_or_default()
                )
            }
        }
        Err(msg) => {
            warn!("Failed to wait on child process {}", msg);
            return None;
        }
    };
    Some(())
}

// Takes in a a child process handle and waits for it to exit
// Returns a new child handle on success
// Tries to wait on previous process before executing new one
fn exec_child(
    child: Option<process::Child>,
    command: &Vec<String>,
    kill: bool,
    clear: bool,
    verbose: bool,
) -> Option<process::Child> {
    if let Some(child) = child {
        kill_or_wait_child(child, kill, verbose)?
    };
    if verbose {
        println!("Executing process {:?}", command);
    }

    if clear {
        clear_term();
    }
    let child = match process::Command::new(&command[0])
        .args(&command[1..])
        .spawn()
    {
        Ok(child) => Some(child),
        Err(msg) => {
            warn!("Failed to exec process {}, {}", &command[0], msg);
            None
        }
    };
    child
}

// Replaces all strings equal to "{}" with the whole contents of subs
fn substitute_args(args: &Vec<String>, subs: &Vec<String>) -> Vec<String> {
    let mut result = Vec::with_capacity(args.len());
    for arg in args {
        if arg == "{}" {
            result.append(&mut subs.clone());
        } else {
            result.push(arg.clone());
        }
    }
    result
}

/// Checks if any files recursively have been modified or created after specified since
/// Returns a vector for all files that changed
fn find_changed(since: time::SystemTime, path: &Path) -> Vec<String> {
    let mut changed: Vec<String> = Vec::new();

    let metadata = match fs::metadata(path) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!("Unable to get metadata for entry {:?}, {}", path, msg);
            return Vec::new();
        }
    };
    if has_changed(since, &metadata) {
        changed.push(path.to_path_buf().to_string_lossy().into_owned());
    }

    if metadata.is_file() {
        return changed;
    };

    let files = match fs::read_dir(path) {
        Ok(v) => v,
        Err(msg) => {
            println!("Failed to read directory '{}'", msg);
            return Vec::new();
        }
    };

    // Look through all files
    for entry in files {
        let entry = match entry {
            Ok(v) => v,
            Err(_) => continue,
        };

        let metadata = match entry.metadata() {
            Ok(v) => v,
            Err(msg) => {
                eprintln!(
                    "Unable to get metadata for entry {:?}, {}",
                    entry.path(),
                    msg
                );
                continue;
            }
        };

        if metadata.is_dir() {
            changed.append(&mut find_changed(since, &entry.path()));
        } else if has_changed(since, &metadata) {
            changed.push(entry.path().to_path_buf().to_string_lossy().into_owned());
        }
    }
    changed
}

fn has_changed(since: time::SystemTime, metadata: &fs::Metadata) -> bool {
    let modified = match metadata.modified() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Unable to get modification date");
            return false;
        }
    };

    modified > since
}
