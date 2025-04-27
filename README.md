# HealthWand

HealthWand is an open-source framework for detecting, redacting, masking, and anonymizing sensitive healthcare data, known as Protected Health Information (PHI), across text, images, and structured data. Designed for healthcare applications, HealthWand ensures compliance with HIPAA, GDPR, and HITRUST by identifying and protecting PHI such as Social Security Numbers (SSNs), medical IDs, ICD-10 codes, drug names, and patient information. With support for Natural Language Processing (NLP), pattern matching, and customizable pipelines, HealthWand integrates seamlessly into CI/CD pipelines and platforms, empowering developers to secure healthcare software.

## Features

- **PHI Detection**: Identifies sensitive data in text (e.g., code, commits), images (e.g., OCR-extracted text), and structured data (e.g., JSON, CSV) using regex and NLP.
- **Redaction and Anonymization**: Masks or redacts PHI (e.g., replaces SSNs with XXX-XX-XXXX) to prevent data leaks.
- **Customizable Pipelines**: Define custom PHI patterns (e.g., medical IDs, FHIR fields) and detection rules via configuration files.
- **NLP Support**: Context-aware detection to reduce false positives (e.g., distinguishes “patient SSN” from random numbers).
- **GitHub Actions Integration**: Available on the GitHub Actions Marketplace for automated PHI scanning in CI/CD pipelines.
- **High Performance**: Built in Rust for speed and memory safety, ensuring efficient scanning of large codebases.
- **Compliance-Ready**: Supports audit trails and reporting for HIPAA, GDPR, and HITRUST compliance.
- **Extensible**: Modular architecture for adding new detection methods, including future ML-based capabilities.

## Use Cases

- **Healthcare Software Development**: Scan codebases for PHI in comments, strings, or commits to prevent leaks in FHIR APIs, EHR systems, or telemedicine apps.
- **CI/CD Pipelines**: Integrate with GitHub Actions to fail builds if PHI is detected, ensuring compliance in development workflows.
- **Platform Integration**: Deploy as an API server for real-time PHI detection in healthcare platforms, with secure storage for compliance audits.
- **Open-Source Projects**: Provide a free, community-driven tool for healthcare startups to secure their applications.

## Installation

### Prerequisites

- **Rust**: Stable toolchain (1.65 or later).
- **Cargo**: Rust’s package manager.
- **Git**: For cloning the repository.

### Steps

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/your-org/healthwand.git
   cd healthwand
   ```

2. **Build HealthWand:**

   ```bash
   cargo build --release
   ```

3. **Install CLI:**

   ```bash
   cargo install --path .
   ```

4. **Verify Installation:**

   ```bash
   healthwand --version
   ```

## Usage

### CLI

Scan a file or directory for PHI using the HealthWand CLI:

```bash
healthwand scan --input src/ --config config.yaml --output phi-report.json
```

- `--input`: Path to a file or directory (e.g., `src/` for codebases).
- `--config`: Path to a YAML configuration file with PHI patterns.
- `--output`: JSON file to store detection results.

**Example output (`phi-report.json`):**

```json
[
  {
    "type": "SSN",
    "value": "123-45-6789",
    "file": "src/api.rs",
    "line": 42,
    "confidence": 0.95
  },
  {
    "type": "ICD10_CODE",
    "value": "E11.9",
    "file": "src/data.yaml",
    "line": 10,
    "confidence": 0.9
  }
]
```

### Configuration

Customize PHI detection with a `config.yaml` file:

```yaml
patterns:
  - name: SSN
    regex: '\b\d{3}-\d{2}-\d{4}\b'
    score: 0.95
  - name: ICD10_CODE
    regex: '\b[A-Z][0-9]{2}\.[0-9]{1,2}\b'
    score: 0.9
  - name: MEDICAL_ID
    regex: '\b[A-Z0-9]{8}\b'
    score: 0.85
nlp:
  model: en_core
  context: ["patient", "medical", "diagnosis"]
```

- `patterns`: Define regex-based PHI patterns with confidence scores.
- `nlp`: Specify NLP model and context keywords for improved accuracy.

## GitHub Action

Integrate HealthWand into your CI/CD pipeline using the HealthWand GitHub Action, available on the GitHub Actions Marketplace.

### Example Workflow

Create a `.github/workflows/healthcare-compliance.yml` file:

```yaml
name: Healthcare Compliance
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  workflow_dispatch:

jobs:
  phi-detection:
    runs-on: ubuntu-latest
    steps:
      # Checkout code
      - name: Checkout
        uses: actions/checkout@v3
        with:
          fetch-depth: 0

      # Set up Rust
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      # Install HealthWand
      - name: Install HealthWand
        run: cargo install --path .

      # Run PHI scan
      - name: Run PHI Scan
        run: healthwand scan --input src/ --config .github/config.yaml --output phi-report.json

      # Upload report
      - name: Upload PHI Report
        uses: actions/upload-artifact@v3
        with:
          name: phi-report
          path: phi-report.json
