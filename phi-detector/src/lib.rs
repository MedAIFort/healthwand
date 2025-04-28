// Library root for phi-detector. Move shared code here for integration testing.

pub mod results;
pub mod phi_patterns;
pub mod scanner;
pub mod redactor;
pub mod file_source;

pub use results::{DetectionResult, ResultsSummary, OutputBundle};
pub use file_source::{FileSource, LocalFileSource};
pub use redactor::*;

// Re-export main logic if needed (move from main.rs if used by tests)
// pub use crate::main_logic;
