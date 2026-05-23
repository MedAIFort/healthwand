use std::io;

/// HealthWand's main error type.
///
/// Uses thiserror for ergonomic error handling.
/// All fallible operations should return `Result<T>` = `std::result::Result<T, HealthwandError>`.
#[derive(Debug, thiserror::Error)]
pub enum HealthwandError {
    #[error("config error: {0}")]
    ConfigError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("YAML error: {0}")]
    YamlError(String),

    #[error("unsupported detector type: {0}")]
    UnsupportedDetector(String),
}

/// Convenience type alias for `Result<T, HealthwandError>`.
pub type Result<T> = std::result::Result<T, HealthwandError>;
