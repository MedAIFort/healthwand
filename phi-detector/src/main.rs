use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use file_source::{FileSource, LocalFileSource};

/// Supported output formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputFormat {
    Json,
    Text,
}

/// CLI configuration
#[derive(Parser, Debug)]
#[command(name = "phi-detector")]
#[command(about = "Detect and redact PHI in text files", long_about = None)]
struct Cli {
    /// Input file or directory to scan
    #[arg(short, long)]
    input: String,

    /// Output format (json or text)
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Json)]
    output: OutputFormat,

    /// Enable redaction (replace PHI with placeholders)
    #[arg(short, long, default_value_t = false)]
    redact: bool,

    /// Verbosity level (repeat for more verbose)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

mod file_source;
mod phi_patterns;
mod scanner;
mod redactor;

fn main() {
    let cli = Cli::parse();
    println!("Parsed CLI args: {:?}", cli);

    // Allowed text file extensions
    let allowed_exts = vec!["txt", "md", "csv"]; // Extend as needed
    let file_source = LocalFileSource::new(&cli.input, allowed_exts.iter().map(|s| s.to_string()).collect());

    match file_source.files() {
        Ok(files) => {
            if files.is_empty() {
                println!("No text files found in the specified input.");
            } else {
                println!("Found {} file(s):", files.len());
                for f in &files {
                    println!("  {}", f.display());
                }
                // Example: Read and print first 100 chars of each file
                for f in &files {
                    match file_source.read_file(f) {
                        Ok(content) => {
                            let preview = content.chars().take(100).collect::<String>();
                            println!("\n--- {} ---\n{}...", f.display(), preview);
                        }
                        Err(e) => println!("Error reading {}: {}", f.display(), e),
                    }
                }
            }
        }
        Err(e) => {
            println!("Error traversing input: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parsing() {
        let args = [
            "phi-detector",
            "--input", "data/file.txt",
            "--output", "text",
            "--redact",
            "-vv",
        ];
        let cli = Cli::parse_from(&args);
        assert_eq!(cli.input, "data/file.txt");
        assert_eq!(cli.output, OutputFormat::Text);
        assert!(cli.redact);
        assert_eq!(cli.verbose, 2);
    }
}
