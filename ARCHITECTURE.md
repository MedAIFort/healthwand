# ARCHITECTURE — HealthWand

**Status:** `v0.1.0-draft.1`
**Locked:** 2026-05-13
**Aligned with:**

- `POSITIONING.md` v0.1.0-draft.1
- `README.md` v0.2.0-draft.1
- `docs/regulatory-mapping.md` v0.1.0-draft.1
- `docs/phi-taxonomy-id.md` v0.1.0-draft.1
- Decisions ratified 2026-05-13: rule-based Python NLP for v1.0 (transformer v2.0+ opt-in); API server deferred to v2.0+; rename `phi-detector` → `healthwand` (single crate, lib + bin)

**License of this document:** MIT (same as repo)

---

## 0. Purpose, scope, and design tenets

### 0.1 What this document is

This is the technical architecture for HealthWand's revival to v1.0. It defines:

- **Component boundaries** — what runs where, what depends on what.
- **Public contracts** — the types, traits, and APIs that external consumers depend on.
- **Data flow** — input → detection → output, end to end.
- **Extension points** — where contributions plug in.
- **Migration path** — from the existing `phi-detector` to `healthwand`.
- **Anti-architecture** — what is explicitly out of scope.

This document is the bridge between positioning and code. Every claim in `POSITIONING.md` and the regulatory framing in `docs/regulatory-mapping.md` must be physically achievable in the architecture below — otherwise, one of the two documents is wrong.

### 0.2 Design tenets (non-negotiable)

These tenets are inherited from the project's design preferences and apply throughout. CI must enforce them where possible (see §9.2).

