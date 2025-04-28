use serde::{Serialize, Deserialize};
use crate::phi_patterns::PHIType;

/// Represents a single PHI detection result, suitable for JSON output.
///
/// # Fields
/// - `file_path`: The file where the detection was found.
/// - `phi_type`: The type of PHI detected (e.g., SSN, MRN).
/// - `location`: A tuple (start, end) indicating the byte offsets of the match in the file.
/// - `context`: A snippet of text surrounding the match for context.
/// - `matched_text`: The exact text that matched the PHI pattern.
/// - `redacted_text`: The redacted version of the matched text (if redaction is enabled).
#[derive(Debug, Serialize, Deserialize)]
pub struct DetectionResult {
    pub file_path: String,
    pub phi_type: PHIType,
    pub location: (usize, usize),
    pub context: String,
    pub matched_text: String,
    pub redacted_text: Option<String>,
}

/// Summary statistics for a PHI detection run.
///
/// # Fields
/// - `files_processed`: Number of files scanned.
/// - `total_detections`: Total number of PHI detections.
/// - `detections_by_type`: Map from PHI type to count of detections.
/// - `redacted_count`: Number of redactions performed.
/// - `errors`: List of error messages encountered during processing.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResultsSummary {
    pub files_processed: usize,
    pub total_detections: usize,
    pub detections_by_type: std::collections::HashMap<PHIType, usize>,
    pub redacted_count: usize,
    pub errors: Vec<String>,
}

/// Combined output for JSON serialization: detection results and summary.
#[derive(Debug, Serialize, Deserialize)]
pub struct OutputBundle {
    pub results: Vec<DetectionResult>,
    pub summary: ResultsSummary,
}
