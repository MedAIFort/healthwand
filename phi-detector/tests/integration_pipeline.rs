use phi_detector::phi_patterns::PHIType;
use phi_detector::redactor::{RedactionStrategy, Redactor};
use phi_detector::results::{DetectionResult, ResultsSummary};
use phi_detector::scanner::Scanner;

#[test]
fn test_full_pipeline_json_output() {
    let text = "SSN: 123-45-6789, MRN: 12345678, NIK: 1234567890123456";
    let scanner = Scanner::new(phi_detector::phi_patterns::PHIPattern::all_patterns(), 10);
    let detections = scanner.scan(text);
    let redactor = Redactor::new(RedactionStrategy::FullReplacement);
    let redacted = redactor.redact(text, &detections);
    let mut results = Vec::new();
    for det in &detections {
        results.push(DetectionResult {
            file_path: "test.txt".to_string(),
            phi_type: det.phi_type.clone(),
            location: (det.start, det.end),
            context: det.context.clone(),
            matched_text: det.matched_text.clone(),
            redacted_text: Some(redacted[det.start..det.end].to_string()),
        });
    }
    let json = serde_json::to_string_pretty(&results).unwrap();
    assert!(json.contains("SSN"));
    assert!(json.contains("MedicalRecordNumber"));
    assert!(json.contains("IndonesianNIK"));
}

#[test]
fn test_pipeline_summary() {
    let text = "SSN: 123-45-6789, SSN: 987-65-4321";
    let scanner = Scanner::new(phi_detector::phi_patterns::PHIPattern::all_patterns(), 10);
    let detections = scanner.scan(text);
    let mut summary = ResultsSummary::default();
    for det in &detections {
        *summary
            .detections_by_type
            .entry(det.phi_type.clone())
            .or_insert(0) += 1;
    }
    summary.files_processed += 1;
    summary.total_detections += detections.len();
    summary.redacted_count += detections.len();
    assert_eq!(summary.files_processed, 1);
    assert_eq!(summary.total_detections, 2);
    assert_eq!(summary.redacted_count, 2);
    assert!(summary.detections_by_type.contains_key(&PHIType::SSN));
}
