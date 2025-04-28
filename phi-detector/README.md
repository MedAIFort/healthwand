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

---

For more details on usage, options, and contributing, see CLI help (`phi-detector --help`) or open an issue.