1. **KISS / YAGNI.** Every component must justify its existence against a real use case. No speculative abstractions.
2. **Hexagonal / Ports & Adapters.** Domain (detection logic) is isolated from adapters (CLI, YAML, output formatters, I/O). Domain types do not import I/O or formatting concerns.
3. **Parse-Don't-Validate.** External data (YAML configs, file content, user input) crosses the boundary exactly once and becomes typed values. Once parsed, illegal states are unrepresentable.
4. **Make-Illegal-States-Unrepresentable.** Rust newtypes for `Severity`, `PatternId`, `Score`, `MatchSpan`. Enums for `DetectorType`. No stringly-typed APIs in the public surface.
5. **Library-first, binary-last.** The library (`healthwand` crate's `[lib]` section) defines the system. The binary is one consumer among potentially many.
6. **High Cohesion, Low Coupling.** Modules group by _what they do_, not by _what they are_ (no `utils` modules).
7. **SemVer enforced.** Public API changes follow SemVer strictly, enforced via `cargo-semver-checks` in CI.
8. **Principle of Least Privilege.** Components see only what they need. The YAML loader does not see the output formatter; the file walker does not see the pattern catalogue.
9. **Chesterton's Fence.** Existing `phi-detector` code that solves its problem correctly is preserved during migration, even if reorganized.
10. **Boy Scout Rule.** Every PR leaves the area it touches at least as well-organized as it found it.

---

## 1. System overview

### 1.1 Components

HealthWand v1.0 consists of two independently-distributed components, with a third planned for v1.x:

```
┌──────────────────────────────────────────────────────────────────────┐
│                          HealthWand                                  │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────┐    ┌─────────────────────────────┐  │
│  │ healthwand (Rust)           │    │ healthwand-nlp (Python)     │  │
│  │ [lib] + [[bin]]             │    │ planned v1.x                │  │
│  │                             │    │                             │  │
│  │ Distribution:               │    │ Distribution:               │  │
│  │   crates.io                 │    │   PyPI                      │  │
│  │   GitHub Action             │    │   uv pip install            │  │
│  │   cargo install             │    │                             │  │
│  │                             │    │ Scope:                      │  │
│  │ Detectors:                  │    │   - Bahasa names (rules)    │  │
│  │   - regex                   │    │   - Diagnosis text (rules)  │  │
│  │   - regex + context         │    │   - Dictionary lookups      │  │
│  │   - dictionary              │    │                             │  │
│  │   - combinatorial (v1.x)    │    │ Transformer NLP: v2.0+ opt- │  │
│  │                             │    │   in (separate extras)      │  │
│  └──────────┬──────────────────┘    └─────────────┬───────────────┘  │
│             │                                     │                  │
│             └──────── shared via ─────────────────┘                  │
│                                                                      │
│                   ┌────────────────────────┐                         │
│                   │ phi-patterns.yaml      │                         │
│                   │ (single source of      │                         │
│                   │ truth for detectors)   │                         │
│                   └────────────────────────┘                         │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

### 1.2 What HealthWand explicitly is NOT (architectural anti-goals)

- **Not an HTTP service.** No API server in v1.0 or v1.x. CLI-as-subprocess is the integration pattern for production runtime use cases. v2.0+ may add an opt-in HTTP wrapper if real demand materializes; until then, it is out of scope.
- **Not a daemon.** No background process, no persistent state, no IPC server.
- **Not a workflow engine.** HealthWand scans; it does not orchestrate DPIA workflows, breach notification submissions, or compliance audits.
- **Not a tokenization or pseudonymization tool.** Tonic.ai, Skyflow, and others serve that category.
- **Not an EHR / SIMRS adapter.** No FHIR client, no HL7 parser, no DICOM handling in v1.0. Image-PHI is planned for v2.0; FHIR-aware detection is a possible future direction but not part of v1.0/v1.x.

These are explicit. Pull requests moving HealthWand toward any of these will be declined with reference to this section.

### 1.3 Distribution model

| Component                       | Channel                   | Versioning                          | Notes                                   |
| ------------------------------- | ------------------------- | ----------------------------------- | --------------------------------------- |
| `healthwand` (Rust crate)       | crates.io                 | SemVer, `cargo-semver-checks` in CI | Library + binary in one crate           |
| `healthwand` (binary, prebuilt) | GitHub Releases           | Same SemVer                         | Multi-platform binaries via release CI  |
| HealthWand GitHub Action        | GitHub Action Marketplace | Independent action versioning       | Thin wrapper around the binary          |
| `healthwand-nlp` (Python)       | PyPI                      | SemVer                              | Planned v1.x; independent release cycle |

All consumer-facing artifacts ship from the same monorepo.

---

## 2. The Rust core (`healthwand` crate)

The Rust crate is the primary artifact. The library defines the system; the binary is one consumer.

### 2.1 Public API surface

The `lib.rs` exports:

```rust
// Core domain types
pub use domain::{Pattern, Finding, Severity, Score, MatchSpan, DetectorType};

// Scanner orchestrator (the main entry point)
pub use scanner::{Scanner, ScanConfig, ScanReport};

// Detector trait (for custom detectors)
pub use detect::Detector;

// Configuration loading
pub use config::{PatternCatalogue, load_yaml};

// Output formatting
pub use format::{Format, JsonFormatter, TextFormatter, SarifFormatter};

// Error types
pub use error::{HealthwandError, Result};
```

This surface is **stable from v1.0**. Breaking changes require a major version bump and `cargo-semver-checks` will fail otherwise.

### 2.2 Hexagonal layering

The crate organizes by hexagonal/ports-and-adapters discipline:

```
   ┌───────────────── Driving adapters ─────────────────┐
   │  CLI (bin/healthwand.rs)                           │
   │  GitHub Action (calls the binary)                  │
   │  Library consumers (other Rust crates)             │
   │  Python NLP wrapper (subprocess from companion)    │
   └────────────────────────┬───────────────────────────┘
                            │
   ┌────────────────────────▼───────────────────────────┐
   │  Application core (domain)                         │
   │                                                    │
   │  - Scanner            (orchestrator)               │
   │  - Pattern            (typed regex + context)      │
   │  - Finding            (typed scan result)          │
   │  - Severity, Score    (newtypes)                   │
   │  - Detector trait     (pluggable strategy)         │
   │  - Combinatorial      (v1.x)                       │
   │                                                    │
   │  No imports from: io, format, config (only        │
   │  defines abstractions; impls live in adapters).   │
   └────────────────────────┬───────────────────────────┘
                            │
   ┌────────────────────────▼───────────────────────────┐
   │  Driven adapters                                   │
   │  - YAML pattern loader (config/)                   │
   │  - File walker (io/walker.rs)                      │
   │  - File reader (io/reader.rs)                      │
   │  - Output formatter (format/)                      │
   │  - NLP IPC bridge (nlp/) — v1.x                    │
   └────────────────────────────────────────────────────┘
```

The domain layer (`src/domain/`) is the architectural heart. It imports nothing from the io, format, or config modules. Its tests run in milliseconds without touching the filesystem.

### 2.3 Domain types

Critical types, with type discipline applied:

```rust
// Severity — newtype enum (no stringly-typed)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Informational,
    Medium,
    High,
    Critical,
}

// Score — newtype, validated in 0.0..=1.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Score(f32);

