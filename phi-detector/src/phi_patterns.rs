use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PHIType {
    SSN,
    MedicalRecordNumber,
    ICD10,
    DateOfBirth,
    IndonesianNIK,
    IndonesianBPJS,
    // Add more as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternContext {
    pub before: Option<Vec<String>>,
    pub after: Option<Vec<String>>,
    pub window: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redaction {
    pub template: String,
    pub strategy: String, // Could be enum if you want strong typing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMetadata {
    pub category: Option<String>,
    pub severity: Option<String>,
    pub examples: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PHIPatternConfig {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub regex: String,
    pub context: Option<PatternContext>,
    pub confidence: Option<f32>,
    pub redaction: Redaction,
    pub metadata: Option<PatternMetadata>,
}

impl PHIPatternConfig {
    /// Loads PHI pattern configurations from a YAML file.
    ///
    /// Reads the specified YAML file and deserializes its contents into a `PHIPatternsFile` struct containing pattern definitions, defaults, and overrides.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or if the YAML content is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// let patterns = PHIPatternConfig::from_yaml_file("patterns.yaml").unwrap();
    /// assert!(!patterns.patterns.is_empty());
    /// ```
    pub fn from_yaml_file<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<PHIPatternsFile, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        let patterns_file: PHIPatternsFile = serde_yaml::from_reader(reader)?;
        Ok(patterns_file)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PHIPatternsFile {
    pub patterns: Vec<PHIPatternConfig>,
    pub defaults: Option<serde_yaml::Value>,
    pub overrides: Option<serde_yaml::Value>,
}

#[derive(Debug, Clone)]
pub struct PHIPattern {
    pub phi_type: PHIType,
    pub regex: Regex,
    pub context: Option<PatternContext>,
    pub confidence: Option<f32>,
    pub redaction: Redaction,
    pub metadata: Option<PatternMetadata>,
}

impl PHIPattern {
    /// Returns a list of predefined PHI patterns for common sensitive data types.
    ///
    /// Each pattern includes a compiled regular expression, redaction strategy, and associated PHI type. Patterns cover SSN, Medical Record Number, ICD-10 code, Date of Birth, Indonesian NIK, and Indonesian BPJS numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// let patterns = PHIPattern::all_patterns();
    /// assert!(patterns.iter().any(|p| p.phi_type == PHIType::SSN));
    /// ```
    pub fn all_patterns() -> Vec<PHIPattern> {
        vec![
            PHIPattern {
                phi_type: PHIType::SSN,
                regex: Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap(),
                context: None,
                confidence: None,
                redaction: Redaction {
                    template: "[REDACTED]".to_string(),
                    strategy: "full".to_string(),
                },
                metadata: None,
            },
            PHIPattern {
                phi_type: PHIType::MedicalRecordNumber,
                // Example: 8-12 digit numeric, can be adjusted for real-world formats
                regex: Regex::new(r"\b\d{8,12}\b").unwrap(),
                context: None,
                confidence: None,
                redaction: Redaction {
                    template: "[REDACTED]".to_string(),
                    strategy: "full".to_string(),
                },
                metadata: None,
            },
            PHIPattern {
                phi_type: PHIType::ICD10,
                // ICD-10: Letter, 2 digits, optional "." and 1-4 more alphanums
                regex: Regex::new(r"\b[A-TV-Z][0-9]{2}(\.[A-Z0-9]{1,4})?\b").unwrap(),
                context: None,
                confidence: None,
                redaction: Redaction {
                    template: "[REDACTED]".to_string(),
                    strategy: "full".to_string(),
                },
                metadata: None,
            },
            PHIPattern {
                phi_type: PHIType::DateOfBirth,
                // MM/DD/YYYY or YYYY-MM-DD or DD/MM/YYYY (basic)
                regex: Regex::new(r"\b(\d{2}/\d{2}/\d{4}|\d{4}-\d{2}-\d{2}|\d{2}/\d{2}/\d{4})\b")
                    .unwrap(),
                context: None,
                confidence: None,
                redaction: Redaction {
                    template: "[REDACTED]".to_string(),
                    strategy: "full".to_string(),
                },
                metadata: None,
            },
            PHIPattern {
                phi_type: PHIType::IndonesianNIK,
                // NIK: 16 digits, usually no separators
                regex: Regex::new(r"\b\d{16}\b").unwrap(),
                context: None,
                confidence: None,
                redaction: Redaction {
                    template: "[REDACTED]".to_string(),
                    strategy: "full".to_string(),
                },
                metadata: None,
            },
            PHIPattern {
                phi_type: PHIType::IndonesianBPJS,
                // BPJS: 13 digits
                regex: Regex::new(r"\b\d{13}\b").unwrap(),
                context: None,
                confidence: None,
                redaction: Redaction {
                    template: "[REDACTED]".to_string(),
                    strategy: "full".to_string(),
                },
                metadata: None,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssn_pattern() {
        let pat = &PHIPattern::all_patterns()[0];
        assert_eq!(pat.phi_type, PHIType::SSN);
        assert!(pat.regex.is_match("123-45-6789"));
        assert!(!pat.regex.is_match("123456789"));
    }

    #[test]
    fn test_medical_record_pattern() {
        let pat = &PHIPattern::all_patterns()[1];
        assert_eq!(pat.phi_type, PHIType::MedicalRecordNumber);
        assert!(pat.regex.is_match("12345678"));
        assert!(pat.regex.is_match("123456789012"));
        assert!(!pat.regex.is_match("1234"));
        assert!(!pat.regex.is_match("abc12345678"));
    }

    #[test]
    fn test_icd10_pattern() {
        let pat = &PHIPattern::all_patterns()[2];
        assert_eq!(pat.phi_type, PHIType::ICD10);
        assert!(pat.regex.is_match("A12"));
        assert!(pat.regex.is_match("B99.8"));
        assert!(pat.regex.is_match("C01.123"));
        assert!(!pat.regex.is_match("123-45-6789"));
    }

    #[test]
    fn test_date_of_birth_pattern() {
        let pat = &PHIPattern::all_patterns()[3];
        assert_eq!(pat.phi_type, PHIType::DateOfBirth);
        assert!(pat.regex.is_match("12/31/2000"));
        assert!(pat.regex.is_match("2000-12-31"));
        assert!(pat.regex.is_match("31/12/2000"));
        assert!(!pat.regex.is_match("20001231"));
    }

    #[test]
    fn test_indonesian_nik_pattern() {
        let pat = &PHIPattern::all_patterns()[4];
        assert_eq!(pat.phi_type, PHIType::IndonesianNIK);
        assert!(pat.regex.is_match("1234567890123456"));
        assert!(!pat.regex.is_match("123456789012345"));
        assert!(!pat.regex.is_match("12345678901234567"));
        assert!(!pat.regex.is_match("1234-5678-9012-3456"));
    }

    #[test]
    fn test_indonesian_bpjs_pattern() {
        let pat = &PHIPattern::all_patterns()[5];
        assert_eq!(pat.phi_type, PHIType::IndonesianBPJS);
        assert!(pat.regex.is_match("1234567890123"));
        assert!(!pat.regex.is_match("123456789012"));
        assert!(!pat.regex.is_match("12345678901234"));
        assert!(!pat.regex.is_match("BPJS1234567890"));
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    /// Tests loading PHI pattern configurations from a YAML file and verifies correct deserialization.
    ///
    /// This test writes a sample YAML pattern definition to a temporary file, loads it using `from_yaml_file`,
    /// and asserts that the pattern's fields are correctly parsed.
    ///
    /// # Examples
    ///
    /// ```
    /// test_load_patterns_from_yaml();
    /// ```
    fn test_load_patterns_from_yaml() {
        let yaml = r#"
patterns:
  - id: 'ssn'
    name: 'Social Security Number'
    regex: "\\b\\d{3}-\\d{2}-\\d{4}\\b"
    redaction:
      template: '[REDACTED-SSN]'
      strategy: 'full'
"#;
        let mut tmp = NamedTempFile::new().expect("Failed to create temp file");
        write!(tmp, "{}", yaml).unwrap();
        let patterns_file =
            PHIPatternConfig::from_yaml_file(tmp.path()).expect("Failed to load YAML");
        assert_eq!(patterns_file.patterns.len(), 1);
        assert_eq!(patterns_file.patterns[0].id, "ssn");
        assert_eq!(
            patterns_file.patterns[0].redaction.template,
            "[REDACTED-SSN]"
        );
    }

    #[test]
    /// ```
    fn test_invalid_yaml() {
        let yaml = "not: valid: yaml";
        let mut tmp = NamedTempFile::new().expect("Failed to create temp file");
        write!(tmp, "{}", yaml).unwrap();
        let result = PHIPatternConfig::from_yaml_file(tmp.path());
        assert!(result.is_err());
    }
}
