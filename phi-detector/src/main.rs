use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use phi_detector::file_source::{FileSource, LocalFileSource};
use phi_detector::results::{DetectionResult, ResultsSummary, OutputBundle};
use phi_detector::scanner;
use phi_detector::phi_patterns;
use phi_detector::redactor::*;
use log::{info, warn, error};
use thiserror::Error;
use std::collections::HashMap;
use serde_json;

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

#[derive(Debug, Error)]
pub enum AppError {
    #[error("File IO error: {0}")]
    FileIO(#[from] std::io::Error),
    #[error("JSON serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Scan error: {0}")]
    Scan(String),
    #[error("Redaction error: {0}")]
    Redact(String),
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    info!("Starting PHI detection pipeline");
    let mut summary = ResultsSummary::default();
    let mut all_results = Vec::new();
    let mut errors = Vec::new();

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
                for f in &files {
                    match file_source.read_file(f) {
                        Ok(content) => {
                            let scanner = scanner::Scanner::new(phi_patterns::PHIPattern::all_patterns(), 10);
                            let detections = scanner.scan(&content);
                            let redactor = Redactor::new(RedactionStrategy::FullReplacement);

                            // Precompute redacted text for each detection (safe, independent of buffer mutation)
                            let mut redacted_map = std::collections::HashMap::new();
                            for det in &detections {
                                let replacement = redactor.redaction_text(&det.phi_type, &det.matched_text);
                                redacted_map.insert((det.start, det.end), replacement);
                            }
                            let redacted = redactor.redact(&content, &detections);

                            for det in &detections {
                                let result = DetectionResult {
                                    file_path: f.display().to_string(),
                                    phi_type: det.phi_type.clone(),
                                    location: (det.start, det.end),
                                    context: det.context.clone(),
                                    matched_text: det.matched_text.clone(),
                                    redacted_text: redacted_map.get(&(det.start, det.end)).cloned(),
                                };
                                *summary.detections_by_type.entry(det.phi_type.clone()).or_insert(0) += 1;
                                all_results.push(result);
                            }
                            summary.files_processed += 1;
                            summary.total_detections += detections.len();
                            summary.redacted_count += detections.len();
                        }
                        Err(e) => {
                            error!("Error reading {}: {}", f.display(), e);
                            errors.push(format!("Read: {}", e));
                            summary.errors.push(format!("Read: {}", e));
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error traversing input: {}", e);
            errors.push(format!("Traverse: {}", e));
        }
    }

    // Output results according to --output format
    match cli.output {
        OutputFormat::Json => {
            let output_bundle = OutputBundle {
                results: all_results,
                summary,
            };
            match serde_json::to_string_pretty(&output_bundle) {
                Ok(json) => println!("{}", json),
                Err(e) => {
                    error!("Failed to serialize results: {}", e);
                    errors.push(format!("Serialize: {}", e));
                }
            }
        }
        OutputFormat::Text => {
            println!("Detection Results:");
            for result in &all_results {
                println!("- File: {} | Type: {:?} | Location: {:?} | Context: {} | Matched: {} | Redacted: {}",
                    result.file_path,
                    result.phi_type,
                    result.location,
                    result.context,
                    result.matched_text,
                    result.redacted_text.as_deref().unwrap_or("<none>"));
            }
            println!("\nSummary:\n  Files processed: {}\n  Total detections: {}\n  Redacted: {}\n  Detections by type: {:?}",
                summary.files_processed,
                summary.total_detections,
                summary.redacted_count,
                summary.detections_by_type);
            if !summary.errors.is_empty() {
                println!("  Errors: {:?}", summary.errors);
            }
        }
    }

    // Print errors if any
    if !errors.is_empty() {
        println!("Errors: {:?}", errors);
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