impl Score {
    pub fn new(value: f32) -> Result<Self> { /* parse-don't-validate */ }
    pub fn value(&self) -> f32 { self.0 }
}

// MatchSpan — typed byte range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchSpan {
    pub start: usize,
    pub end: usize,
    pub line: u32,
    pub column: u32,
}

// PatternId — newtype to prevent mixing with arbitrary strings
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatternId(String);

// DetectorType — enum, no fallthrough
pub enum DetectorType {
    Regex,
    RegexWithContext,
    Dictionary,
    Combinatorial,    // v1.x
    Nlp,              // hand-off to companion package
}

// Pattern — fully-parsed configuration
pub struct Pattern {
    pub id: PatternId,
    pub name: String,
    pub detector_type: DetectorType,
    pub category: Category,            // UU PDP categorization
    pub default_severity: Severity,
    pub score: Score,
    pub regex: Option<regex::Regex>,   // present if DetectorType::Regex*
    pub context_words: Vec<String>,    // present if context-required
    pub context_window: usize,
}

// Finding — output of a single detection
pub struct Finding {
    pub pattern_id: PatternId,
    pub span: MatchSpan,
    pub matched_text: String,
    pub severity: Severity,
    pub score: Score,
    pub context_matched: bool,
    pub uu_pdp_article: Option<UuPdpArticle>,  // optional regulatory cross-reference
}
```

**Parse-don't-validate boundary.** YAML enters the system as `serde_yaml::Value` → passes through `config::validate::parse_catalogue` → becomes `Vec<Pattern>`. After that point, no string-typed access to the catalogue is allowed in domain code.

### 2.4 Detector trait

The `Detector` trait is the pluggable extension point:

```rust
pub trait Detector: Send + Sync {
    /// The detector type discriminator.
    fn detector_type(&self) -> DetectorType;

    /// Scan a chunk of text. Returns findings; should not mutate state.
    fn scan(&self, text: &str, pattern: &Pattern) -> Vec<Finding>;

    /// Indicate whether this detector handles a given pattern.
    fn handles(&self, pattern: &Pattern) -> bool;
}
```

Built-in implementations (in `src/detect/`):

- `RegexDetector` — `DetectorType::Regex` and `DetectorType::RegexWithContext`
- `DictionaryDetector` — `DetectorType::Dictionary`
- `CombinatorialDetector` — `DetectorType::Combinatorial` (v1.x)
- `NlpStubDetector` — `DetectorType::Nlp` (v1.0 stub; emits warning that NLP is not enabled)

Custom detectors implementing this trait can be registered via the `Scanner` builder. This is the contributor extension point.

### 2.5 Scanner orchestrator

`Scanner` is the entry point for callers:

```rust
pub struct Scanner {
    catalogue: PatternCatalogue,
    detectors: Vec<Box<dyn Detector>>,
    config: ScanConfig,
}

impl Scanner {
    pub fn builder() -> ScannerBuilder { /* ... */ }

    pub fn scan_text(&self, text: &str) -> ScanReport { /* ... */ }
    pub fn scan_path(&self, path: &Path) -> Result<ScanReport> { /* ... */ }
}
```

`ScannerBuilder` provides fluent configuration:

```rust
let scanner = Scanner::builder()
    .with_catalogue(catalogue)
    .with_detector(Box::new(MyCustomDetector::new()))
    .with_min_severity(Severity::High)
    .with_context_window(100)
    .build()?;
```

### 2.6 YAML configuration loader

Located in `src/config/`. The YAML schema is documented separately (see §5); the loader's job is to translate YAML into typed `Pattern` values.

Boundary discipline:

```rust
// External: YAML bytes
let yaml: &[u8] = std::fs::read("phi-patterns.yaml")?;

// Boundary parse: YAML → typed PatternCatalogue (parse-don't-validate)
let catalogue: PatternCatalogue = config::load_yaml(yaml)?;

