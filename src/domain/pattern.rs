use crate::domain::{Category, Score, Severity};
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Unique identifier for a pattern (e.g., "ssn", "mrn-generic").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PatternId(String);

impl PatternId {
    /// Create a new PatternId, validating no whitespace or quotes.
    pub fn new(s: String) -> crate::error::Result<Self> {
        if s.contains(char::is_whitespace) {
            return Err(crate::error::HealthwandError::ConfigError(format!(
                "PatternId cannot contain whitespace: {}",
                s
            )));
        }
        if s.contains('"') || s.contains('\'') {
            return Err(crate::error::HealthwandError::ConfigError(format!(
                "PatternId cannot contain quotes: {}",
                s
            )));
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
    /// Called automatically by `validated()` to enforce invariants.
    pub fn validate(&self) -> crate::error::Result<()> {
        match self.detector_type {
            DetectorType::Regex | DetectorType::RegexWithContext if self.regex.is_none() => {
                return Err(crate::error::HealthwandError::ConfigError(format!(
                    "Pattern {} ({:?}) requires a regex field",
                    self.id, self.detector_type
                )));
            }
            _ => {}
        }
        Ok(())
    }

    /// Validate and return a Pattern, ensuring all invariants hold.
    /// This is the canonical way to construct Patterns from external data (e.g., YAML).
    /// The config loader (M2 PHASE 3) must use this method to ensure no invalid patterns reach the scanner.
    pub fn validated(self) -> crate::error::Result<Self> {
        self.validate()?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_regex_pattern_without_regex_field_fails() {
        let pattern = Pattern {
            id: PatternId::new("test-id".to_string()).unwrap(),
            name: "Test Pattern".to_string(),
            detector_type: DetectorType::Regex,
            category: crate::domain::Category::Medical,
            default_severity: Severity::High,
            score: Score::new(0.9).unwrap(),
            regex: None,
            context_words: vec![],
            context_window: 10,
            redaction_template: None,
            redaction_strategy: None,
        };

        let result = pattern.validate();
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::error::HealthwandError::ConfigError(msg) => {
                assert!(msg.contains("requires a regex field"));
            }
            _ => panic!("Expected ConfigError"),
        }
    }

    #[test]
    fn test_validate_regex_with_context_pattern_without_regex_field_fails() {
        let pattern = Pattern {
            id: PatternId::new("test-id".to_string()).unwrap(),
            name: "Test Pattern".to_string(),
            detector_type: DetectorType::RegexWithContext,
            category: crate::domain::Category::Personal,
            default_severity: Severity::Medium,
            score: Score::new(0.7).unwrap(),
            regex: None,
            context_words: vec!["context".to_string()],
            context_window: 20,
            redaction_template: None,
            redaction_strategy: None,
        };

        let result = pattern.validate();
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::error::HealthwandError::ConfigError(msg) => {
                assert!(msg.contains("requires a regex field"));
            }
            _ => panic!("Expected ConfigError"),
        }
    }

    #[test]
    fn test_validate_dictionary_pattern_without_regex_succeeds() {
        let pattern = Pattern {
            id: PatternId::new("dict-pattern".to_string()).unwrap(),
            name: "Dictionary Pattern".to_string(),
            detector_type: DetectorType::Dictionary,
            category: crate::domain::Category::Medical,
            default_severity: Severity::High,
            score: Score::new(0.8).unwrap(),
            regex: None,
            context_words: vec![],
            context_window: 5,
            redaction_template: None,
            redaction_strategy: None,
        };

        assert!(pattern.validate().is_ok());
    }

    #[test]
    fn test_validated_method_enforces_invariants() {
        let pattern = Pattern {
            id: PatternId::new("invalid".to_string()).unwrap(),
            name: "Invalid Pattern".to_string(),
            detector_type: DetectorType::Regex,
            category: crate::domain::Category::Identifier,
            default_severity: Severity::Critical,
            score: Score::new(0.95).unwrap(),
            regex: None,
            context_words: vec![],
            context_window: 10,
            redaction_template: None,
            redaction_strategy: None,
        };

        let result = pattern.validated();
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::error::HealthwandError::ConfigError(msg) => {
                assert!(msg.contains("requires a regex field"));
            }
            _ => panic!("Expected ConfigError"),
        }
    }

    #[test]
    fn test_validated_method_succeeds_with_valid_pattern() {
        let pattern = Pattern {
            id: PatternId::new("valid".to_string()).unwrap(),
            name: "Valid Pattern".to_string(),
            detector_type: DetectorType::Dictionary,
            category: crate::domain::Category::Medical,
            default_severity: Severity::High,
            score: Score::new(0.85).unwrap(),
            regex: None,
            context_words: vec![],
            context_window: 10,
            redaction_template: None,
            redaction_strategy: None,
        };

        let result = pattern.validated();
        assert!(result.is_ok());
    }
}
