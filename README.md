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

> Note: The repository currently provides the Rust CLI `phi-detector` (under `phi-detector/`) for PHI scanning and redaction. Additional components (e.g., Python NLP validator and API server) are planned and tracked in the roadmap.

## Features

- **PHI Detection**: Identifies sensitive data (e.g., SSNs, ICD-10 codes, FHIR fields, drug names) in code, commits, images (via OCR), and structured data (e.g., JSON, CSV).
- **Redaction and Anonymization**: Masks or redacts PHI (e.g., "123-45-6789" → "XXX-XX-XXXX") to prevent data leaks.
- **Customizable Pipelines**: Define custom PHI patterns and detection rules via YAML configuration.
- **Context-Aware NLP (planned)**: Uses Python-based NLP to reduce false positives (e.g., distinguishes "patient SSN" from random numbers).
- **GitHub Actions Integration**: Automates PHI scanning in CI/CD pipelines, available on the GitHub Actions Marketplace.
- **High Performance**: Rust ensures fast scanning of large codebases, with Python enhancing NLP accuracy.
- **Compliance-Ready**: Supports audit trails and reporting for healthcare regulations.
- **Extensible**: Modular Rust and Python architecture for adding new detection methods, including future AI capabilities.
- **Regional Support (Indonesia)**: Built-in YAML patterns for Indonesian identifiers, including **KTP/NIK (16-digit)** and **BPJS (13-digit)** numbers.

## Use Cases

- **Healthcare Software Development**: Scan codebases for PHI to ensure compliance in FHIR APIs or EHR systems.
- **CI/CD Pipelines**: Fail builds if PHI is detected, protecting healthcare repos during development.
- **Platform Integration**: Deploy as an API for real-time PHI detection in healthcare platforms.
- **Open-Source Projects**: Provide a free tool for startups to secure healthcare data cost-effectively.

## Installation

### Prerequisites
- **Rust**: Stable toolchain (1.65 or later) with Cargo.
- **Python**: Version 3.9 or later (optional; NLP validator is planned).
- **Git**: For cloning the repository.

### Steps
1. **Clone the Repository**:
   ```bash
   git clone https://github.com/MedAIFort/healthwand.git
   cd healthwand
   ```
2. **Build Rust CLI (`phi-detector`):**
   ```bash
   cd phi-detector
   cargo build --release
   cargo install --path .
   ```
3. **Verify Installation:**
   ```bash
   phi-detector --help
   ```

## Usage

### CLI (phi-detector)
Scan files or directories for PHI using the Rust CLI:

```bash
phi-detector --input ./docs --output json --redact -vv
```
- `--input`: Path to a file or directory to scan.
- `--output`: `json` (structured bundle) or `text`.
- `--redact`: Enable in-line redaction in outputs.
- `-v`/`-vv`: Increase verbosity for logs.

By default, the CLI scans text-like files with extensions: `.txt`, `.md`, `.csv`.

See detailed JSON schema in `phi-detector/docs/output_format.md`.

#### Example: Indonesian identifiers

```bash
echo "NIK: 1234567890123456, BPJS: 1234567890123" > sample.txt
phi-detector --input ./sample.txt --output text
```

## Configuration
Customize PHI detection using YAML patterns (example provided at `phi-detector/config/phi_patterns.yaml`):

```yaml
patterns:
  - name: SSN
    regex: '\b\d{3}-\d{2}-\d{4}\b'
    score: 0.95
  - name: Indonesian NIK (KTP)
    regex: '\b\d{16}\b'
    score: 0.95
  - name: Indonesian BPJS
    regex: '\b\d{13}\b'
    score: 0.93
# nlp configuration is planned and will be added when the NLP validator is integrated
```

Note: The current CLI uses built-in patterns; YAML loading is available in the library and planned for the CLI.

## Regional Support: Indonesia (BPJS, KTP/NIK)

- **KTP/NIK (Nomor Induk Kependudukan)**: 16-digit national ID. Detected via `indonesian_nik` pattern with KTP/NIK context words.
- **BPJS Kesehatan**: 13-digit health insurance number. Detected via `indonesian_bpjs` pattern with BPJS context words.
- See `phi-detector/config/phi_patterns.yaml` for configurable regexes, confidence, redaction templates, and examples.

## GitHub Action
Integrate `phi-detector` into CI/CD to block PHI from entering your repo history.

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
        uses: dtolnay/rust-toolchain@stable
      - name: Build phi-detector
        run: |
          cd phi-detector
          cargo build --release
      - name: Run PHI Scan
        run: |
          cd phi-detector
          ./target/release/phi-detector --input . --output json > ../phi-report.json
      - name: Upload Report
        uses: actions/upload-artifact@v3
        with:
          name: phi-report
          path: phi-report.json
```

**Outputs:**
- `phi-report.json`: JSON file with detected PHI, uploaded as an artifact.

## API Server (Planned)
A Python-based API server for real-time PHI detection is planned. For now, use the Rust CLI in CI or local workflows.

## Project Structure
```
healthwand/
├── phi-detector/        # Rust CLI crate for PHI detection & redaction
│   ├── src/             # CLI + library modules
│   ├── config/          # YAML patterns (e.g., Indonesian NIK/BPJS)
│   ├── docs/            # Output format docs
│   └── tests/           # Integration tests
├── docs/                # Repository documentation
├── scripts/             # Project scripts
├── tasks/               # Task descriptions
├── README.md            # This file
├── LICENSE              # MIT License
└── CHANGELOG.md         # Release notes
```

## Customization

### Adding PHI Patterns
Modify `phi-detector/config/phi_patterns.yaml` to include healthcare-specific patterns:

```yaml
patterns:
  - name: DRUG_NAME
    regex: '\b(lipitor|metformin)\b'
    score: 0.9
  - name: Indonesian BPJS
    regex: '\b\d{13}\b'
    score: 0.93
```

### Extending NLP
Customize NLP in `python/nlp/` to enhance context-aware detection, such as recognizing medical terms or FHIR-specific entities.

### Custom Pipelines
Use the library scanner in Rust to build detection flows:

```rust
use phi_detector::scanner::Scanner;
let scanner = Scanner::new(phi_detector::phi_patterns::PHIPattern::all_patterns(), 10);
let findings = scanner.scan("NIK: 1234567890123456");
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