// Internal use: typed only, no string-based access
for pattern in catalogue.patterns() {
    // pattern.regex is regex::Regex, not String
    // pattern.severity is Severity, not String
    // ...
}
```

Validation happens once, at the boundary. Errors are `HealthwandError::ConfigError` variants with precise locations (line, column, key).

### 2.7 Output formatters

Located in `src/format/`. Each formatter implements:

```rust
pub trait Formatter {
    fn format(&self, report: &ScanReport, writer: &mut dyn Write) -> Result<()>;
}
```

Built-in:

- `JsonFormatter` — structured findings, suitable for SARIF-style consumption.
- `SarifFormatter` — full SARIF 2.1.0 output, native CI/security-dashboard integration.
- `TextFormatter` — human-readable, default for terminal use.

The formatter does not see the catalogue or the scanner — only the report. This keeps it cleanly decoupled.

### 2.8 Crate layout

```
healthwand/                     # repo root
├── Cargo.toml                  # [package] name = "healthwand"
├── src/
│   ├── lib.rs                  # Public API surface
│   ├── error.rs                # HealthwandError, Result type alias
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── pattern.rs
│   │   ├── finding.rs
│   │   ├── severity.rs
│   │   ├── score.rs
│   │   ├── span.rs
│   │   └── category.rs         # UU PDP categorization
│   ├── detect/
│   │   ├── mod.rs              # Detector trait
│   │   ├── regex_detector.rs
│   │   ├── dictionary.rs
│   │   ├── combinatorial.rs    # v1.x
│   │   └── nlp_stub.rs
│   ├── scanner/
│   │   ├── mod.rs
│   │   ├── builder.rs
│   │   └── report.rs
│   ├── config/
│   │   ├── mod.rs
│   │   ├── yaml_schema.rs
│   │   └── parse.rs            # Parse-don't-validate boundary
│   ├── io/
│   │   ├── mod.rs
│   │   ├── walker.rs           # Directory traversal (uses `ignore` crate)
│   │   └── reader.rs
│   ├── format/
│   │   ├── mod.rs              # Formatter trait
│   │   ├── json.rs
│   │   ├── sarif.rs
│   │   └── text.rs
│   ├── nlp/                    # v1.x — NLP companion bridge
│   │   ├── mod.rs
│   │   └── ipc.rs
│   └── bin/
│       └── healthwand.rs       # CLI entry point
├── tests/
│   ├── integration_yaml.rs
│   ├── integration_cli.rs
│   └── fixtures/
│       └── synthetic_phi/      # Synthetic Indonesian PHI for tests
└── benches/
    └── scan_throughput.rs
```

This is a _recommended_ layout — actual file organization may iterate during implementation. The module boundaries (`domain`, `detect`, `scanner`, `config`, `io`, `format`, `nlp`, `bin`) are the architectural commitment.

### 2.9 Recommended dependencies

These are conventional Rust ecosystem choices, not mandates. Final versions are pinned during the modernization audit.

| Crate                            | Purpose                     | Notes                                                  |
| -------------------------------- | --------------------------- | ------------------------------------------------------ |
| `regex`                          | Regex engine                | Rust-flavor regex; no lookarounds — design accordingly |
| `serde` + `serde_yaml`           | YAML config                 | Yaml 1.2 strict subset                                 |
| `serde_json`                     | JSON output                 | Standard                                               |
| `clap` (derive)                  | CLI parsing                 | Latest stable                                          |
| `anyhow`                         | Binary-side error context   | CLI only                                               |
| `thiserror`                      | Library error definitions   | `HealthwandError`                                      |
| `tracing` + `tracing-subscriber` | Structured logging          | `-v`/`-vv` controls level                              |
| `rayon`                          | Data parallelism            | Per-file scanning                                      |
| `ignore` (BurntSushi)            | Git-aware directory walking | Honors `.gitignore`                                    |
| `cargo-semver-checks` (dev)      | Public API stability        | CI gate                                                |

Avoid: `tokio` (no async needed for a CPU-bound scanner), `reqwest` (no network), heavy framework dependencies.

---

## 3. The CLI

### 3.1 Command structure

```
healthwand --input <PATH> [--output <FORMAT>] [--config <YAML>]
           [--min-severity <LEVEL>] [--redact] [-v|-vv|-vvv]
           [--exclude <GLOB>] [--include <GLOB>]
           [--catalogue-print] [--list-detectors]
```

### 3.2 Subcommands (planned v1.x)

```
healthwand scan       (default, alias for the no-subcommand invocation)
healthwand validate   (validate a YAML config without scanning)
healthwand explain    (print regulatory mapping for a finding ID)
healthwand version    (print version, build metadata)
```

### 3.3 Configuration precedence

1. CLI flags (highest)
2. Environment variables (`HEALTHWAND_*`)
3. Repository-local config (`.healthwand.yaml` or `.healthwand.toml`)
4. User config (`~/.config/healthwand/config.yaml`)
5. Built-in defaults (lowest)

### 3.4 Exit codes

| Code | Meaning                                                           |
| ---- | ----------------------------------------------------------------- |
| 0    | No findings, or findings below `--min-severity` threshold         |
| 1    | Findings at or above `--min-severity` threshold (CI gate trigger) |
| 2    | Configuration error (invalid YAML, missing file)                  |
| 3    | Runtime error (I/O failure, internal bug)                         |
| 64   | Usage error (invalid flag combination)                            |

CI workflows assume `0 = pass, ≥1 = fail`. The granular codes are for diagnostic tooling.

### 3.5 GitHub Action wrapper

A thin wrapper invokes the binary with sensible CI defaults:

```yaml
- uses: medaifort/healthwand-action@v1
  with:
    input: '.'
    min-severity: 'high'
    output-format: 'sarif'
    config: '.healthwand.yaml'
