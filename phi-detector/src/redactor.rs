use crate::scanner::Detection;
use crate::phi_patterns::PHIType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedactionStrategy {
    FullReplacement,         // e.g., XXX-XX-XXXX
    PartialMasking,         // e.g., ***-**-1234
    PlaceholderSubstitution // e.g., [REDACTED-SSN]
}

pub struct Redactor {
    pub strategy: RedactionStrategy,
}

impl Redactor {
    pub fn new(strategy: RedactionStrategy) -> Self {
        Self { strategy }
    }

    pub fn redact(&self, text: &str, detections: &[Detection]) -> String {
        let mut redacted = String::with_capacity(text.len());
        let mut last = 0;
        let mut sorted = detections.to_vec();
        sorted.sort_by_key(|d| d.start);

        for det in sorted {
            if det.start >= last {
                // Non-overlapping span – redact normally
                redacted.push_str(&text[last..det.start]);
                redacted.push_str(&self.redaction_text(&det.phi_type, &det.matched_text));
                last = det.end;
            } else if det.end > last {
                // Overlaps but extends further – make sure the tail gets redacted
                redacted.push_str(&self.redaction_text(
                    &det.phi_type,
                    // use the uncovered slice for length parity
                    &det.matched_text[(det.matched_text.len() - (det.end - last))..]
                ));
                last = det.end;
            }
            // Skip if this detection overlaps with an already processed one
            // (start position is before the end of the last processed detection)
        }
        redacted.push_str(&text[last..]);
        redacted
    }

