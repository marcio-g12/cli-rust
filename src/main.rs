use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "rgl")]
struct Args{
    /// Pattern to search
    pattern: String,
    
    ///path to dir
    path: String,
}

fn main () -> Result<()> {
    let args = Args::parse();

    for entry in WalkDir::new(&args.path).into_iter().filter_map(|e| e.ok()){
        
        if entry.file_type().is_file(){
            search_file(entry.path(), &args.pattern)?;
        }
    }
    Ok(())
}

fn search_file(path: &std::path::Path, pattern: &str) -> Result<()> {
    let file= File::open(path)?;
    let reader = BufReader::new(file);

    for (line_num, line_result) in reader.lines().enumerate() {
        
        let line = match line_result {
            Ok(line)=> line,
            Err(_) => {
                return Ok(());
            }
        };
    
        if line.contains(pattern) {
            println! ("{}:{}: {}", path.display(), line_num + 1, line);
        }
    }
    Ok(())
}