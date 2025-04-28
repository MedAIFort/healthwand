use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PHIType {
    SSN,
    MedicalRecordNumber,
    ICD10,
    DateOfBirth,
    IndonesianNIK, // Indonesian National ID Number
    IndonesianBPJS, // BPJS Health Insurance Number
}

pub struct PHIPattern {
    pub phi_type: PHIType,
    pub regex: Regex,
}

impl PHIPattern {
    pub fn all_patterns() -> Vec<PHIPattern> {
        vec![
            PHIPattern {
                phi_type: PHIType::SSN,
                regex: Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap(),
            },
            PHIPattern {
                phi_type: PHIType::MedicalRecordNumber,
                // Example: 8-12 digit numeric, can be adjusted for real-world formats
                regex: Regex::new(r"\b\d{8,12}\b").unwrap(),
            },
            PHIPattern {
                phi_type: PHIType::ICD10,
                // ICD-10: Letter, 2 digits, optional "." and 1-4 more alphanums
                regex: Regex::new(r"\b[A-TV-Z][0-9]{2}(\.[A-Z0-9]{1,4})?\b").unwrap(),
            },
            PHIPattern {
                phi_type: PHIType::DateOfBirth,
                // MM/DD/YYYY or YYYY-MM-DD or DD/MM/YYYY (basic)
                regex: Regex::new(r"\b(\d{2}/\d{2}/\d{4}|\d{4}-\d{2}-\d{2})\b").unwrap(),
            },
            PHIPattern {
                phi_type: PHIType::IndonesianNIK,
                // NIK: 16 digits, usually no separators
                regex: Regex::new(r"\b\d{16}\b").unwrap(),
            },
            PHIPattern {
                phi_type: PHIType::IndonesianBPJS,
                // BPJS: 13 digits
                regex: Regex::new(r"\b\d{13}\b").unwrap(),
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