```

#### Action Inputs

- `config`: Path to the configuration file (default: `.github/config.yaml`).
- `fail-on-phi`: Fail the build if PHI is detected (default: `true`).

#### Outputs

- `phi-report.json`: JSON file with detected PHI, uploaded as an artifact.

## API Server

Deploy HealthWand as an API server for real-time PHI detection:

### Build and Run:

```bash
cargo run --release --bin healthwand-api
```

### API Endpoint:

```bash
curl -X POST http://localhost:8080/scan \
  -H "Content-Type: application/json" \
  -d '{"text": "Patient SSN: 123-45-6789"}'
```

**Response:**

```json
{
  "findings": [
    {
      "type": "SSN",
      "value": "123-45-6789",
      "confidence": 0.95
    }
  ]
}
```

## Repository Structure

```
healthwand/
├── src/
│   ├── analyzer/         # PHI detection logic (regex, NLP)
│   ├── anonymizer/       # Redaction and masking logic
│   ├── cli/              # Command-line interface
│   ├── api/              # API server (Actix Web)
│   ├── nlp/              # NLP processing (context-aware detection)
│   └── lib.rs            # Core library
├── config/               # Default and custom PHI patterns (config.yaml)
├── tests/                # Unit and integration tests
├── action/               # GitHub Action definition (action.yml)
├── docker/               # Dockerfile for containerized deployment
├── docs/                 # Documentation
├── .github/workflows/    # CI/CD workflows
├── README.md             # This file
├── LICENSE               # MIT License
├── Cargo.toml            # Rust dependencies and metadata
└── CHANGELOG.md          # Release notes
```

## Customization

### Adding New PHI Patterns

Modify `config/config.yaml` to include healthcare-specific patterns:

```yaml
patterns:
  - name: DRUG_NAME
    regex: '\b(lipitor|metformin)\b'
    score: 0.9
  - name: FHIR_FIELD
    regex: '\b(patient\.id|encounter\.id)\b'
    score: 0.85
```

### Extending NLP

Implement custom NLP logic in `src/nlp/` to enhance context-aware detection. For example, train models to recognize medical contexts (e.g., “diagnosis”, “prescription”).

### Custom Pipelines

Create custom detection pipelines in `src/analyzer/` to support new data types (e.g., DICOM images):

```rust
use healthwand::analyzer::Pipeline;
let pipeline = Pipeline::new()
    .add_regex_detector("SSN")
    .add_nlp_detector("en_core");
let findings = pipeline.scan("Patient SSN: 123-45-6789");
```

## Compliance

HealthWand is designed for HIPAA, GDPR, and HITRUST compliance:

- **Local Execution**: Runs locally in CI/CD to prevent PHI exposure.
- **Encryption**: Supports integration with encrypted storage (e.g., AWS S3 with AES-256).
- **Audit Trails**: Generates JSON reports for compliance audits, linking findings to code changes.
- **Security**: Built in Rust for memory safety, reducing vulnerabilities.

For HIPAA-compliant deployments:

- Deploy on a platform with a Business Associate Agreement (BAA).
- Enable encryption for data at rest and in transit.
- Maintain audit logs for all scans.

## Contributing

We welcome contributions to HealthWand! To contribute:

1. Fork the repository.
2. Create a branch:
   ```bash
   git checkout -b feature/your-feature
   ```
3. Commit changes:
   ```bash
   git commit -m "Add your feature"
   ```
4. Push to your fork:
   ```bash
   git push origin feature/your-feature
   ```
5. Open a pull request.

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines on adding PHI patterns, improving NLP, or fixing bugs.

## Community

- **GitHub Issues**: Report bugs or request features.
- **Discussions**: Join our GitHub Discussions for community support.
- **X Posts**: Follow [@ks_sha88](https://x.com/ks_sha88) on X for updates and healthcare security tips.

## Roadmap

- **v1.0 (Phase 1)**: Core PHI detection, GitHub Action, and API server.
- **v2.0 (Phase 2)**: Image processing (OCR), advanced NLP, and compliance reporting.
- **v3.0 (Phase 3)**: ML-based detection, enterprise integrations, and HITRUST certification.

## License

HealthWand is licensed under the MIT License. See [LICENSE](/LICENSE) for details.

## Contact

For support or inquiries, contact [@ks_sha88](https://x.com/ks_sha88) or open an issue on GitHub.

---

**HealthWand: Secure healthcare data, empower developers, and ensure compliance.**
