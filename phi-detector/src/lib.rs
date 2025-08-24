// Library root for phi-detector. Move shared code here for integration testing.

pub mod file_source;
pub mod phi_patterns;
pub mod redactor;
pub mod results;
pub mod scanner;

pub use file_source::{FileSource, LocalFileSource};
pub use redactor::*;
pub use results::{DetectionResult, OutputBundle, ResultsSummary};

// Re-export main logic if needed (move from main.rs if used by tests)
// pub use crate::main_logic;
