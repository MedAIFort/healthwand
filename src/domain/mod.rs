//! Domain layer: core types for PHI detection
//!
//! This module contains the heart of HealthWand's domain logic:
//! zero I/O, zero external adapters, pure business logic.
//!
//! The hexagonal architecture ensures that domain types are I/O-agnostic,
//! allowing adapters (scanner, config, format) to remain decoupled.

pub mod category;
pub mod finding;
pub mod pattern;
pub mod score;
pub mod severity;
pub mod span;

// Re-exports for convenience at crate::domain::* level
pub use category::Category;
pub use finding::{Finding, UuPdpArticle};
pub use pattern::{DetectorType, Pattern, PatternId, RedactionStrategy};
pub use score::Score;
pub use severity::Severity;
pub use span::MatchSpan;
