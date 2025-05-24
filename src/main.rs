use clap::Parser;
use clipboard_rs::{Clipboard, ClipboardContext};
use ignore::WalkBuilder;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

/// Recursively list files and copy to clipboard, including content of non-binary files.
#[derive(Parser, Debug)]
#[command(name = "listclip")]
#[command(author = "You <you@example.com>")]
#[command(version = "1.2")]
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
}

/// Check if a file is binary by inspecting first few KB for null bytes
fn is_binary_file(path: &PathBuf) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut buffer = [0; 1024];
    let bytes_read = file.read(&mut buffer)?;
    Ok(buffer[..bytes_read].contains(&0))
}

fn collect_file_data(args: &Args) -> io::Result<String> {
    let mut builder = WalkBuilder::new(&args.start);
    builder.standard_filters(true);

    if args.no_gitignore {
        builder.git_ignore(false).git_exclude(false).ignore(false);
    }

    let mut output = String::new();
    let mut count = 0;

    for result in builder.build() {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path().to_path_buf();
        if !path.is_file() {
            continue;
        }

        count += 1;
        if args.verbose {
            println!("Reading: {}", path.display());
        }

        if let Ok(false) = is_binary_file(&path) {
            output.push_str(&format!("=== {} ===\n", path.display()));
            match fs::read_to_string(&path) {
                Ok(contents) => output.push_str(&format!("{}\n\n", contents)),
                Err(err) => output.push_str(&format!("(Could not read file: {})\n\n", err)),
            }
        } else {
            output.push_str(&format!("=== {} ===\n(Binary file skipped)\n\n", path.display()));
        }
    }

    if !args.verbose {
        println!("Copied {} file paths (and contents) to clipboard.", count);
    }

    Ok(output)
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let full_output = collect_file_data(&args)?;

    // Copy to clipboard
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_text(full_output.clone()).unwrap();

    // Save to file if needed
    if let Some(path) = args.out.as_ref() {
        let mut file = File::create(path)?;
        file.write_all(full_output.as_bytes())?;
    }

    thread::sleep(Duration::from_secs(2)); // For X11 clipboard hold
    Ok(())
}

