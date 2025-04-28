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
                redacted.push_str(&text[last..det.start]);
                let replacement = self.redaction_text(&det.phi_type, &det.matched_text);
                redacted.push_str(&replacement);
                last = det.end;
            }
            // If overlaps, skip (already handled by previous match)
        }
        redacted.push_str(&text[last..]);
        redacted
    }

    fn redaction_text(&self, phi_type: &PHIType, matched: &str) -> String {
        match self.strategy {
            RedactionStrategy::FullReplacement => match phi_type {
                PHIType::SSN => "XXX-XX-XXXX".to_string(),
                PHIType::MedicalRecordNumber => "[REDACTED-MRN]".to_string(),
                PHIType::ICD10 => "[REDACTED-ICD10]".to_string(),
                PHIType::DateOfBirth => "[REDACTED-DOB]".to_string(),
                PHIType::IndonesianNIK => "[REDACTED-NIK]".to_string(),
                PHIType::IndonesianBPJS => "[REDACTED-BPJS]".to_string(),
            },
            RedactionStrategy::PartialMasking => match phi_type {
                PHIType::SSN => {
                    if matched.len() >= 4 {
                        let visible_part = &matched[matched.len() - 4..];
                        let mask_length = matched.len() - 4;
                        format!("{}{}", "*".repeat(mask_length), visible_part)
                    } else {
                        "*".repeat(matched.len())
                    }
                },
                PHIType::MedicalRecordNumber => {
                    let len = matched.len();
                    if len > 4 {
                        format!("{}{}", "*".repeat(len - 4), &matched[len - 4..])
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
        assert_eq!(result, "SSN: XXX-XX-XXXX, MRN: [REDACTED-MRN]");
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
        assert_eq!(result, "NIK: [REDACTED-NIK], BPJS: [REDACTED-BPJS]");
    }
}