```

The Action's responsibility is: install the binary (cached), invoke it with mapped inputs, upload the SARIF as an artifact, and surface findings in the PR review UI via SARIF code-scanning integration.

The Action lives in a separate repo (`MedAIFort/healthwand-action`) and pins to specific `healthwand` binary versions. This separation allows independent action-version cadence.

---

## 4. The Python NLP layer (planned v1.x)

### 4.1 Why a separate package

Per decision §11 of `POSITIONING.md` (ratified 2026-05-13): rule-based + dictionary for v1.x; transformer NLP deferred to v2.0+ opt-in.

The separate-package design has three rationales:

1. **OSS adoption gradient.** A `cargo install healthwand` is < 30 seconds; a `pip install healthwand-nlp` is comparable. Combining them via `cargo install healthwand-with-python-and-models` punishes adopters who only want the Rust CLI.
2. **Independent failure modes.** A bug in the Python NLP layer must not break Rust CLI users who don't use NLP.
3. **Independent release cadence.** The Rust CLI's SemVer can stabilize at v1.0 while the NLP package iterates rapidly.

### 4.2 Decoupling pattern (independent tool, shared YAML)

The two components share the **YAML pattern catalogue** but not state, not runtime, and not packaging.

```yaml
patterns:
  - name: NIK
    type: regex # processed by healthwand (Rust)
    regex: '\b\d{16}\b'
    context_words: [NIK, KTP]

  - name: Indonesian names
    type: nlp # processed by healthwand-nlp (Python)
    nlp_detector: bahasa_name_rules
    score: 0.85
```

The Rust CLI processes `type: regex`, `type: dictionary`, `type: combinatorial` patterns and emits `NlpStubDetector` warnings for `type: nlp` patterns when NLP is not enabled. The Python NLP tool processes `type: nlp` patterns. Users orchestrate the two in their CI pipeline:

```yaml
# Example CI integration
- run: healthwand --input . --output json > findings-rust.json
- run: healthwand-nlp scan --input . --output json > findings-nlp.json
- run: healthwand-merge findings-rust.json findings-nlp.json > findings.json
```

The `healthwand-merge` step is itself a small companion utility (planned v1.x).

### 4.3 Detector types: regex vs NLP vs dictionary vs combinatorial

| Detector type   | Runtime       | YAML key                                           | Notes                                    |
| --------------- | ------------- | -------------------------------------------------- | ---------------------------------------- |
| `regex`         | Rust          | `regex`, `context_words`                           | Highest precision; default               |
| `regex_context` | Rust          | `regex`, `context_words` (required)                | Context required for severity escalation |
| `dictionary`    | Rust          | `dictionary` (path to terms file), `context_words` | Drug names, FORNAS subset                |
| `combinatorial` | Rust (v1.x)   | `triggers` (list of co-occurring detectors)        | Combinatorial-severity engine            |
| `nlp`           | Python (v1.x) | `nlp_detector` (named rule/model)                  | Bahasa names, diagnosis text             |

### 4.4 v1.x rule-based scope; v2.0+ transformer scope

**v1.x rule-based scope:**

- Bahasa honorific rules beyond what the Rust regex covers (multi-token name disambiguation, conjugation handling, edge cases).
- Bahasa medical vocabulary dictionary (ICD-10 plain-language equivalents in Indonesian, common disease and symptom terms, medication formulary subset).
- Context-window rules using Indonesian-language grammar features (e.g., distinguishing "pasien" predicate vs. attribute).
- No transformer dependency. Pure Python + `regex` library.

**v2.0+ transformer scope (opt-in extras):**

- IndoBERT-derived or fine-tuned medical NER (if labeled data becomes available).
- spaCy with Indonesian models for shallow parsing.
- Installation: `pip install healthwand-nlp[transformer]` — explicit opt-in, surfaces the dependency weight.

The boundary is sharp: v1.x ships with no transformer dependency at all. v2.0+ introduces the dependency only via opt-in extras. This protects the v1.x adoption surface.

### 4.5 Python package structure (planned, draft)

```
python/
├── pyproject.toml              # uv-managed
├── healthwand_nlp/
│   ├── __init__.py
│   ├── cli.py                  # entry point
│   ├── domain/
│   │   ├── pattern.py          # Pydantic models, parse-don't-validate
│   │   └── finding.py
│   ├── detect/
│   │   ├── name_rules.py       # Bahasa name disambiguation
│   │   ├── diagnosis_rules.py  # Bahasa diagnosis text
│   │   └── dictionary.py
│   ├── io/
│   │   └── walker.py
│   ├── format/
│   │   └── json.py
│   └── config/
│       └── yaml_loader.py      # Loads shared YAML schema
└── tests/
```

**Stack:** `uv` for project management, `pydantic` for typed config (Parse-Don't-Validate boundary), stdlib + `regex` for rule-based detection. Python ≥ 3.11 (3.9 floor in the pre-revival README is too old — 3.11 minimum reflects an actively-supported runtime).

---

## 5. Pattern catalogue (YAML schema)

### 5.1 Schema versioning

The YAML schema is versioned independently of the crate version, using the same SemVer discipline. Each pattern file declares its schema version:

```yaml
schema_version: '1.0'
patterns:
  - ...
