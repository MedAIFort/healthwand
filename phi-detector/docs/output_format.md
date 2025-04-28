# PHI Detector Output Format

This document describes the output format for the PHI Detector CLI.

## DetectionResult JSON Schema

Each detection result is represented as a JSON object with the following fields:

- `file_path` (string): Path to the file where the PHI was detected.
- `phi_type` (string): The type of PHI detected (e.g., SSN, MedicalRecordNumber, ICD10, DateOfBirth, IndonesianNIK, IndonesianBPJS).
- `location` (array of two integers): The byte offsets (start, end) of the detected PHI in the file/text.
- `context` (string): A snippet of text surrounding the match for context.
- `matched_text` (string): The exact text that matched the PHI pattern.
- `redacted_text` (string, optional): The redacted version of the matched text, if redaction is enabled.

Example:
```json
{
  "file_path": "data/example.txt",
  "phi_type": "SSN",
  "location": [10, 21],
  "context": "...123-45-6789...",
  "matched_text": "123-45-6789",
  "redacted_text": "XXX-XX-XXXX"
}
```

## ResultsSummary JSON Schema

A summary object is also output with the following fields:

- `files_processed` (integer): Number of files scanned.
- `total_detections` (integer): Total number of PHI detections found.
- `detections_by_type` (object): Map of PHI type to the number of detections.
- `redacted_count` (integer): Number of redactions performed.
- `errors` (array of strings): List of error messages encountered during processing.

Example:
```json
{
  "files_processed": 2,
  "total_detections": 3,
  "detections_by_type": { "SSN": 2, "MedicalRecordNumber": 1 },
  "redacted_count": 3,
  "errors": []
}
```

## Output Modes

- **JSON**: All detection results are output as a JSON array, followed by a summary object.
- **Text**: Results are printed in a human-readable format to the console.

## Logging

- Logging is controlled by the `--verbose` flag and supports multiple levels (info, warn, error).
- Errors and warnings are logged to the console and included in the summary.

---

For more details on CLI usage and configuration, see the project README.