    fn redaction_text(&self, phi_type: &PHIType, matched: &str) -> String {
        match self.strategy {
            RedactionStrategy::FullReplacement => match phi_type {
                PHIType::SSN => "XXX-XX-XXXX".to_string(),
                PHIType::MedicalRecordNumber => "X".repeat(matched.len()),
                PHIType::ICD10 => "X".repeat(matched.len()),
                PHIType::DateOfBirth => "XX/XX/XXXX".to_string(),
                PHIType::IndonesianNIK => "XXXXXXXXXXXXXXXX".to_string(),
                PHIType::IndonesianBPJS => "XXXXXXXXXXXXX".to_string(),
            },
            RedactionStrategy::PartialMasking => match phi_type {
                PHIType::SSN => {
                    // Preserve SSN format: ***-**-1234
                    if matched.len() == 11 && matched.chars().nth(3) == Some('-') && matched.chars().nth(6) == Some('-') {
                        format!("***-**-{}", &matched[7..])
                    } else {
                        // fallback: mask all but last 4
                        let len = matched.len();
                        if len > 4 {
                            format!("{}{}", "*".repeat(len-4), &matched[len-4..])
                        } else {
                            "*".repeat(len)
                        }
                    }
                },
                PHIType::MedicalRecordNumber => {
                    let len = matched.len();
                    if len > 4 {
                        format!("{}{}", "*".repeat(len-4), &matched[len-4..])
                    } else {
                        "*".repeat(len)
                    }
                },
                PHIType::ICD10 => {
                    // Keep only the first character visible
                    if !matched.is_empty() {
                        format!("{}{}", &matched[0..1], "*".repeat(matched.len() - 1))
                    } else {
                        "*".repeat(matched.len())
                    }
                },
                PHIType::DateOfBirth => {
                    // Show only the year part
                    if matched.len() >= 4 {
                        if matched.contains('/') || matched.contains('-') {
                            let parts: Vec<&str> =
                                matched.split(|c| c == '/' || c == '-').collect();
                            if parts.len() >= 3 {
                                let year_part =
                                    if parts[0].len() == 4 { parts[0] } else { parts[2] };
                                format!("**/**/{}", year_part)
                            } else {
                                "*".repeat(matched.len())
                            }
                        } else {
                            "*".repeat(matched.len())
                        }
                    } else {
                        "*".repeat(matched.len())
                    }
                },
                PHIType::IndonesianNIK => {
                    if matched.len() > 8 {
                        format!(
                            "{}{}",
                            "*".repeat(matched.len() - 8),
                            &matched[matched.len() - 8..]
                        )
                    } else {
                        "*".repeat(matched.len())
                    }
                },
                PHIType::IndonesianBPJS => {
                    if matched.len() > 6 {
                        format!(
                            "{}{}",
                            "*".repeat(matched.len() - 6),
                            &matched[matched.len() - 6..]
                        )
                    } else {
                        "*".repeat(matched.len())
                    }
                },
            },
            RedactionStrategy::PlaceholderSubstitution => match phi_type {
                PHIType::SSN => "[REDACTED-SSN]".to_string(),
                PHIType::MedicalRecordNumber => "[REDACTED-MRN]".to_string(),
                PHIType::ICD10 => "[REDACTED-ICD10]".to_string(),
                PHIType::DateOfBirth => "[REDACTED-DOB]".to_string(),
                PHIType::IndonesianNIK => "[REDACTED-NIK]".to_string(),
                PHIType::IndonesianBPJS => "[REDACTED-BPJS]".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;
    use crate::phi_patterns::PHIPattern;

    #[test]
    fn test_full_replacement() {
        let text = "SSN: 123-45-6789, MRN: 12345678";
        let scanner = Scanner::new(PHIPattern::all_patterns(), 0);
        let detections = scanner.scan(text);
        let redactor = Redactor::new(RedactionStrategy::FullReplacement);
        let result = redactor.redact(text, &detections);
        assert_eq!(result, "SSN: XXX-XX-XXXX, MRN: XXXXXXXXXXXX");
    }

    #[test]
    fn test_partial_masking() {
        let text = "SSN: 123-45-6789, MRN: 12345678";
        let scanner = Scanner::new(PHIPattern::all_patterns(), 0);
        let detections = scanner.scan(text);
        let redactor = Redactor::new(RedactionStrategy::PartialMasking);
        let result = redactor.redact(text, &detections);
        assert_eq!(result, "SSN: ***-**-6789, MRN: ****5678");
    }

    #[test]
    fn test_placeholder_substitution() {
        let text = "SSN: 123-45-6789, MRN: 12345678";
        let scanner = Scanner::new(PHIPattern::all_patterns(), 0);
        let detections = scanner.scan(text);
        let redactor = Redactor::new(RedactionStrategy::PlaceholderSubstitution);
        let result = redactor.redact(text, &detections);
        assert_eq!(result, "SSN: [REDACTED-SSN], MRN: [REDACTED-MRN]");
    }

    #[test]
    fn test_overlapping_matches() {
        let text = "NIK: 1234567890123456, BPJS: 1234567890123";
        let scanner = Scanner::new(PHIPattern::all_patterns(), 0);
        let detections = scanner.scan(text);
        let redactor = Redactor::new(RedactionStrategy::FullReplacement);
        let result = redactor.redact(text, &detections);
        assert_eq!(result, "NIK: XXXXXXXXXXXXXXXX, BPJS: XXXXXXXXXXXXX");
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let scanner = Scanner::new(PHIPattern::all_patterns(), 0);
        let detections = scanner.scan(text);
        let redactor = Redactor::new(RedactionStrategy::FullReplacement);
        let result = redactor.redact(text, &detections);
        assert_eq!(result, "");
    }

    #[test]
    fn test_no_detections() {
        let text = "This text contains no PHI patterns.";
        let empty_detections: Vec<crate::scanner::Detection> = vec![];
        let redactor = Redactor::new(RedactionStrategy::FullReplacement);
        let result = redactor.redact(text, &empty_detections);
        assert_eq!(result, text);
    }

    #[test]
    fn test_detection_at_start() {
        let text = "123-45-6789 is a SSN at the start.";
        let scanner = Scanner::new(PHIPattern::all_patterns(), 0);
        let detections = scanner.scan(text);
        let redactor = Redactor::new(RedactionStrategy::FullReplacement);
        let result = redactor.redact(text, &detections);
        assert_eq!(result, "XXX-XX-XXXX is a SSN at the start.");
    }

    #[test]
    fn test_detection_at_end() {
        let text = "SSN at the end: 123-45-6789";
        let scanner = Scanner::new(PHIPattern::all_patterns(), 0);
        let detections = scanner.scan(text);
        let redactor = Redactor::new(RedactionStrategy::FullReplacement);
        let result = redactor.redact(text, &detections);
        assert_eq!(result, "SSN at the end: XXX-XX-XXXX");
    }

    #[test]
    fn test_partial_masking_short_mrn() {
        let text = "Short MRN: 1234";
        let scanner = Scanner::new(PHIPattern::all_patterns(), 0);
        let detections = scanner.scan(text);
        let redactor = Redactor::new(RedactionStrategy::PartialMasking);
        let result = redactor.redact(text, &detections);
        // Should not mask, as MRN pattern requires 8-12 digits; expect original text
        assert_eq!(result, "Short MRN: 1234");
    }
}
