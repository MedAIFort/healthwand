<div align="center">

# HealthWand

![Rust](https://img.shields.io/badge/Rust-1.65%2B-orange?logo=rust)
![Python](https://img.shields.io/badge/Python-3.9%2B-brightgreen?logo=python)
![Status](https://img.shields.io/badge/status-revival-yellow)

![CodeRabbit Pull Request Reviews](https://img.shields.io/coderabbit/prs/github/MedAIFort/healthwand?utm_source=oss&utm_medium=github&utm_campaign=MedAIFort%2Fhealthwand&labelColor=171717&color=FF570A&link=https%3A%2F%2Fcoderabbit.ai&label=CodeRabbit+Reviews)
[![codecov](https://codecov.io/gh/MedAIFort/healthwand/branch/main/graph/badge.svg)](https://codecov.io/gh/MedAIFort/healthwand)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

**Indonesian PHI detection for engineering teams.**

</div>

HealthWand is an open-source developer-tooling layer for detecting and redacting Indonesian Protected Health Information (PHI) — including data classified as _"specific personal data"_ (data spesifik) under **UU PDP** (Law No. 27 of 2022) Article 4(1). It runs as a Rust CLI, a GitHub Action, and (planned) a Bahasa-aware Python NLP validator, helping engineering teams catch sensitive patient data before it ships into code, logs, exports, or AI training artifacts.

> **Repository status (May 2026).** HealthWand is in active revival after a development pause. The framing, audience, and scope are defined in [`POSITIONING.md`](./POSITIONING.md) (v0.1.0-draft.1). This README reflects that positioning. The Rust CLI `phi-detector` is the only built component; the Python NLP validator and API server are planned. MSRV and Python floor on the badges above are pre-revival values and will be re-verified during the modernization audit.

---

## Why HealthWand

The PHI-detection landscape is well-served for English-language data by mature tools like Microsoft Presidio, Google Cloud DLP, Azure Health De-identification Service, and AWS Comprehend Medical. These are good products serving their target markets.

**Indonesian PHI is the gap.** None of those tools ship first-class detection for NIK, BPJS Kesehatan, NPWP (16-digit post-2024), STR, No. Rekam Medis formats, Indonesian phone numbers, RT/RW address tokens, or Bahasa Indonesia context patterns. HealthWand fills that gap, runs alongside whatever else you already use, and grounds its detection categories in UU PDP rather than in marketing.

**UU PDP enforcement matters now.** The transition period ended 17 October 2024; the law is in force. Health data is explicitly classified as _specific personal data_ requiring stricter handling. Breach notification to authorities is required within 3×24 hours. Criminal penalties under Article 67 reach up to 5 years imprisonment and IDR 5 billion in fines. Extraterritorial scope means foreign organizations processing Indonesian patient data are subject too. A leak that costs a multinational a fine costs a 15-engineer startup the company.

**HealthWand is what you put in your CI before that happens.**

---

## Primary audience

HealthWand is built first for **Indonesian AI and health-tech startups** — 5- to 100-person engineering teams shipping products that touch Indonesian patient data. Telemedicine platforms, clinical decision support, hospital information systems, AI diagnostics, clinical research pipelines.

**Secondary audience:** hospital and fasyankes vendors who build SIMRS, HMIS modules, telemedicine integrations, or AI tools for the Indonesian market. HealthWand reaches healthcare facilities (RS, klinik, Puskesmas) _through_ the vendors who serve them, not directly. Hospital procurement is not the design center.

**Expansion audience:** multinationals touching Indonesian patient data (medical tourism, MNC hospital chains, international insurance) — once HealthWand is proven in the wild, not before.

See [`POSITIONING.md`](./POSITIONING.md) for the full audience analysis, sequencing rationale, and anti-goals.

---

## What HealthWand does

- **Indonesian PHI pattern detection.** First-class detectors for NIK (16-digit), BPJS Kesehatan (13-digit), NPWP (15-digit legacy + 16-digit post-2024), STR (medical professional license), Indonesian phone formats (+62 / 08xx), and address tokens (Jl., RT/RW, Kelurahan, Kecamatan, Kabupaten).
- **Western PHI patterns as supported community contributions.** US SSN, US medical record number formats, ICD-10 codes, and similar are shipped but not the design center.
- **Redaction and masking.** PHI matches can be redacted in place (e.g., `1234567890123456` → `XXXXXXXXXXXXXXXX`) so derived artifacts can ship safely.
- **YAML-configurable patterns.** Custom detectors are defined in `phi-patterns.yaml` — no Rust required for pattern authors.
- **GitHub Action integration.** Drop-in CI workflow to fail builds when PHI is detected before merge.
- **Structured output.** JSON output suitable for SARIF-style consumption, security dashboards, or compliance audit trails.
- **High performance.** Rust gives fast scans over large repositories and exports without becoming a CI bottleneck.

## What HealthWand does NOT do

These are explicit anti-goals (see [`POSITIONING.md`](./POSITIONING.md) §3):

- HealthWand is **not** a UU PDP, HIPAA, GDPR, or HITRUST compliance certification.
- HealthWand is **not** an EHR, SIMRS, or hospital information system.
- HealthWand is **not** a hosted SaaS — the OSS tool is the product.
- HealthWand is **not** a replacement for DPIA, DPO appointment, or organizational governance.
- HealthWand is **not** a synthetic-data generator or tokenization vault.

---

## Quickstart

### Prerequisites

- Rust stable toolchain with Cargo (current MSRV badge is 1.65+ pre-revival; final MSRV will be set during the modernization audit).
- Git.
- Python 3.9+ (optional, for the planned NLP validator).

### Install the Rust CLI

```bash
git clone https://github.com/MedAIFort/healthwand.git
cd healthwand/phi-detector
cargo build --release
cargo install --path .
phi-detector --help
```

### Scan a file

```bash
echo "NIK pasien: 3271010101010001, BPJS: 0001234567890" > sample.txt
phi-detector --input ./sample.txt --output text
```

### Scan a repository with redaction

```bash
phi-detector --input ./docs --output json --redact -vv > phi-report.json
```

Flags:

- `--input` — file or directory to scan.
- `--output` — `json` (structured findings) or `text`.
- `--redact` — replace PHI matches with masked values in derived output.
- `-v` / `-vv` — verbosity.

By default, the CLI scans files with extensions `.txt`, `.md`, `.csv`. The detailed JSON schema lives in [`phi-detector/docs/output_format.md`](./phi-detector/docs/output_format.md).

---

## Indonesian PHI patterns

HealthWand's headline coverage. Each pattern is configurable; the table below is the default catalogue.

| Pattern              | Format                                                                     | Example                          | Notes                                                               |
| -------------------- | -------------------------------------------------------------------------- | -------------------------------- | ------------------------------------------------------------------- |
| **NIK (KTP)**        | 16 digits, encodes province/regency/DOB                                    | `3271010101010001`               | National identity number. Context words: NIK, KTP, NomorInduk.      |
| **BPJS Kesehatan**   | 13 digits                                                                  | `0001234567890`                  | Health insurance number. Context words: BPJS, JKN, PesertaBPJS.     |
| **NPWP**             | 15-digit legacy `XX.XXX.XXX.X-XXX.XXX` or 16-digit post-2024 (NIK-aligned) | `1234567890123456`               | Tax ID. Both formats valid during ongoing transition.               |
| **STR**              | Variable; medical professional license number                              | (format varies)                  | Surat Tanda Registrasi. Issued by KKI / MTKI for health workers.    |
| **No. Rekam Medis**  | Hospital-specific format                                                   | (varies per fasyankes)           | Detection is context-dependent; requires NLP layer for high recall. |
| **Indonesian phone** | `+62 8xx-xxxx-xxxx` or `08xx-xxxx-xxxx`                                    | `+628123456789`                  | Mobile prefixes 81–89.                                              |
| **Address tokens**   | Jl./Jalan, Gg./Gang, RT/RW, Kel./Desa, Kec.                                | `Jl. Sudirman No. 5 RT 03/RW 02` | High value; current detection is regex + token-pattern-based.       |
| **Indonesian DOB**   | `dd/mm/yyyy`, `dd-mm-yyyy`, with month names                               | `12 Januari 1985`                | Common formats; Bahasa month names included.                        |

The full taxonomy with regex sources, false-positive notes, and detection-approach rationale will live in `docs/phi-taxonomy-id.md` (forthcoming).

---

## Configuration

Custom patterns are defined in `phi-detector/config/phi_patterns.yaml`. Indonesian patterns first:

```yaml
patterns:
  - name: Indonesian NIK (KTP)
    regex: '\b\d{16}\b'
    score: 0.95
    context_words: [NIK, KTP, NomorInduk]

  - name: Indonesian BPJS Kesehatan
    regex: '\b\d{13}\b'
    score: 0.93
    context_words: [BPJS, JKN, PesertaBPJS]

  - name: Indonesian phone (+62)
    regex: '(\+62|0)8[1-9]\d{6,11}'
    score: 0.85

  - name: US SSN
    regex: '\b\d{3}-\d{2}-\d{4}\b'
    score: 0.95
```

> **Status (May 2026):** The CLI currently uses built-in patterns. YAML loading is implemented in the library and planned for the CLI flag surface during modernization. NLP-related config keys will be defined when the Python validator is integrated.

---

## GitHub Action integration

The most common use is a pre-merge CI check. Drop this into `.github/workflows/healthcare-compliance.yml`:

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
        uses: actions/checkout@v4
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
        uses: actions/upload-artifact@v4
        with:
          name: phi-report
          path: phi-report.json
```

A native GitHub Action listing in the Marketplace is planned (see Roadmap below). Until then, the workflow above is the canonical integration pattern.

---

## Regulatory framing

HealthWand is built to support organizations operating under UU PDP and adjacent frameworks. The claim discipline matters; the full statement of _what HealthWand says and does not say_ lives in [`POSITIONING.md`](./POSITIONING.md) §7. Summary:

### UU PDP (Law No. 27 of 2022)

HealthWand detects categories of data classified as _"specific personal data"_ (data spesifik) under Article 4(1) — health and medical information, biometric and genetic data, children's data, criminal records, personal financial data — to support DPIA workflows, pre-deployment scanning, and the 3×24-hour breach-prevention posture required under Article 46.

HealthWand does **not** make any organization UU PDP compliant. Compliance is procedural. It does not replace DPIA execution, DPO appointment (clarified by Constitutional Court Decision No 151/PUU-XXII/2024), or breach notification responsibility.

The implementing regulation (Draft GR PDP / RPP PDP) completed harmonization in October 2025 and is awaiting Presidential approval as of May 2026. HealthWand documentation will be updated to reference specific GR PDP articles once enacted.

### HIPAA, GDPR, HITRUST

HealthWand ships patterns useful to organizations operating under these frameworks. It is **not** HIPAA-certified, GDPR-certified, or HITRUST-certified — software is not certified under these regimes; organizations are.

The detector-to-regulation mapping table lives in `docs/regulatory-mapping.md` (forthcoming).

---

## Comparison with other PHI detection tools

The following tools are well-engineered and serve their target markets. HealthWand is complementary; it runs alongside them when Indonesian coverage is needed.

| Tool                           | Strength                                  | Indonesian coverage    |
| ------------------------------ | ----------------------------------------- | ---------------------- |
| Microsoft Presidio             | Mature OSS, broad English entity coverage | Not shipped by default |
| Google Cloud DLP               | 150+ entity types, custom infoTypes       | Not shipped by default |
| Azure Health De-identification | Healthcare-specific, 27 entity categories | Not shipped by default |
| AWS Comprehend Medical         | Medical-NER focus, FHIR integration       | Not shipped by default |

If you already run one of these, HealthWand adds Indonesian patterns and UU PDP-grounded categorization without requiring you to switch.

---

## Project structure

```
healthwand/
├── POSITIONING.md       # Audience anchor, anti-goals, regulatory claim discipline
├── README.md            # This file
├── LICENSE              # MIT
├── CHANGELOG.md         # Release notes
├── phi-detector/        # Rust CLI crate (the only built component as of May 2026)
│   ├── src/             # CLI + library modules
│   ├── config/          # YAML patterns (Indonesian-first)
│   ├── docs/            # Output format docs
│   └── tests/           # Integration tests
├── docs/                # Repository documentation
│   ├── regulatory-mapping.md   # (forthcoming) detector → UU PDP article table
│   └── phi-taxonomy-id.md      # (forthcoming) full Indonesian PHI catalogue
├── scripts/             # Project scripts
└── .taskmaster/         # Task Master configuration and tasks
```

---

## Adding custom patterns

Edit `phi-detector/config/phi_patterns.yaml`:

```yaml
patterns:
  - name: Drug name (Indonesian formulary subset)
    regex: '\b(parasetamol|amoksisilin|metformin)\b'
    score: 0.9
    context_words: [obat, resep, dosis]

  - name: Custom hospital MRN
    regex: '\bMRN-\d{8}\b'
    score: 0.95
    context_words: [RM, RekamMedis, NomorRM]
```

Or build detection flows from the library:

```rust
use phi_detector::scanner::Scanner;
let scanner = Scanner::new(phi_detector::phi_patterns::PHIPattern::all_patterns(), 10);
let findings = scanner.scan("NIK: 3271010101010001");
```

---

## Roadmap

Aligned with [`POSITIONING.md`](./POSITIONING.md) §9 (Build → Works → Community).

- **v1.0 — Build phase.** CLI feature-complete for Indonesian PHI patterns; YAML pattern loading exposed via CLI flag; GitHub Action published to Marketplace; `docs/regulatory-mapping.md` and `docs/phi-taxonomy-id.md` published; modernization audit complete (dependency bumps, MSRV verification, CI v4, Python floor).
- **v1.x — Works phase.** Python NLP validator (Bahasa-first context-aware detection); deployment patterns documented for vendor adoption; first community-contributed patterns merged; first external write-up referencing HealthWand by name.
- **v2.0+ — Community / expansion phase.** Expanded ecosystem integrations; multinational-adjacent features (attested releases, signed pattern bundles) considered if and only if there is demonstrated demand. Hosted dashboard concepts (originally floated as `medaifort.com`) remain deferred and out of scope for the OSS repo.

The roadmap is tentative and subject to community signal. Open questions deliberately deferred (Python NLP shape, API-server need, possible rename of `phi-detector`) are documented in [`POSITIONING.md`](./POSITIONING.md) §11.

---

## Status of planned components

Honest summary as of May 2026:

| Component                          | Status                                                      |
| ---------------------------------- | ----------------------------------------------------------- |
| `phi-detector` Rust CLI            | Built; primary distribution channel                         |
| YAML pattern loading (library)     | Implemented                                                 |
| YAML pattern loading (CLI flag)    | Planned (modernization phase)                               |
| GitHub Action Marketplace listing  | Planned                                                     |
| Python NLP validator (Bahasa)      | Planned; shape decisions in `ARCHITECTURE.md` (forthcoming) |
| API server                         | Deferred; rationale revisited in `ARCHITECTURE.md`          |
| Hosted dashboard / `medaifort.com` | Out of scope for OSS repo                                   |

---

## Contributing

Contributions are welcome. The highest-value contributions right now:

1. **Indonesian PHI patterns we missed.** Hospital-specific MRN formats, region-specific identifiers, formulary-specific drug names. PRs adding YAML patterns are reviewed quickly.
2. **Bahasa context word lists.** Words that disambiguate "this 16-digit number is a NIK" from "this 16-digit number is a transaction ID."
3. **Regulatory mapping entries.** If you know UU PDP, Permenkes, or BSSN guidance well, help build out `docs/regulatory-mapping.md`.
4. **Modernization PRs.** Dependency bumps, MSRV verification, CI version updates.

See `CONTRIBUTING.md` for the standard workflow. PRs that move HealthWand toward the anti-goals in [`POSITIONING.md`](./POSITIONING.md) §3 will be declined politely with reference to that document.

---

## Community

- **GitHub Issues** — bug reports, feature requests, pattern requests.
- **GitHub Discussions** — design conversations, regulatory questions, adoption stories.
- **X / Bluesky:** `@ks_sha888` for updates and Indonesian health-data security notes.

---

## License

MIT. See [LICENSE](LICENSE).

---

## Acknowledgments

HealthWand is positioned as complementary to (not a replacement for) Microsoft Presidio, Google Cloud DLP, Azure Health De-identification Service, AWS Comprehend Medical, and the broader open-source PHI-detection ecosystem. Those projects do the foundational work of making PHI detection a normal part of healthcare software development; HealthWand extends that practice into the Indonesian-language, UU PDP-grounded context.

---

_HealthWand: Indonesian PHI detection, grounded in regulation, built for engineering teams._
