use crate::domain::{Category, Score, Severity};
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Unique identifier for a pattern (e.g., "ssn", "mrn-generic").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PatternId(String);

impl PatternId {
    /// Create a new PatternId, validating no whitespace or quotes.
    pub fn new(s: String) -> Result<Self, String> {
        if s.contains(char::is_whitespace) {
            return Err(format!("PatternId cannot contain whitespace: {}", s));
        }
        if s.contains('"') || s.contains('\'') {
            return Err(format!("PatternId cannot contain quotes: {}", s));
        }
        Ok(PatternId(s))
    }

    /// Get the underlying string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PatternId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type of detector used for this pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectorType {
    Regex,
    RegexWithContext,
    Dictionary,
    Combinatorial,
    Nlp,
}

/// A PHI detection pattern: regex, dictionary, or NLP-based rule.
#[derive(Debug, Clone)]
pub struct Pattern {
    pub id: PatternId,
    pub name: String,
    pub detector_type: DetectorType,
    pub category: Category,
    pub default_severity: Severity,
    pub score: Score,
    pub regex: Option<Regex>,
    pub context_words: Vec<String>,
    pub context_window: usize,
    pub redaction_template: Option<String>,
    pub redaction_strategy: Option<String>,
}

impl Pattern {
    /// Validate that regex field matches detector_type.
    pub fn validate(&self) -> Result<(), String> {
        match self.detector_type {
            DetectorType::Regex | DetectorType::RegexWithContext if self.regex.is_none() => {
                return Err(format!(
                    "Pattern {} ({:?}) requires a regex field",
                    self.id, self.detector_type
                ));
            }
            _ => {}
        }
        Ok(())
    }
}
