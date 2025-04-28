// Library root for phi-detector. Move shared code here for integration testing.

pub mod results;
pub mod phi_patterns;
pub mod scanner;
pub mod redactor;

// Re-export main logic if needed (move from main.rs if used by tests)
// pub use crate::main_logic;
