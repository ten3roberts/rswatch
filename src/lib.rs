use libcli::args;
use std::fs;
use std::path::Path;

pub fn run(config: args::Config) {
    let verbose = config.option("verbose").is_some();

    for watch in config.option("(unnamed)").unwrap() {
        read_dirs(Path::new(watch));
    }
}

fn read_dirs(path: &Path) {
    let files = match fs::read_dir(path) {
        Ok(v) => v,
        Err(msg) => {
            println!("Failed to read directory '{}'", msg);
            return;
        }
    };

    for file in files {
        let file = match file {
            Ok(v) => v,
            Err(_) => continue,
        };

        let metadata = match file.metadata() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if metadata.is_dir() {
            read_dirs(&file.path());
        }
        else
        {
            println!("File: {:?}", file.path());
        }
    }
}
