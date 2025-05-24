use clap::Parser;
use clipboard_rs::{Clipboard, ClipboardContext};
use ignore::WalkBuilder;
use serde::Serialize;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

/// Recursively list files and copy to clipboard, optionally with contents.
#[derive(Parser, Debug)]
#[command(name = "listclip")]
#[command(author = "You <you@example.com>")]
#[command(version = "1.5")]
#[command(about = "List files and copy to clipboard", long_about = None)]
struct Args {
    /// Starting directory (default: current dir)
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    start: PathBuf,

    /// Output file to save the list and contents
    #[arg(short, long, value_name = "FILE")]
    out: Option<PathBuf>,

    /// Verbose mode: print file paths while reading
    #[arg(short, long)]
    verbose: bool,

    /// Do not respect .gitignore rules
    #[arg(long)]
    no_gitignore: bool,

    /// Filter by file extensions (e.g. --ext rs --ext toml)
    #[arg(short, long, value_name = "EXT")]
    ext: Vec<String>,

    /// Disable content inclusion, list files only
    #[arg(long)]
    no_content: bool,

    /// Output as JSON
    #[arg(long)]
    json: bool,
}

#[derive(Serialize)]
struct FileEntry {
    path: String,
    content: Option<String>,
}

/// Check if a file is binary by inspecting the first chunk for null bytes
fn is_binary_file(path: &PathBuf) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut buffer = [0; 1024];
    let bytes_read = file.read(&mut buffer)?;
    Ok(buffer[..bytes_read].contains(&0))
}

/// Return true if file matches extension filters (or if no filters given)
fn matches_extension(path: &Path, filters: &[String]) -> bool {
    if filters.is_empty() {
        return true;
    }
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext_str| filters.iter().any(|f| f == ext_str))
        .unwrap_or(false)
}

fn collect_file_data(args: &Args) -> io::Result<String> {
    let mut builder = WalkBuilder::new(&args.start);
    builder.standard_filters(true);

    if args.no_gitignore {
        builder.git_ignore(false).git_exclude(false).ignore(false);
    }

    let mut entries: Vec<FileEntry> = Vec::new();
    let mut list_section = String::from("=== File & Directory List ===\n");
    let mut content_section = String::new();
    let mut count = 0;

    for result in builder.build() {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path().to_path_buf();
        let display_path = path.display().to_string();
        list_section.push_str(&format!("{}\n", display_path));

        if path.is_file() && matches_extension(&path, &args.ext) {
            count += 1;
            if args.verbose {
                println!("Reading: {}", display_path);
            }

            let content = if args.no_content {
                None
            } else if let Ok(false) = is_binary_file(&path) {
                match fs::read_to_string(&path) {
                    Ok(c) => Some(c),
                    Err(_) => None,
                }
            } else {
                None
            };

            entries.push(FileEntry {
                path: display_path.clone(),
                content: content.clone(),
            });

            if !args.json && content.is_some() {
                content_section.push_str(&format!("\n=== {} ===\n{}\n", display_path, content.unwrap()));
            }
        } else {
            entries.push(FileEntry {
                path: display_path.clone(),
                content: None,
            });
        }
    }

    if args.json {
        Ok(serde_json::to_string_pretty(&entries).unwrap())
    } else {
        Ok(format!("{}\n{}", list_section, content_section))
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let output = collect_file_data(&args)?;

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_text(output.clone()).unwrap();

    if let Some(path) = &args.out {
        let mut file = File::create(path)?;
        file.write_all(output.as_bytes())?;
    }

    
    println!(
        "Copied file list{} to clipboard.",
        if args.no_content { "" } else { " with content" }
    );


    thread::sleep(Duration::from_secs(2));
    Ok(())
}
