use clap::{Parser, ValueEnum};

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

fn main() {
    let cli = Cli::parse();
    println!("Parsed CLI args: {:?}", cli);
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
