use clap::Parser;
use clipboard_rs::{Clipboard, ClipboardContext};
use ignore::WalkBuilder;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

/// Recursively list files and copy to clipboard, respecting .gitignore by default.
#[derive(Parser, Debug)]
#[command(name = "listclip")]
#[command(author = "You <you@example.com>")]
#[command(version = "1.1")]
#[command(about = "List files and copy to clipboard", long_about = None)]
struct Args {
    /// Starting directory (default: current dir)
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    start: PathBuf,

    /// Output file to save the list
    #[arg(short, long, value_name = "FILE")]
    out: Option<PathBuf>,

    /// Verbose mode: print all file paths
    #[arg(short, long)]
    verbose: bool,

    /// Do not respect .gitignore rules
    #[arg(long)]
    no_gitignore: bool,
}

fn collect_paths(args: &Args) -> Vec<String> {
    let mut file_list = Vec::new();

    let mut builder = WalkBuilder::new(&args.start);
    builder.standard_filters(true);

    if args.no_gitignore {
        builder.git_ignore(false).git_exclude(false).ignore(false);
    }

    let walker = builder.build();

    for result in walker {
        if let Ok(entry) = result {
            let path = entry.path();
            if path.is_file() || path.is_dir() {
                file_list.push(path.display().to_string());
            }
        }
    }

    file_list
}

fn main() {
    let args = Args::parse();

    let file_list = collect_paths(&args);
    let joined = file_list.join("\n");

    // Copy to clipboard
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_text(joined.clone()).unwrap();

    // Save to file if specified
    if let Some(path) = args.out.as_ref() {
        if let Ok(mut file) = File::create(path) {
            if let Err(e) = file.write_all(joined.as_bytes()) {
                eprintln!("Failed to write to file: {}", e);
            }
        } else {
            eprintln!("Failed to create output file.");
        }
    }

    // Print output
    if args.verbose {
        println!(
            "Collected {} paths from {:?} (gitignore respected: {})\n",
            file_list.len(),
            args.start,
            !args.no_gitignore
        );
        for path in &file_list {
            println!("{}", path);
        }
    } else {
        println!(
            "Copied {} paths from {:?} to clipboard (gitignore respected: {}).",
            file_list.len(),
            args.start,
            !args.no_gitignore
        );
    }

    thread::sleep(Duration::from_secs(2)); // For Linux/X11 clipboard hold
}
