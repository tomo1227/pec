use std::collections::HashMap;

use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    dirs: Vec<std::path::PathBuf>,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    debug: bool,
}

pub fn run(_args: std::env::ArgsOs) -> Result<(), i32> {
    let cli = Cli::parse();

    let mut file_map: HashMap<String, Vec<String>> = HashMap::new();
    for dir in cli.dirs {
        for entry in WalkDir::new(&dir).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                file_map
                    .entry(file_name)
                    .or_default()
                    .push(entry.path().to_string_lossy().into_owned());
            }
        }
    }
    for (file_name, paths) in file_map {
        if paths.len() > 1 {
            println!("File: {}", file_name);
            for path in paths {
                println!("  {}", path);
            }
        }
    }
    println!("pattern: {:?}", cli.pattern);
    // println!("path: {:?}", cli.dirs);
    println!("verbose mode: {:?}", cli.verbose);
    Ok(())
}
