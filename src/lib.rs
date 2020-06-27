use libcli::args;
use std::{fs, path::Path, path::PathBuf, time};

pub fn run(config: args::Config) {
    let print = config.option("print").is_some();
    let watching = config.option("(unnamed)").unwrap_or_else(|| {
        eprintln!("Failed to get files or directories to watch");
        std::process::exit(1)
    });

    // logic
    let mut last_check = time::SystemTime::now();
    let mut changed = Vec::new();
    loop {
        for watch in watching {
            changed.append(&mut find_changed(last_check, Path::new(watch)));
        }

        if print {
            changed
                .iter()
                .for_each(|path| println!("{}", path.to_string_lossy()));
        }
        last_check = time::SystemTime::now();
        changed.clear();
        std::thread::sleep(time::Duration::from_millis(100));
    }
}

/// Checks if any files recursively have been modified or created after specified since
/// Returns a vector for all files that changed
fn find_changed(since: time::SystemTime, path: &Path) -> Vec<PathBuf> {
    let mut changed: Vec<PathBuf> = Vec::new();

    let metadata = match fs::metadata(path) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!("Unable to get metadata for entry {:?}, {}", path, msg);
            return Vec::new();
        }
    };
    if has_changed(since, &metadata) {
        changed.push(path.to_path_buf());
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
            changed.push(entry.path().to_path_buf());
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
