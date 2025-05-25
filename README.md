# CodebaseAsReference4Chatbot
Simple Tool that enambles you to get whole codebase as reference for chatbots like chatgpt etc. so you dont have to copy all your files and pass them to it


📋 A fast, flexible Rust CLI to recursively **list files** and **copy them to your clipboard** (or save to a file). Supports file content copying, filtering, JSON output, `.gitignore` respect, and more.

## 🔧 Features

- ✅ Recursively list files and directories
- 📋 Copy file names and contents to clipboard
- 📁 Optional output to file
- 🎯 Filter by file extensions
- 🧾 JSON output support
- 🚫 Skip binary files
- 🛑 Ignore files via `.gitignore` (by default)
- ⚡ CLI powered by `clap`

🎛 Options

| Option                  | Description                                                  |
| ----------------------- | ------------------------------------------------------------ |
| `-d`, `--dir <DIR>`     | Start directory (default: current directory)                 |
| `-o`, `--output <FILE>` | Save output to file instead of clipboard                     |
| `-v`, `--verbose`       | Verbosely list all file paths                                |
| `--no-content`          | Do not include file contents in output                       |
| `--no-gitignore`        | Disable `.gitignore` filtering                               |
| (`--filter <EXT>`)        | Not working rn Only include files with given extension(s) (e.g. `rs`, `md`) |
| `--json`                | Output data as JSON                                          |
| `-h`, `--help`          | Print help information                                       |
| `--version`             | Show version info                                            |

📋 Output Format
Default: 
```
file1.txt
dir/file2.rs
...

=== Contents ===

--- file1.txt ---
(content of file1)

--- dir/file2.rs ---
(content of file2)
```

Json: 
```
{
  "files": [
    {
      "path": "file1.txt",
      "content": "..."
    },
    {
      "path": "dir/file2.rs",
      "content": "..."
    }
  ]
}
```

Build:
 -- test:

```
cargo run
```

Install: 
```
cargo install --path .
```