```

Schema bumps:

- **Patch** — new optional fields, additive enum variants.
- **Minor** — new detector types, backward-compatible additions.
- **Major** — breaking changes (renamed fields, removed types).

The loader emits warnings on minor-version-newer schemas (forward compat) and errors on major-version mismatches.

### 5.2 Detector type reference

See §4.3 for the type table. Each detector type defines its own required and optional YAML keys.

### 5.3 Built-in patterns vs. user-extensible

HealthWand ships a default `phi-patterns.yaml` covering the catalogue documented in `docs/phi-taxonomy-id.md`. Users extend it via:

1. **Replacement** — `--config my-patterns.yaml` replaces the default.
2. **Merge** — `--config-include extra-patterns.yaml` merges with the default.
3. **Override** — patterns with matching `id` fields in user config override defaults.

The merge order is: built-in → user-config → CLI overrides.

---

## 6. Data flow

### 6.1 Single-file scan

```
input bytes → reader → text (UTF-8 validated)
                                │
                                ▼
                          scanner.scan_text()
                                │
                                ▼
                          for each pattern in catalogue:
                              detector.scan(text, pattern)
                                  │
                                  ▼
                              regex/dictionary match
                                  │
                                  ▼
                              context window check
                                  │
                                  ▼
                              Finding(pattern_id, span, severity, score)
                                │
                                ▼
                          ScanReport
                                │
                                ▼
                          formatter.format(report) → output bytes
```

### 6.2 Repository scan

The file walker (uses `ignore` crate) honors `.gitignore` by default. Each file scan is independent and parallelizable; `rayon` distributes scans across worker threads.

### 6.3 CI gate flow

```
GitHub Action
    │
    ▼
healthwand scan --input . --output sarif --min-severity high
    │
    ├── exit 0 → CI passes
    │
    └── exit 1 → CI fails
        │
        ▼
    SARIF uploaded as artifact
        │
        ▼
    GitHub Code Scanning surfaces findings in PR review
```

---

## 7. Extension points

### 7.1 Adding a YAML pattern

The lowest-friction extension. Edit (or create) a YAML file, add a pattern entry per the schema in `docs/phi-taxonomy-id.md`, and the scanner picks it up.

No Rust knowledge required. This is the _primary_ contribution surface for the community.

### 7.2 Implementing a custom detector (Rust)

For detection logic that the built-in types don't cover (e.g., checksum-validated detectors), implement the `Detector` trait in a downstream crate and register via `ScannerBuilder::with_detector`.

```rust
struct LuhnValidatedDetector;

