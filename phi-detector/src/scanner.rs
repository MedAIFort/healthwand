use crate::phi_patterns::{PHIPattern, PHIType};

#[derive(Debug, Clone)]
pub struct Detection {
    pub matched_text: String,
    pub start: usize,
    pub end: usize,
    pub phi_type: PHIType,
    pub confidence: f32,
    pub context: String,
}

pub struct Scanner {
    patterns: Vec<PHIPattern>,
    context_window: usize,
}

impl Scanner {
    /// Creates a new `Scanner` with the specified PHI patterns and context window size.
    ///
    /// # Examples
    ///
    /// ```
    /// let patterns = PHIPattern::all_patterns();
    /// let scanner = Scanner::new(patterns, 10);
    /// assert_eq!(scanner.context_window, 10);
    /// ```
    pub fn new(patterns: Vec<PHIPattern>, context_window: usize) -> Self {
        Self {
            patterns,
            context_window,
        }
    }

    /// Scans the input text for all configured PHI patterns and returns detected instances.
    ///
    /// For each PHI pattern, finds all matches in the text, extracts the matched substring,
    /// its byte indices, the PHI type, a fixed confidence score, and a configurable window of surrounding context.
    ///
    /// # Examples
    ///
    /// ```
    /// let patterns = PHIPattern::all_patterns();
    /// let scanner = Scanner::new(patterns, 5);
    /// let text = "Patient SSN: 123-45-6789";
    /// let detections = scanner.scan(text);
    /// assert!(detections.iter().any(|d| d.phi_type.to_string() == "SSN"));
    /// ```
    pub fn scan(&self, text: &str) -> Vec<Detection> {
        let mut detections = Vec::new();
        for pat in &self.patterns {
            for mat in pat.regex.find_iter(text) {
                let start = mat.start();
                let end = mat.end();
                let matched_text = mat.as_str().to_string();
                let context = Self::extract_context(text, start, end, self.context_window);
                let confidence = 1.0; // Placeholder, can be refined
                detections.push(Detection {
                    matched_text,
                    start,
                    end,
                    phi_type: pat.phi_type.clone(),
                    confidence,
                    context,
                });
            }
        }
        detections
    }

    /// Extracts a substring of the input text surrounding a specified range, including a given number of characters before and after the range.
    ///
    /// The context window extends up to `window` characters before `start` and after `end`, without exceeding the text boundaries.
    ///
    /// # Examples
    ///
    /// ```
    /// let text = "Sensitive: 123-45-6789 is an SSN.";
    /// let context = extract_context(text, 11, 22, 5);
    /// assert_eq!(context, "tive: 123-45-6789 is a");
    /// ```
    fn extract_context(text: &str, start: usize, end: usize, window: usize) -> String {
        let left = start.saturating_sub(window);
        let right = usize::min(text.len(), end + window);
        text[left..right].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phi_patterns::{PHIPattern, PHIType};

    fn get_test_patterns() -> Vec<PHIPattern> {
        PHIPattern::all_patterns()
    }

    #[test]
    fn test_scanner_detects_phi() {
        let scanner = Scanner::new(get_test_patterns(), 5);
        let text = "Patient SSN: 123-45-6789. MRN: 123456789. ICD: A12.34. NIK: 1234567890123456. BPJS: 1234567890123. Date: 31/12/2000.";
        let results = scanner.scan(text);
        assert!(results.iter().any(|d| d.phi_type == PHIType::SSN));
        assert!(
            results
                .iter()
                .any(|d| d.phi_type == PHIType::MedicalRecordNumber)
        );
        assert!(results.iter().any(|d| d.phi_type == PHIType::ICD10));
        assert!(results.iter().any(|d| d.phi_type == PHIType::IndonesianNIK));
        assert!(
            results
                .iter()
                .any(|d| d.phi_type == PHIType::IndonesianBPJS)
        );
        assert!(results.iter().any(|d| d.phi_type == PHIType::DateOfBirth));
    }

    #[test]
    fn test_context_extraction() {
        let text = "abcdefg 123-45-6789 xyz";
        let scanner = Scanner::new(get_test_patterns(), 3);
        let results = scanner.scan(text);
        for d in &results {
            println!(
                "Detected: {:?} at {}-{}: {}",
                d.phi_type, d.start, d.end, d.matched_text
            );
        }
        assert!(
            !results.is_empty(),
            "No PHI detected in test_context_extraction. Patterns: {:?}",
            get_test_patterns()
        );
        let ssn = results.iter().find(|d| d.phi_type == PHIType::SSN).unwrap();
        // Should include 3 chars before and after the match
        assert!(ssn.context.contains("fg 123-45-6789 x"));
    }
}
