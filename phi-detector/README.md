# PHI Detector

A Rust CLI tool for detecting and redacting Protected Health Information (PHI) in text files.

## Features

- Detects multiple PHI types (SSN, Medical Record Number, ICD10, Date of Birth, Indonesian NIK, BPJS)
- Outputs results in JSON or human-readable text format
- Supports redaction with multiple strategies
- Structured logging with configurable verbosity
- Robust error handling and summary statistics

## Output Format

Detection results and summary statistics are output in structured JSON or pretty-printed text. See [`docs/output_format.md`](docs/output_format.md) for the detailed schema and examples.

## Example Usage

```sh
phi-detector --input data/ --output json --redact -vv
```

## Documentation

- [Output Format](docs/output_format.md): JSON schema and field descriptions for detection results and summary.

## Logging

- Controlled by `--verbose` flag (`-v`, `-vv`, etc.)
- Logs info, warnings, and errors to the console

## Error Handling

- Custom error types and helpful messages for file I/O, serialization, scanning, and redaction errors
- Errors are reported in the summary output

## Integration Tests

- See `tests/integration_pipeline.rs` for end-to-end pipeline tests

## YAML Configuration (Custom PHI Patterns)

PHI patterns can be defined and extended using a YAML configuration file. This enables you to add, remove, or modify detection rules without changing Rust code.

### Example: `config/phi_patterns.yaml`

```yaml
patterns:
  - id: "ssn"
    name: "Social Security Number"
    regex: "\\b\\d{3}-\\d{2}-\\d{4}\\b"
    redaction:
      template: "[REDACTED-SSN]"
      strategy: "full"
  - id: "indonesian_nik"
    name: "Indonesian NIK"
    regex: "\\b\\d{16}\\b"
    redaction:
      template: "[REDACTED-NIK]"
      strategy: "full"
# ... see full schema in config/phi_patterns.yaml
```

### Loading Patterns from YAML

You can load PHI patterns from a YAML file at runtime:

```rust
use phi_detector::phi_patterns::PHIPatternConfig;

let patterns_file = PHIPatternConfig::from_yaml_file("config/phi_patterns.yaml")?;
for pat in patterns_file.patterns {
    println!("Loaded pattern: {} - {}", pat.id, pat.name);
}
```

- The loader validates the YAML structure and returns an error for invalid files.
- See tests in `src/phi_patterns.rs` for usage examples.

### Testing Configuration Loading

Run the Rust tests to validate YAML loading and error handling:

```sh
cargo test --lib -- phi_patterns::config_tests
```

- Tests cover loading valid YAML, handling invalid YAML, and basic pattern field checks.

---

For more details on usage, options, and contributing, see CLI help (`phi-detector --help`) or open an issue.