impl Detector for LuhnValidatedDetector {
    fn detector_type(&self) -> DetectorType {
        DetectorType::Regex
    }
    fn handles(&self, pattern: &Pattern) -> bool {
        pattern.id.0 == "credit-card-luhn"
    }
    fn scan(&self, text: &str, pattern: &Pattern) -> Vec<Finding> {
        // regex match + Luhn validation
    }
}
```

### 7.3 Contributing to the pattern catalogue

Per `POSITIONING.md` §4, the highest-value contributions are Indonesian patterns: hospital-specific MRN formats, regional identifiers, formulary-specific drug names. The contribution workflow is:

1. Fork the repo.
2. Add a pattern entry to `config/phi-patterns.yaml` (with tests in `tests/fixtures/`).
3. Update `docs/phi-taxonomy-id.md` if the pattern is canonical.
4. Open a PR.

Reviewers verify against the `POSITIONING.md` §3 anti-goals.

### 7.4 Future: WASM detector hosting (v2.0+ consideration)

If real demand emerges for language-agnostic custom detectors, compiling detectors to WASM and hosting them inside HealthWand becomes a reasonable v2.0+ direction. Not in scope for v1.0 or v1.x. Mentioned here only to document that the extension architecture admits the possibility.

---

## 8. Cross-cutting concerns

### 8.1 Error handling

- **Library code** (`src/` except `bin/`): uses `thiserror` for `HealthwandError` variants. No `anyhow`. No `unwrap` in non-test code.
- **Binary code** (`src/bin/healthwand.rs`): uses `anyhow` for top-level error context.
- All errors carry actionable location info (file, line, column) where applicable.

### 8.2 Logging

`tracing` everywhere. Spans for major operations (`scan_file`, `load_catalogue`, `parse_yaml`). Field-structured for grep-ability.

CLI verbosity:

- `-v` → `INFO`
- `-vv` → `DEBUG`
- `-vvv` → `TRACE`

### 8.3 Configuration

Single-pass loading at startup. No reloading mid-scan. Configuration sources documented in §3.3 with explicit precedence.

### 8.4 Performance

Default target for v1.0: scan 100,000 lines of mixed text in under 1 second on a developer laptop.

- File-level parallelism via `rayon`.
- Regex compilation once per pattern, reused across files.
- No allocation in the hot path beyond what the `regex` crate requires.

Benchmarks (`benches/scan_throughput.rs`) gate regressions at 5% slowdown in CI.

### 8.5 Memory

For very large files (>100MB), streaming scan with bounded memory is required. v1.0 ships with `--max-file-size` flag (default 50MB) that skips larger files; streaming support is planned v1.x.

---

## 9. Build, test, release

### 9.1 Workspace layout

For v1.0, the repo is a single Rust crate (not a multi-crate workspace). The Python NLP component lives in a parallel `python/` directory with its own `pyproject.toml`.

If, in v2.0+, multiple Rust crates become necessary (e.g., `healthwand-core` and `healthwand-cli` split for binary distribution), the migration to a workspace happens then. v1.0 prioritizes simplicity.

### 9.2 SemVer discipline

CI gates:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-features`
- `cargo semver-checks check-release` (against latest published crate version)
- `cargo audit` (security advisories)
- Python (when present): `uv run ruff check`, `uv run pytest`, `uv run mypy`

Per the meta-rule from the user preferences: "principles without CI enforcement are decoration." Every design tenet in §0.2 has a corresponding CI check.

### 9.3 CI strategy

GitHub Actions workflow matrix:

| Platform       | Rust   | Python           | Purpose              |
| -------------- | ------ | ---------------- | -------------------- |
| ubuntu-latest  | stable | 3.11, 3.12, 3.13 | Primary              |
| macos-latest   | stable | 3.11             | Cross-platform smoke |
| windows-latest | stable | 3.11             | Cross-platform smoke |

MSRV (Minimum Supported Rust Version) is set during the modernization audit (the pre-revival 1.65 is too low for current ecosystem). Recommendation: pin to Rust stable minus two versions, validated by CI.

### 9.4 Distribution channels

- **crates.io**: `cargo publish` from a tagged release; SemVer enforced.
- **PyPI**: `uv build` + `uv publish` for `healthwand-nlp` (v1.x); SemVer enforced.
- **GitHub Releases**: prebuilt binaries via `cargo-dist` or similar tooling (final tool selection during modernization). Multi-platform (linux x86_64, linux aarch64, macOS arm64, macOS x86_64, windows x86_64).
- **GitHub Action Marketplace**: independent release cadence for the Action wrapper repo.
- **Docker (deferred)**: A container image is plausible for v1.x if demand emerges. Not in v1.0.

---

## 10. Anti-architecture (non-goals, repeated for emphasis)

To prevent scope creep:

