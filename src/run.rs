use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    dirs: Vec<PathBuf>,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    debug: bool,
}

pub fn run(_args: std::env::ArgsOs) -> Result<(), i32> {
    let cli = Cli::parse();

    let mut file_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for dir in cli.dirs.clone() {
        // TODO:fn dir内のすべてのファイルを抽出
        // 同じものをhashmapに入れる
        for entry in WalkDir::new(&dir).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                file_map
                    .entry(file_name)
                    .or_default()
                    .push(entry.path().to_path_buf());
            }
        }
    }
    for (file_name, paths) in file_map {
        if paths.len() > 1 {
            println!("重複ファイル: {}", file_name);
            for path in paths {
                println!("  {}", path.display());
            }
        }
    }
    Ok(())
}
