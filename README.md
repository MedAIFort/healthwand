<div align="center">

# HealthWand

![Rust](https://img.shields.io/badge/Rust-1.65%2B-orange?logo=rust)
![Python](https://img.shields.io/badge/Python-3.9%2B-brightgreen?logo=python)
![Status](https://img.shields.io/badge/status-development-yellow)

![CodeRabbit Pull Request Reviews](https://img.shields.io/coderabbit/prs/github/MedAIFort/healthwand?utm_source=oss&utm_medium=github&utm_campaign=MedAIFort%2Fhealthwand&labelColor=171717&color=FF570A&link=https%3A%2F%2Fcoderabbit.ai&label=CodeRabbit+Reviews)
[![codecov](https://codecov.io/gh/MedAIFort/healthwand/branch/main/graph/badge.svg)](https://codecov.io/gh/MedAIFort/healthwand)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

</div>

**HealthWand** is an open-source framework for detecting, redacting, masking, and anonymizing sensitive healthcare data, known as **Protected Health Information (PHI)**, across text, images, and structured data. Designed for healthcare applications and built with **Rust** for high-performance pattern matching and **Python** for context-aware Natural Language Processing (NLP), HealthWand ensures compliance with **HIPAA**, **GDPR**, and **HITRUST** standards by identifying and protecting PHI such as **Social Security Numbers (SSNs)**, **Medical IDs**, **ICD-10 codes**, **Drug names**, and **Patient information**. With support for Natural Language Processing (NLP), pattern matching, and customizable pipelines, HealthWand integrates seamlessly into CI/CD pipelines and platforms, empowering developers to secure healthcare software. 

<!-- Available on the **GitHub Actions Marketplace**, it empowers developers to secure healthcare applications like FHIR APIs, EHR systems, and telemedicine platforms. -->

## Features

- **PHI Detection**: Identifies sensitive data (e.g., SSNs, ICD-10 codes, FHIR fields, drug names) in code, commits, images (via OCR), and structured data (e.g., JSON, CSV).
- **Redaction and Anonymization**: Masks or redacts PHI (e.g., "123-45-6789" → "XXX-XX-XXXX") to prevent data leaks.
- **Customizable Pipelines**: Define custom PHI patterns and detection rules via YAML configuration.
- **Context-Aware NLP**: Uses Python-based NLP to reduce false positives (e.g., distinguishes "patient SSN" from random numbers).
- **GitHub Actions Integration**: Automates PHI scanning in CI/CD pipelines, available on the GitHub Actions Marketplace.
- **High Performance**: Rust ensures fast scanning of large codebases, with Python enhancing NLP accuracy.
- **Compliance-Ready**: Supports audit trails and reporting for healthcare regulations.
- **Extensible**: Modular Rust and Python architecture for adding new detection methods, including future AI capabilities.

## Use Cases

- **Healthcare Software Development**: Scan codebases for PHI to ensure compliance in FHIR APIs or EHR systems.
- **CI/CD Pipelines**: Fail builds if PHI is detected, protecting healthcare repos during development.
- **Platform Integration**: Deploy as an API for real-time PHI detection in healthcare platforms.
- **Open-Source Projects**: Provide a free tool for startups to secure healthcare data cost-effectively.

## Installation

### Prerequisites
- **Rust**: Stable toolchain (1.65 or later) with Cargo.
- **Python**: Version 3.9 or later.
- **Git**: For cloning the repository.

### Steps
1. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-org/healthwand.git
   cd healthwand
   ```
2. **Build Rust CLI:**
   ```bash
   cargo build --release
   cargo install --path .
   ```
3. **Install Python NLP:**
   ```bash
   pip install -r python/requirements.txt
   ```
4. **Verify Installation:**
   ```bash
   healthwand --version
   healthwand-nlp --version
   ```

## Usage

### CLI
Scan a file or directory for PHI using the HealthWand CLI:

```bash
healthwand scan --input src/ --config config.yaml --output phi-report.json
healthwand-nlp validate --input phi-report.json --output validated-phi-report.json
```
- `--input`: Path to a file or directory (e.g., `src/` for codebases).
- `--config`: YAML configuration file with PHI patterns.
- `--output`: JSON file for detection results.

Example output (`phi-report.json`):

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

## Configuration
Customize PHI detection with a `config.yaml` file:

```yaml
patterns:
  - name: SSN
    regex: '\b\d{3}-\d{2}-\d{4}\b'
    score: 0.95
  - name: ICD10_CODE
    regex: '\b[A-Z][0-9]{2}\.[0-9]{1,2}\b'
    score: 0.9
  - name: FHIR_FIELD
    regex: '\b(patient\.id|encounter\.id)\b'
    score: 0.85
nlp:
  model: en_core_sci_sm
  context: ["patient", "medical", "diagnosis"]
```

## GitHub Action
Integrate HealthWand into CI/CD pipelines using the HealthWand GitHub Action, available on the GitHub Actions Marketplace.

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
      - name: Checkout
        uses: actions/checkout@v3
        with:
          fetch-depth: 0
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install Python
        uses: actions/setup-python@v4
        with:
          python-version: '3.9'
      - name: Install dependencies
        run: |
          pip install -r python/requirements.txt
      - name: Build HealthWand CLI
        run: |
          cargo build --release
      - name: Run PHI Scan
        run: |
          healthwand scan --input src/ --config config.yaml --output phi-report.json
      - name: Validate with NLP
        run: |
          healthwand-nlp validate --input phi-report.json --output validated-phi-report.json
      - name: Upload Report
        uses: actions/upload-artifact@v3
        with:
          name: phi-report
          path: validated-phi-report.json
```

- **fail-on-phi**: Fail the build if PHI is detected (default: true).

**Outputs:**
- `phi-report.json`: JSON file with detected PHI, uploaded as an artifact.

## API Server
Deploy HealthWand as a Python-based API server for real-time PHI detection:

**Run API:**
```bash
cd python
uvicorn api.main:app --host 0.0.0.0 --port 8080
```

**API Endpoint:**
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

## Project Structure
```
healthwand/
├── src/                 # Rust source code
│   ├── analyzer/        # Detection pipelines
│   ├── cli/             # Command-line interface
│   ├── lib.rs           # Core library
├── python/              # Python source code
│   ├── nlp/             # NLP processing
│   ├── api/             # API server
│   ├── requirements.txt # Python dependencies
├── config/              # PHI patterns (config.yaml)
├── action/              # GitHub Action definition (action.yml)
├── tests/               # Unit and integration tests
├── docker/              # Dockerfile for CLI and API
├── docs/                # Documentation
├── .github/workflows/   # CI/CD workflows
├── README.md            # This file
├── LICENSE              # MIT License
├── Cargo.toml           # Rust dependencies
└── CHANGELOG.md         # Release notes
```

## Customization

### Adding PHI Patterns
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
Customize NLP in `python/nlp/` to enhance context-aware detection, such as recognizing medical terms or FHIR-specific entities.

### Custom Pipelines
Create detection pipelines in `src/analyzer/` for new data types:

```rust
use healthwand::analyzer::Pipeline;
let pipeline = Pipeline::new()
    .add_regex_detector("SSN")
    .add_nlp_validator("en_core_sci_sm");
let findings = pipeline.scan("Patient SSN: 123-45-6789");
```

## Compliance
HealthWand supports HIPAA, GDPR, and HITRUST compliance:

- **Local Execution**: Runs locally in CI/CD to prevent PHI exposure.
- **Encryption**: Integrates with encrypted storage (e.g., AWS S3 with AES-256).
- **Audit Trails**: Generates JSON reports for compliance audits.
- **Security**: Rust’s memory safety and Python’s robust NLP reduce vulnerabilities.

For HIPAA-compliant deployments:
- Use a platform with a Business Associate Agreement (BAA).
- Enable encryption for data at rest and in transit.
- Maintain audit logs for all scans.

<!-- Premium compliance features, such as advanced reporting and HITRUST certification, are available via the MedAIFort dashboard ([https://medaifort.com](https://medaifort.com)). -->

## Contributing
We welcome contributions to HealthWand! To contribute:

1. Fork the repository.
2. Create a branch: `git checkout -b feature/your-feature`
3. Commit changes: `git commit -m "Add your feature"`
4. Push to your fork: `git push origin feature/your-feature`
5. Open a pull request.

See `CONTRIBUTING.md` for guidelines on adding PHI patterns, optimizing Rust code, or enhancing Python NLP.

## Community
- **GitHub Issues**: Report bugs or request features.
- **Discussions**: Join GitHub Discussions for community support.
- **X Posts**: Follow @ks_sha888 on X for updates and healthcare security insights.

## Roadmap
HealthWand evolves to secure healthcare data. This roadmap is tentative and subject to community feedback.

- **v1.0 (Phase 1)**: Core PHI detection (e.g., SSNs, ICD-10 codes, FHIR fields), GitHub Action for CI/CD, and API server for platform integration.
- **v2.0 (Phase 2)**: Image processing (OCR for scanned records), advanced NLP for context-aware detection, and compliance reporting for HIPAA/GDPR.
- **v3.0 (Phase 3)**: Machine learning-based PHI detection, enterprise integrations (e.g., EHR platforms), and HITRUST certification.

*Note: We welcome feedback via GitHub Issues or Discussions to shape HealthWand’s future.*

## License
HealthWand is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contact
For support or inquiries, contact @ks_sha888 on X or open an issue on GitHub.

---

*HealthWand: Secure healthcare data, empower developers, ensure compliance.*