| Component                       | Status                                              | Rationale                                                       |
| ------------------------------- | --------------------------------------------------- | --------------------------------------------------------------- |
| HTTP API server                 | Out of scope v1.0/v1.x; opt-in v2.0+ if real demand | CLI-subprocess covers production use; YAGNI applies             |
| Daemon mode                     | Out of scope                                        | No persistent state; no IPC server                              |
| EHR/SIMRS/FHIR adapters         | Out of scope v1.0/v1.x                              | Detection is the wedge; integration is application-specific     |
| HL7 / DICOM parsers             | Out of scope v1.0/v1.x                              | Different specialty; image PHI deferred to v2.0                 |
| Tokenization / pseudonymization | Out of scope, indefinitely                          | Detection-and-redaction only; other tools serve tokenization    |
| Workflow / case-management UI   | Out of scope, indefinitely                          | HealthWand emits findings; orchestration is downstream          |
| Hosted dashboard / SaaS         | Out of scope for OSS repo                           | `medaifort.com` is a separate concern; see `POSITIONING.md` §11 |

---

## 11. Migration plan: `phi-detector` → `healthwand`

The rename is a v1.0 task. Migration steps, in order:

1. **Pre-migration**: confirm `healthwand` is available on crates.io. If not, fall back to `healthwand-rs` (decided at migration time).
2. **Workspace rename**: move `phi-detector/src/` → `src/` at repo root. Move `phi-detector/config/` → `config/`. Move `phi-detector/docs/` → `docs/` (merge with existing `docs/`). Move `phi-detector/tests/` → `tests/`.
3. **Cargo.toml rewrite**: `[package].name = "healthwand"`; `[lib]` and `[[bin]]` sections. Binary name = `healthwand`.
4. **Module reorganization** to the layout in §2.8. Where existing modules are correctly factored, preserve verbatim (Chesterton's Fence); where reorganization is needed, do it in a separate commit with clear "rename" git history.
5. **Public API audit**: ensure the API exposed in §2.1 is what `lib.rs` re-exports. Mark internal items `pub(crate)`.
6. **CLI flag compatibility**: `phi-detector --input` and `healthwand --input` accept the same flags. Existing scripts continue to work after the binary rename.
7. **Documentation update**: `README.md` already done. Update any pre-revival docs not yet covered.
8. **CI workflow rewrite**: replace `phi-detector` references with `healthwand` in all `.github/workflows/`.
9. **First release**: tag `v0.2.0` (or `v1.0.0-alpha.1`, depending on stability stance) as the first release under the new name. Publish to crates.io.
10. **Optional**: reserve `phi-detector` on crates.io with a `0.0.0` placeholder that yanks itself or points to `healthwand`. Decided based on whether any user-visible adoption of the old name exists.

The migration is a single PR (or a small series), not a long-running branch.

---

## 12. Open architectural questions (deferred)

These are real unknowns that this document deliberately does not resolve. They are flagged here so that v1.x and v2.0 planning can address them with explicit decisions.

| Question                                                          | Decision deferred to                              | Notes                                                                                          |
| ----------------------------------------------------------------- | ------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| WASM detector hosting                                             | v2.0+ planning                                    | Only if demand emerges                                                                         |
| Cross-platform binary distribution tooling                        | Modernization audit                               | `cargo-dist`, `dist`, or custom — depends on what works in 2026                                |
| Streaming scan for very large files                               | v1.x                                              | Currently capped at 50MB by default                                                            |
| GitHub Action repo split                                          | Pre-v1.0                                          | `MedAIFort/healthwand-action` or in-repo subdirectory?                                         |
| Permenkes article-level citations in `docs/regulatory-mapping.md` | When Permenkes 24/2022 is read article-by-article | Currently structure-level; refining to article-level requires direct reading of the regulation |
| Combinatorial engine implementation strategy                      | v1.x                                              | Stream-based co-occurrence within sliding window, or batch post-processing?                    |
| Python NLP CLI design parity with Rust CLI                        | v1.x                                              | Same flags? Same exit codes? Same YAML loading?                                                |

---

## 13. Versioning and maintenance

This document is versioned alongside the repository. Updates triggered by:

- Resolution of any open question in §12.
- Major architectural changes (new component, removed component).
- Migration milestones (rename completion, first release).
- v1.x and v2.0 planning rounds.

---

## 14. Change log

- **v0.1.0-draft.1** (2026-05-13) — Initial architecture lock. Single Rust crate `healthwand` (lib + bin), hexagonal layering, Python NLP companion (v1.x rule-based, v2.0+ transformer opt-in), API server deferred, migration plan from `phi-detector`. Three decisions ratified: Python NLP shape, API server scope, crate rename.
