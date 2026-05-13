# TODO — HealthWand Revival

**Status:** `v0.1.0-draft.1`
**Locked:** 2026-05-13
**Derived from:**

- `POSITIONING.md` v0.1.0-draft.1
- `README.md` v0.2.0-draft.1
- `docs/regulatory-mapping.md` v0.1.0-draft.1
- `docs/phi-taxonomy-id.md` v0.1.0-draft.1
- `ARCHITECTURE.md` v0.1.0-draft.1

**License of this document:** MIT (same as repo)

---

## How to use this document

This is the complete atomic task list for HealthWand's revival from its current stalled state to v1.0 and beyond. It is structured for solo work or AI-coding-agent delegation.

**Task ID format:** `T-NNNN` (zero-padded, monotonically assigned). IDs are stable — never reuse; if a task is dropped, mark `[ABANDONED]` and keep the ID.

**Status conventions:**

- `[ ]` — not started
- `[~]` — in progress
- `[x]` — done
- `[!]` — blocked (see flag)
- `[-]` — abandoned

**Flags (in order of severity / importance):**

- `[BLOCKER]` — blocks downstream work until resolved
- `[DECISION]` — requires explicit decision before action
- `[REPO-STATE]` — depends on inspecting actual current repo state
- `[EXISTS-PARTIAL]` — code likely exists in current `phi-detector/`; audit before reimplementing (Chesterton's Fence)
- `[VERSION-OUTDATED]` — modernization required (deps, CI versions, MSRV)
- `[REGULATORY]` — depends on regulatory primary source verification
- `[COMMUNITY]` — opt-in / external contribution territory; not maintainer-blocking
- `[DOCS]` — pure documentation task
- `[CI]` — CI configuration task
- `[SECURITY]` — security-related task

**Reference convention:** Each task cites the design document(s) it derives from in brackets, e.g., `[ARCH §2.8]`, `[TAX §2.1]`, `[POS §3]`, `[REG §1.2]`, `[README #usage]`. Abbreviations: `POS` = POSITIONING, `ARCH` = ARCHITECTURE, `REG` = regulatory-mapping, `TAX` = phi-taxonomy-id, `README` = README.

**Acceptance criteria** are included only where ambiguity is likely. Most tasks are self-evident from their description.

---

## Milestone overview

| Milestone | Version    | Theme                                       | Estimated task count |
| --------- | ---------- | ------------------------------------------- | -------------------- |
| M0        | (pre-work) | Foundation & audit                          | ~12                  |
| M1        | v0.2.0     | Migration: `phi-detector` → `healthwand`    | ~20                  |
| M2        | v0.3.0     | Hexagonal refactor: domain, detect, scanner | ~25                  |
| M3        | v0.4.0     | Indonesian pattern catalogue                | ~30                  |
| M4        | v0.5.0     | CLI surface + GitHub Action                 | ~18                  |
| M5        | v0.6.0     | Docs, tests, benchmarks                     | ~15                  |
| M6        | v1.0.0     | Stabilization & release                     | ~12                  |
| M7        | v1.x       | Python NLP companion + combinatorial engine | ~25 (planned)        |
| M8        | v2.0+      | Transformer NLP, API server (if demand)     | ~15 (planned)        |

---

## M0 — Foundation & audit (pre-work, before any code changes)

These tasks unblock everything downstream. None of them produce code; all produce knowledge or decisions.

- [x] **T-0001** `[BLOCKER][REPO-STATE]` Inspect current repo state. Capture `Cargo.toml`, `Cargo.lock` (if present), all CI workflows under `.github/workflows/`, full file tree (top 3 levels), and last-commit dates per major path. Output to `audit-2026-05.md` in a scratch branch.
- [x] **T-0002** `[BLOCKER][REPO-STATE]` Verify the build status of `phi-detector/`. Document: (a) does `cargo build --release` succeed on current `main`? (b) does `cargo test` pass? (c) what warnings does `cargo clippy` produce? Output to the audit file.
- [x] **T-0003** `[BLOCKER]` Check crates.io availability for the name `healthwand`. If taken: pick fallback per [ARCH §11 step 1]. Suggested fallbacks in order: `healthwand-rs`, `healthwand-phi`, `healthwand-scan`. Document the result in the audit file. [ARCH §11]
- [x] **T-0004** `[DECISION][BLOCKER]` Decide MSRV (Minimum Supported Rust Version) for v1.0. Pre-revival badge says 1.65 — that is no longer reasonable. Recommended: stable minus two minor versions, validated empirically. Run `cargo msrv find` on the current code to determine the actual current MSRV.
  - Decided MSRV: **Rust 1.87.0** (as recorded in audit).
- [ ] **T-0005** `[DECISION][BLOCKER]` Decide Python floor for the planned NLP companion. Pre-revival badge says 3.9 — EOL Oct 2025. Recommend 3.11+ minimum.
- [ ] **T-0006** `[REPO-STATE]` Audit `.taskmaster/` directory contents. The current README references it; decide whether to keep, retire, or migrate the Task Master configuration. If retire: open a follow-up task to remove the directory.
- [ ] **T-0007** `[REPO-STATE]` Audit existing `CHANGELOG.md` and `CONTRIBUTING.md` (if present). If not present, flag for creation in M5.
- [ ] **T-0008** `[REPO-STATE]` Audit existing `LICENSE` file — confirm MIT, current year, correct copyright holder line. The MIT license is locked; this is verification only.
- [ ] **T-0009** `[DECISION]` Decide GitHub Action wrapper repo strategy: (a) `MedAIFort/healthwand-action` as a separate repo, or (b) action lives in-tree under `.github/actions/healthwand/`. Recommendation: (a) for independent action versioning. [ARCH §3.5]
- [ ] **T-0010** `[DECISION]` Decide whether to keep `phi-detector` reserved on crates.io (yank-deprecate) post-rename, or release the name. Recommendation: release; no significant pre-revival adoption. [ARCH §11 step 10]
- [ ] **T-0011** `[REPO-STATE]` Identify any third-party references to `phi-detector` (mentions in other repos, blog posts, Substack, social media). If found, plan announcement of the rename. [ARCH §11]
- [ ] **T-0012** Commit `audit-2026-05.md` to the scratch branch as the M0 deliverable. All M1+ work proceeds against this audit.

---

## M1 — Migration: `phi-detector` → `healthwand` (target v0.2.0)

Steps follow `[ARCH §11]` exactly. Single PR per logical step; the whole milestone should be merged within a small series of PRs, not a long-running branch.

### M1.1 Repository structure changes

- [ ] **T-0100** `[EXISTS-PARTIAL]` Move `phi-detector/src/` → repo root `src/`. Preserve verbatim per Chesterton's Fence; reorganization happens in M2. [ARCH §11 step 2]
- [ ] **T-0101** `[EXISTS-PARTIAL]` Move `phi-detector/config/` → repo root `config/`. Update any in-code references.
- [ ] **T-0102** `[EXISTS-PARTIAL]` Move `phi-detector/docs/` content → repo root `docs/`, merging with existing `docs/` (which contains the five design documents). Resolve naming conflicts.
- [ ] **T-0103** `[EXISTS-PARTIAL]` Move `phi-detector/tests/` → repo root `tests/`.
- [ ] **T-0104** Remove the now-empty `phi-detector/` directory.

### M1.2 Cargo manifest

- [ ] **T-0110** Rewrite `Cargo.toml`:
  - [ ] T-0110.1 Set `[package].name = "healthwand"` (or fallback per T-0003)
  - [ ] T-0110.2 Set `[package].version = "0.2.0"`
  - [ ] T-0110.3 Set `[package].edition = "2024"` (current latest stable as of 2026)
  - [ ] T-0110.4 Set `[package].rust-version` to the MSRV decided in T-0004
  - [ ] T-0110.5 Set `[package].authors` correctly (single maintainer per current state)
  - [ ] T-0110.6 Set `[package].license = "MIT"`
  - [ ] T-0110.7 Set `[package].repository`, `homepage`, `documentation` URLs
  - [ ] T-0110.8 Set `[package].keywords = ["phi", "healthcare", "indonesia", "uu-pdp", "redaction"]` (crates.io max 5)
  - [ ] T-0110.9 Set `[package].categories` appropriately (likely "command-line-utilities" + "data-structures" or similar)
  - [ ] T-0110.10 Add `[lib]` section with `name = "healthwand"`, `path = "src/lib.rs"`
  - [ ] T-0110.11 Add `[[bin]]` section with `name = "healthwand"`, `path = "src/bin/healthwand.rs"`

### M1.3 Binary rename

- [ ] **T-0120** Move `src/main.rs` → `src/bin/healthwand.rs`. Add `src/lib.rs` as the public API entry point. [ARCH §2.1]
- [ ] **T-0121** Update binary references in `src/bin/healthwand.rs` from any internal `phi-detector` strings to `healthwand`.
- [ ] **T-0122** `[EXISTS-PARTIAL]` Audit CLI flag compatibility: ensure `--input`, `--output`, `--redact`, `-v`/`-vv` continue to accept the same values they did pre-rename. [ARCH §3, README #usage]

### M1.4 CI rename

- [ ] **T-0130** `[CI]` Rewrite all `.github/workflows/*.yml` to replace `phi-detector` references with `healthwand`. Examples: build directories, binary paths, artifact names.
- [ ] **T-0131** `[CI]` Update workflow Rust toolchain pin to the MSRV from T-0004.

### M1.5 Documentation rename

- [ ] **T-0140** `[DOCS]` `README.md` is already drafted with `healthwand` naming. Verify no residual `phi-detector` references in body text (clone URLs are unchanged; binary name should already be `healthwand`).
- [ ] **T-0141** `[DOCS]` Update `POSITIONING.md` §11 "phi-detector rename" — mark the rename decision as RESOLVED with reference to this milestone.
- [ ] **T-0142** `[DOCS]` Update `ARCHITECTURE.md` §12 deferred questions table — mark "phi-detector rename" as RESOLVED.

### M1.6 Migration commit hygiene

- [ ] **T-0150** Ensure git history preserves blame across moves. Use `git mv` (not delete-and-add) for all file movements. Each move in its own commit if necessary.
- [ ] **T-0151** Tag the migration completion: `v0.2.0-rc.1` for testing; promote to `v0.2.0` after CI green.
- [ ] **T-0152** Open a GitHub Discussion or pinned issue announcing the rename, for any pre-revival user reference.

---

## M2 — Hexagonal refactor (target v0.3.0)

Reorganize source per `[ARCH §2.8]`. The domain layer becomes I/O-free; adapters move to their own modules. No new features in this milestone — pure reorganization.

### M2.1 Domain module

- [ ] **T-0200** `[EXISTS-PARTIAL]` Create `src/domain/` directory with `mod.rs`. [ARCH §2.3, §2.8]
- [ ] **T-0201** Implement `src/domain/severity.rs`:
  - [ ] T-0201.1 `Severity` enum: `Informational`, `Medium`, `High`, `Critical` (ordered)
  - [ ] T-0201.2 `impl Ord, PartialOrd` so that severity comparisons work
  - [ ] T-0201.3 `impl Display, FromStr` for serde and CLI flags
- [ ] **T-0202** Implement `src/domain/score.rs`:
  - [ ] T-0202.1 `Score(f32)` newtype with `Score::new(f32) -> Result<Self>` validating `0.0..=1.0`
  - [ ] T-0202.2 Parse-don't-validate: only construct via `new`; no public `Score(0.95)` literal
- [ ] **T-0203** Implement `src/domain/span.rs`:
  - [ ] T-0203.1 `MatchSpan { start: usize, end: usize, line: u32, column: u32 }`
  - [ ] T-0203.2 Helper: compute line/column from byte offsets given source text
- [ ] **T-0204** Implement `src/domain/pattern.rs`:
  - [ ] T-0204.1 `PatternId(String)` newtype with construction validation (no whitespace, no quotes — see system constraints in your guidebook)
  - [ ] T-0204.2 `Pattern` struct per [ARCH §2.3]
  - [ ] T-0204.3 `DetectorType` enum exhaustive
- [ ] **T-0205** Implement `src/domain/finding.rs`:
  - [ ] T-0205.1 `Finding` struct per [ARCH §2.3]
  - [ ] T-0205.2 `UuPdpArticle` enum for the optional regulatory cross-reference [REG §1.2]
- [ ] **T-0206** Implement `src/domain/category.rs`:
  - [ ] T-0206.1 `Category` enum matching UU PDP Article 4 split: `GeneralPersonalData`, `SpecificPersonalData(SpecificCategory)`
  - [ ] T-0206.2 `SpecificCategory` enum: `Health`, `Biometric`, `Genetic`, `Children`, `CriminalRecord`, `Financial` [REG §1.1]
- [ ] **T-0207** Verify: `src/domain/` has no imports from `src/io`, `src/format`, `src/config`. Add a `#![forbid(...)]` lint at module level if Rust supports per-module dep-restriction (or document the invariant).

### M2.2 Error type

- [ ] **T-0210** Implement `src/error.rs`:
  - [ ] T-0210.1 `HealthwandError` enum with `thiserror::Error` derive
  - [ ] T-0210.2 Variants: `ConfigError`, `IoError`, `RegexError`, `YamlError`, `UnsupportedDetector`, etc.
  - [ ] T-0210.3 `pub type Result<T> = std::result::Result<T, HealthwandError>;` in `lib.rs`
- [ ] **T-0211** Audit: no `.unwrap()` or `.expect()` in non-test, non-binary code. Replace with `?` and proper error variants. [ARCH §8.1]

### M2.3 Detector trait & implementations

- [ ] **T-0220** Implement `src/detect/mod.rs`:
  - [ ] T-0220.1 `Detector` trait per [ARCH §2.4]
  - [ ] T-0220.2 Trait is `Send + Sync` for parallelism
- [ ] **T-0221** `[EXISTS-PARTIAL]` Implement `src/detect/regex_detector.rs`:
  - [ ] T-0221.1 `RegexDetector` struct holding a compiled `regex::Regex` per pattern
  - [ ] T-0221.2 Handles both `DetectorType::Regex` and `DetectorType::RegexWithContext`
  - [ ] T-0221.3 Context window check uses sliding-window byte offsets
- [ ] **T-0222** Implement `src/detect/dictionary.rs`:
  - [ ] T-0222.1 `DictionaryDetector` struct with term set loaded from external file
  - [ ] T-0222.2 Aho-Corasick or similar multi-pattern matcher (the `aho-corasick` crate is conventional)
- [ ] **T-0223** Implement `src/detect/nlp_stub.rs`:
  - [ ] T-0223.1 `NlpStubDetector` that emits a warning when a `DetectorType::Nlp` pattern is encountered but NLP is not enabled
  - [ ] T-0223.2 Returns empty findings vector (does not falsely report) [ARCH §2.4]
- [ ] **T-0224** Deferred to M7: `src/detect/combinatorial.rs` (combinatorial engine). [ARCH §12]

### M2.4 Scanner orchestrator

- [ ] **T-0230** Implement `src/scanner/mod.rs`:
  - [ ] T-0230.1 `Scanner` struct holding catalogue + registered detectors + config
  - [ ] T-0230.2 `ScannerBuilder` per [ARCH §2.5]
  - [ ] T-0230.3 `scan_text(&str) -> ScanReport`
  - [ ] T-0230.4 `scan_path(&Path) -> Result<ScanReport>` with `rayon` parallelism
- [ ] **T-0231** Implement `src/scanner/report.rs`:
  - [ ] T-0231.1 `ScanReport` struct
  - [ ] T-0231.2 Aggregation: total findings, by severity, by pattern, by file
  - [ ] T-0231.3 Filter helpers: `findings_at_or_above(severity)` for CI gates

### M2.5 YAML configuration loader

- [ ] **T-0240** Implement `src/config/yaml_schema.rs`:
  - [ ] T-0240.1 `PatternYamlDto` struct mirroring the YAML schema [ARCH §5, TAX §0.3]
  - [ ] T-0240.2 `serde` derives for deserialization
  - [ ] T-0240.3 `schema_version` field on the root catalogue
- [ ] **T-0241** Implement `src/config/parse.rs`:
  - [ ] T-0241.1 Boundary parse: `parse_catalogue(yaml_bytes) -> Result<PatternCatalogue>` [ARCH §2.6]
  - [ ] T-0241.2 Each `PatternYamlDto` → `Pattern` (compiles regex, validates score, etc.)
  - [ ] T-0241.3 Errors include yaml-line precise locations
- [ ] **T-0242** Implement `src/config/mod.rs`:
  - [ ] T-0242.1 `PatternCatalogue` type (Vec<Pattern> wrapped with helpers)
  - [ ] T-0242.2 `load_yaml(path: &Path) -> Result<PatternCatalogue>` reads + parses
  - [ ] T-0242.3 `load_default() -> PatternCatalogue` loads built-in patterns

### M2.6 I/O adapters

- [ ] **T-0250** Implement `src/io/walker.rs`:
  - [ ] T-0250.1 Use `ignore` crate (BurntSushi); honors `.gitignore` by default
  - [ ] T-0250.2 Configurable include/exclude globs
  - [ ] T-0250.3 Default extensions: `.txt`, `.md`, `.csv`, `.json`, `.yaml`, `.yml`, source files of common languages (configurable)
- [ ] **T-0251** Implement `src/io/reader.rs`:
  - [ ] T-0251.1 UTF-8 validation at read time
  - [ ] T-0251.2 `--max-file-size` enforcement; skip files over limit with warning
  - [ ] T-0251.3 v1.x will add streaming; v1.0 reads fully into memory

### M2.7 Output formatters

- [ ] **T-0260** Implement `src/format/mod.rs`:
  - [ ] T-0260.1 `Formatter` trait per [ARCH §2.7]
  - [ ] T-0260.2 `Format` enum: `Json`, `Sarif`, `Text`
- [ ] **T-0261** Implement `src/format/json.rs`:
  - [ ] T-0261.1 Structured `JsonFinding` with all fields from `Finding`
  - [ ] T-0261.2 Output schema documented in `phi-detector/docs/output_format.md` (rename to `docs/output_format.md`)
- [ ] **T-0262** Implement `src/format/sarif.rs`:
  - [ ] T-0262.1 SARIF 2.1.0 compliant output
  - [ ] T-0262.2 Each `Finding` becomes a SARIF `result` with `ruleId`, `level`, `locations`
  - [ ] T-0262.3 Severity mapping: `Critical/High → error`, `Medium → warning`, `Informational → note`
- [ ] **T-0263** Implement `src/format/text.rs`:
  - [ ] T-0263.1 Human-readable output with colored severity indicators (use `colored` or `anstream` crate)
  - [ ] T-0263.2 Respect `NO_COLOR` env var

### M2.8 Public API surface

- [ ] **T-0270** Write `src/lib.rs` with all `pub use` statements per [ARCH §2.1].
- [ ] **T-0271** Add `#![deny(missing_docs)]` to `lib.rs` and document every public item.
- [ ] **T-0272** Run `cargo public-api` (or `cargo semver-checks`) and verify the public surface matches what's documented.

### M2.9 Refactor verification

- [ ] **T-0280** All existing tests must pass against the reorganized code. Tests may need import-path updates but not logic changes.
- [ ] **T-0281** `cargo clippy --all-targets -- -D warnings` must pass.
- [ ] **T-0282** Tag `v0.3.0` after CI green.

---

## M3 — Indonesian pattern catalogue (target v0.4.0)

Implement all 13 detectors documented in `[TAX §1]`. Each detector gets a YAML entry, integration tests with synthetic data, and documentation.

### M3.1 YAML schema implementation

- [ ] **T-0300** Implement YAML schema v1.0 per [ARCH §5.1]:
  - [ ] T-0300.1 `schema_version` field required on root
  - [ ] T-0300.2 Strict mode: unknown fields error (not ignored)
  - [ ] T-0300.3 Schema documented in `docs/yaml-schema.md`
- [ ] **T-0301** `[DOCS]` Write `docs/yaml-schema.md` documenting every YAML field, with examples per detector type.

### M3.2 Default pattern catalogue file

- [ ] **T-0310** Create `config/phi-patterns.yaml` as the shipped default. Versioned with the crate.
- [ ] **T-0311** Embed `config/phi-patterns.yaml` into the binary at build time using `include_str!` so `healthwand` works without an external config file.

### M3.3 Specific personal data detectors (UU PDP Art 4(1))

Each task implements the detector in YAML, adds integration tests with synthetic data, and verifies severity/category against [REG §2.1] and [TAX §2].

- [ ] **T-0320** Implement BPJS Kesehatan detector [TAX §2.1]:
  - [ ] T-0320.1 YAML entry with regex `\b\d{13}\b` and context words
  - [ ] T-0320.2 Integration test: positive match with `BPJS` context, negative match without
  - [ ] T-0320.3 Verify severity = Critical, category = Health
- [ ] **T-0321** Implement No. Rekam Medis detector [TAX §2.2]:
  - [ ] T-0321.1 YAML entries for both prefixed (`RM-`, `MRN-`) and bare-digit variants
  - [ ] T-0321.2 Bare-digit variant requires context words
  - [ ] T-0321.3 Integration tests for both variants
- [ ] **T-0322** Implement ICD-10 detector [TAX §2.3]:
  - [ ] T-0322.1 YAML entry with regex `\b[A-TV-Z]\d{2}(?:\.\d{1,2})?\b`
  - [ ] T-0322.2 Severity escalates to Critical with patient-context co-occurrence (combinatorial — actual escalation engine in M7)
- [ ] **T-0323** Implement NPWP detector [TAX §2.4]:
  - [ ] T-0323.1 YAML entries for legacy 15-digit (formatted and unformatted) and current 16-digit
  - [ ] T-0323.2 Document the NIK ↔ NPWP-2024 collision in YAML comments [TAX §4.1]
  - [ ] T-0323.3 Severity = Critical, category = Financial
- [ ] **T-0324** Implement Drug name detector [TAX §2.5]:
  - [ ] T-0324.1 Create `config/drugs-fornas-id.yaml` — FORNAS generic name subset (top ~200)
  - [ ] T-0324.2 Dictionary-type detector loads the YAML
  - [ ] T-0324.3 Context words required for severity escalation
  - [ ] T-0324.4 `[COMMUNITY]` Solicit Indonesian-market brand name contributions in CONTRIBUTING.md
- [ ] **T-0325** Children's-data flag detector [TAX §2.6] — DEFERRED to M7 (requires combinatorial engine).
- [ ] **T-0326** Diagnosis text detector [TAX §2.7] — DEFERRED to M7 (NLP-required; companion package).

### M3.4 General personal data detectors (UU PDP Art 4(2))

- [ ] **T-0330** Implement NIK (KTP) detector [TAX §3.1]:
  - [ ] T-0330.1 YAML entry with regex `\b\d{16}\b` and context words `[NIK, KTP, Nomor Induk Kependudukan]`
  - [ ] T-0330.2 Structural validation: optional flag `validate_structure: true` enforces province code in 11–94 range
  - [ ] T-0330.3 Document the NIK ↔ NPWP-2024 collision [TAX §4.1]
  - [ ] T-0330.4 Document the embedded DOB at positions 7–12
- [ ] **T-0331** Implement Indonesian phone detector [TAX §3.2]:
  - [ ] T-0331.1 YAML entry with regex `(?:\+62|62|0)8\d{8,11}`
  - [ ] T-0331.2 Separator-tolerant variant as additional pattern
  - [ ] T-0331.3 No context word required (high-precision prefix)
- [ ] **T-0332** Implement Indonesian address tokens detector [TAX §3.3]:
  - [ ] T-0332.1 YAML entry for composite token regex
  - [ ] T-0332.2 Co-occurrence rule: severity High when ≥3 distinct tokens within sliding window (combinatorial — actual engine in M7)
- [ ] **T-0333** Implement Indonesian DOB detector [TAX §3.4]:
  - [ ] T-0333.1 YAML entries for numeric formats (`dd/mm/yyyy`, `dd-mm-yyyy`, `dd.mm.yyyy`)
  - [ ] T-0333.2 YAML entry for Bahasa month name format
  - [ ] T-0333.3 Context words `[lahir, tanggal lahir, DOB, kelahiran]` required for severity
- [ ] **T-0334** Implement Indonesian names detector (regex-based portion) [TAX §3.5]:
  - [ ] T-0334.1 YAML entry for honorific + capitalized tokens regex
  - [ ] T-0334.2 Document that production-quality detection requires the NLP companion (planned M7)
- [ ] **T-0335** Implement STR detector [TAX §3.6]:
  - [ ] T-0335.1 YAML entry requiring the `STR` token explicitly
  - [ ] T-0335.2 Conservative bounds: digit count 6–20
  - [ ] T-0335.3 `[COMMUNITY]` Solicit council-specific patterns

### M3.5 Pattern catalogue validation

- [ ] **T-0340** Write a CLI subcommand `healthwand validate <path>` that validates a YAML file against the schema [ARCH §3.2].
- [ ] **T-0341** Add CI step: validate `config/phi-patterns.yaml` against the schema on every PR.
- [ ] **T-0342** Document each detector's rationale and false-positive vectors in inline YAML comments referencing [TAX §N.N].

### M3.6 Integration test fixtures

- [ ] **T-0350** Create `tests/fixtures/synthetic_phi/` with curated test files:
  - [ ] T-0350.1 `positive/` — files containing each detector's pattern with context
  - [ ] T-0350.2 `negative/` — files containing patterns without context (should not flag)
  - [ ] T-0350.3 `mixed/` — realistic combinations of patterns and non-patterns
  - [ ] T-0350.4 All data is synthetic — never real PHI [TAX §0.4]
- [ ] **T-0351** Integration test: scan `positive/` → expect specific findings per file.
- [ ] **T-0352** Integration test: scan `negative/` → expect no findings or below-threshold findings only.

### M3.7 Milestone verification

- [ ] **T-0360** All 11 (regex/dictionary) detectors detect their target patterns with synthetic test data.
- [ ] **T-0361** Tag `v0.4.0` after CI green.

---

## M4 — CLI surface + GitHub Action (target v0.5.0)

### M4.1 CLI flags and subcommands

- [ ] **T-0400** Implement full CLI flag surface per [ARCH §3.1]:
  - [ ] T-0400.1 `--input <PATH>` (required)
  - [ ] T-0400.2 `--output <json|sarif|text>` (default text)
  - [ ] T-0400.3 `--config <PATH>` (additional YAML config)
  - [ ] T-0400.4 `--config-include <PATH>` (merge into default)
  - [ ] T-0400.5 `--min-severity <informational|medium|high|critical>`
  - [ ] T-0400.6 `--redact` (in-line redaction)
  - [ ] T-0400.7 `--max-file-size <SIZE>` (default 50MB)
  - [ ] T-0400.8 `--include <GLOB>` / `--exclude <GLOB>` (repeatable)
  - [ ] T-0400.9 `-v` / `-vv` / `-vvv` verbosity flags
  - [ ] T-0400.10 `--catalogue-print` (print active catalogue and exit)
  - [ ] T-0400.11 `--list-detectors` (list registered detectors)
- [ ] **T-0401** Implement subcommand `healthwand validate <CONFIG>` per T-0340.
- [ ] **T-0402** Implement subcommand `healthwand explain <FINDING_ID>` — prints regulatory mapping for the named pattern from [REG §2].
- [ ] **T-0403** Implement subcommand `healthwand version` — prints version, git SHA, build date, target triple.

### M4.2 Exit codes

- [ ] **T-0410** Implement exit codes per [ARCH §3.4]:
  - [ ] T-0410.1 `0` — no findings at/above threshold
  - [ ] T-0410.2 `1` — findings at/above threshold (CI gate trigger)
  - [ ] T-0410.3 `2` — config error
  - [ ] T-0410.4 `3` — runtime error
  - [ ] T-0410.5 `64` — usage error
- [ ] **T-0411** Document exit codes in CLI `--help` output and in `README.md` quickstart.

### M4.3 Configuration precedence

- [ ] **T-0420** Implement config precedence per [ARCH §3.3]:
  - [ ] T-0420.1 CLI flags (highest)
  - [ ] T-0420.2 Env vars (`HEALTHWAND_*`)
  - [ ] T-0420.3 Repo-local `.healthwand.yaml`
  - [ ] T-0420.4 User config `~/.config/healthwand/config.yaml`
  - [ ] T-0420.5 Built-in defaults
- [ ] **T-0421** Test: each level overrides lower levels.

### M4.4 GitHub Action wrapper

- [ ] **T-0430** `[DECISION resolved in T-0009]` Create `MedAIFort/healthwand-action` repo (separate from main repo).
- [ ] **T-0431** Implement Action wrapper:
  - [ ] T-0431.1 `action.yml` declaring inputs (`input`, `min-severity`, `output-format`, `config`)
  - [ ] T-0431.2 Composite action that downloads pinned `healthwand` binary, runs it, uploads SARIF
  - [ ] T-0431.3 Cache binary by version for fast subsequent runs
- [ ] **T-0432** `[DOCS]` Write Action README with usage examples.
- [ ] **T-0433** `[CI]` Tag Action `v1` (rolling major), `v1.0.0` (specific) — standard Action versioning convention.

### M4.5 Milestone verification

- [ ] **T-0440** End-to-end test: invoke `healthwand` via CLI against a repo with mixed PHI, validate JSON and SARIF outputs.
- [ ] **T-0441** End-to-end test: invoke the GitHub Action against a test repo, validate SARIF appears in PR review.
- [ ] **T-0442** Tag `v0.5.0` after CI green.

---

## M5 — Docs, tests, benchmarks (target v0.6.0)

### M5.1 Supporting documentation

- [ ] **T-0500** `[DOCS]` Write `CHANGELOG.md`:
  - [ ] T-0500.1 Follow Keep a Changelog format
  - [ ] T-0500.2 Backfill v0.2.0 (rename), v0.3.0 (hexagonal refactor), v0.4.0 (catalogue), v0.5.0 (CLI/Action) entries
  - [ ] T-0500.3 Document the v0.2.0 rename prominently
- [ ] **T-0501** `[DOCS]` Write `CONTRIBUTING.md`:
  - [ ] T-0501.1 Pattern contribution workflow (highest-value contribution per POSITIONING.md §4)
  - [ ] T-0501.2 Code contribution standards (rustfmt, clippy, semver-checks)
  - [ ] T-0501.3 Reference to anti-goals in [POS §3]
  - [ ] T-0501.4 DCO sign-off or CLA decision [DECISION needed]
- [ ] **T-0502** `[DOCS][SECURITY]` Write `SECURITY.md`:
  - [ ] T-0502.1 Reporting process (private channel, e.g., GitHub Security Advisory)
  - [ ] T-0502.2 Supported versions for security patches (latest minor + previous)
- [ ] **T-0503** `[DOCS]` Write `CODE_OF_CONDUCT.md`:
  - [ ] T-0503.1 Adopt Contributor Covenant v2.1 (or current latest)
- [ ] **T-0504** `[DOCS]` Issue templates: `bug_report.md`, `feature_request.md`, `pattern_request.md` (the third is HealthWand-specific).
- [ ] **T-0505** `[DOCS]` PR template referencing [POS §3] anti-goals.

### M5.2 Documentation refinement

- [ ] **T-0510** `[DOCS][REGULATORY]` Refine `docs/regulatory-mapping.md` §3.1 Permenkes 24/2022 to article-level citations. Requires reading the official Permenkes text. [ARCH §12]
- [ ] **T-0511** `[DOCS]` Move `phi-detector/docs/output_format.md` → `docs/output_format.md`. Update for current JSON/SARIF schema.
- [ ] **T-0512** `[DOCS]` Write `docs/yaml-schema.md` (already noted in T-0301) with detailed schema reference.
- [ ] **T-0513** `[DOCS]` Write `docs/cli-reference.md` — full flag reference with examples.

### M5.3 Test coverage

- [ ] **T-0520** Audit test coverage with `cargo tarpaulin` or `cargo llvm-cov`. Target: ≥80% line coverage for `src/domain/` and `src/detect/`.
- [ ] **T-0521** Property-based tests for `Score::new` (proptest crate): every f32 in `0.0..=1.0` parses; outside range fails.
- [ ] **T-0522** Property-based tests for NIK structural validation: every valid 16-digit string with province ∈ 11..=94 parses.
- [ ] **T-0523** Fuzzing setup for the YAML loader (cargo-fuzz):
  - [ ] T-0523.1 Fuzz target: parse arbitrary YAML bytes → no panic, no infinite loop
  - [ ] T-0523.2 Initial corpus from `tests/fixtures/`
- [ ] **T-0524** Snapshot tests (insta crate) for output formatters: JSON output stable, SARIF output stable.

### M5.4 Benchmarks

- [ ] **T-0530** Create `benches/scan_throughput.rs`:
  - [ ] T-0530.1 Scan 100,000 lines of synthetic mixed text
  - [ ] T-0530.2 Target: <1s on a developer laptop [ARCH §8.4]
  - [ ] T-0530.3 Measure single-threaded and `rayon` parallel
- [ ] **T-0531** `[CI]` Add benchmark regression gate: fail CI on >5% slowdown vs previous release.
- [ ] **T-0532** Memory benchmarks (using `dhat` or similar) for large file scans.

### M5.5 Milestone verification

- [ ] **T-0540** All docs cross-link correctly. Linting with `markdown-link-check` in CI.
- [ ] **T-0541** Tag `v0.6.0` after CI green.

---

## M6 — Stabilization & v1.0 release (target v1.0.0)

### M6.1 SemVer freeze preparation

- [ ] **T-0600** `[CI]` Add `cargo-semver-checks` to CI:
  - [ ] T-0600.1 `cargo install cargo-semver-checks` (per user prefs, also add `cargo install cargo-skill`)
  - [ ] T-0600.2 CI runs `cargo semver-checks check-release` against the previous published release
  - [ ] T-0600.3 Breaking changes require explicit major-version bump
- [ ] **T-0601** Public API audit one more time:
  - [ ] T-0601.1 Run `cargo public-api`
  - [ ] T-0601.2 Document every public item
  - [ ] T-0601.3 Mark internal items `pub(crate)`
  - [ ] T-0601.4 Add `#![deny(missing_docs)]` and resolve all warnings
- [ ] **T-0602** Lock the YAML schema as v1.0. Subsequent additions are minor schema bumps; breaking changes require schema major bump.

### M6.2 Security hardening

- [ ] **T-0610** `[CI][SECURITY]` Add `cargo audit` to CI:
  - [ ] T-0610.1 Fail on any unmitigated advisory
  - [ ] T-0610.2 Document advisory exceptions in `audit.toml`
- [ ] **T-0611** `[SECURITY]` Add `cargo deny check` to CI:
  - [ ] T-0611.1 License compliance check
  - [ ] T-0611.2 Banned crate list (e.g., crates with known supply-chain issues)
- [ ] **T-0612** `[SECURITY]` Sign release artifacts:
  - [ ] T-0612.1 Sigstore / cosign signing for binary releases
  - [ ] T-0612.2 Document verification process in `SECURITY.md`

### M6.3 Distribution

- [ ] **T-0620** `[DECISION]` Select cross-platform binary distribution tooling — `cargo-dist`, `dist`, or alternative. [ARCH §12]
- [ ] **T-0621** Publish prebuilt binaries to GitHub Releases for:
  - [ ] T-0621.1 linux-x86_64
  - [ ] T-0621.2 linux-aarch64
  - [ ] T-0621.3 macos-arm64
  - [ ] T-0621.4 macos-x86_64
  - [ ] T-0621.5 windows-x86_64
- [ ] **T-0622** Publish `healthwand` v1.0.0 to crates.io.
- [ ] **T-0623** Promote GitHub Action wrapper to `v1.0.0` stable.

### M6.4 Launch readiness

- [ ] **T-0630** `[DOCS]` Final `README.md` review — verify all claims in [POS §10] are achievable.
- [ ] **T-0631** `[DOCS]` Verify [POS §7] regulatory claims still align with current Indonesian regulatory state (PP PDP status, etc.).
- [ ] **T-0632** Per [POS §10] success criteria: instrument GitHub repo for the measurable signals (search-discovery proxy, community pattern contributions, external references).
- [ ] **T-0633** Tag `v1.0.0`. Announce per [POS §9] Build phase completion.

---

## M7 — v1.x: Python NLP companion + combinatorial engine (target v1.1.0+)

This milestone covers the planned post-1.0 work. Tasks are coarser; subtasks will be elaborated as the milestone activates.

### M7.1 Python NLP companion package (rule-based)

- [ ] **T-0700** `[DECISION]` Confirm package name `healthwand-nlp` is available on PyPI.
- [ ] **T-0701** Bootstrap `python/` directory per [ARCH §4.5]:
  - [ ] T-0701.1 `uv init` with Python ≥3.11
  - [ ] T-0701.2 `pyproject.toml` with `healthwand-nlp` package
  - [ ] T-0701.3 Standard SDLC: ruff, mypy, pytest
- [ ] **T-0702** Implement domain types in Python (Pydantic models mirroring Rust types):
  - [ ] T-0702.1 `Pattern`, `Finding`, `Severity`, `Score` Pydantic models
  - [ ] T-0702.2 YAML loader using the shared schema
- [ ] **T-0703** Implement Bahasa name rules detector:
  - [ ] T-0703.1 Honorific + capitalized token sequences (matching Rust regex)
  - [ ] T-0703.2 Disambiguation rules (patient vs. non-patient context)
  - [ ] T-0703.3 Tests against synthetic Bahasa text
- [ ] **T-0704** Implement Bahasa diagnosis rules detector:
  - [ ] T-0704.1 Plain-language ICD-10 equivalents dictionary
  - [ ] T-0704.2 Common Indonesian medical vocabulary lookup
  - [ ] T-0704.3 Negation handling ("bukan diabetes" should not flag)
- [ ] **T-0705** Implement `healthwand-nlp` CLI:
  - [ ] T-0705.1 Flag parity with Rust CLI where meaningful [ARCH §12]
  - [ ] T-0705.2 JSON output schema identical to Rust
  - [ ] T-0705.3 Loads same YAML catalogue; processes only `type: nlp` entries
- [ ] **T-0706** Implement `healthwand-merge` companion utility for combining Rust + Python outputs.
- [ ] **T-0707** Publish `healthwand-nlp` v1.0.0 to PyPI.

### M7.2 Combinatorial engine

- [ ] **T-0710** Implement `src/detect/combinatorial.rs` per [ARCH §2.4 / TAX §5]:
  - [ ] T-0710.1 Sliding-window co-occurrence detection
  - [ ] T-0710.2 Configurable trigger rules in YAML
  - [ ] T-0710.3 Severity elevation per [TAX §5] table
- [ ] **T-0711** Wire into `Scanner` to run after primary detection.
- [ ] **T-0712** Integration tests for each combinatorial rule in [TAX §5].

### M7.3 Streaming scan for large files

- [ ] **T-0720** Implement streaming I/O for files >100MB per [ARCH §8.5].
- [ ] **T-0721** Bounded-memory guarantee verified in benchmarks.

### M7.4 Subcommand expansion

- [ ] **T-0730** Implement subcommands deferred from M4: any subcommand from [ARCH §3.2] not yet shipped.

### M7.5 Milestone verification

- [ ] **T-0740** Tag `v1.1.0` (or appropriate minor bump per actual work delivered).

---

## M8 — v2.0+: Transformer NLP, optional API server, image PHI (deferred)

Coarse-grained markers for the v2.0+ direction. Tasks elaborate when the milestone activates.

- [ ] **T-0800** `[COMMUNITY]` Evaluate Indonesian medical NER models on Hugging Face / IndoNLU benchmark for v2.0 transformer NLP. [ARCH §4.4]
- [ ] **T-0801** `[DECISION]` Whether to fine-tune a medical NER model. Requires labeled Bahasa medical data — availability is the decision input.
- [ ] **T-0802** Implement `healthwand-nlp[transformer]` extras for opt-in transformer detection.
- [ ] **T-0803** `[DECISION]` API server: implement only if real demand emerges. [POS §3, ARCH §1.2]
- [ ] **T-0804** Image PHI detection (DICOM burned-in metadata) per [TAX §8].
- [ ] **T-0805** `[DECISION]` WASM detector hosting per [ARCH §7.4].
- [ ] **T-0806** Re-evaluate the `medaifort.com` hosted dashboard — by v2.0, org-level commercial decisions can be revisited per [POS §3].

---

## Cross-cutting tasks (parallel to milestones)

These don't belong to a single milestone but recur throughout the project.

### XC.1 Maintenance

- [ ] **T-9000** `[CI]` Quarterly dependency audit and bumps via Dependabot / Renovate.
- [ ] **T-9001** `[REGULATORY]` Quarterly review of Indonesian regulatory state; update `docs/regulatory-mapping.md` when PP PDP is enacted or significant Permenkes changes happen. Trigger events listed in [REG §7].
- [ ] **T-9002** `[COMMUNITY]` Quarterly review of community-contributed patterns. Merge those aligned with POSITIONING; decline those moving toward anti-goals.

### XC.2 Documentation versioning

- [ ] **T-9010** `[DOCS]` Maintain version stamps in each design document. Significant changes increment the document version per its own change log section.

### XC.3 OSS sequencing discipline (Build → Works → Community)

- [ ] **T-9020** `[COMMUNITY]` Do NOT solicit community contributions before v1.0 ships. Per the user-preferred sequencing principle in [POS §9], the Build phase is private until working code is shipped.
- [ ] **T-9021** `[COMMUNITY]` First community outreach post: scheduled for v1.0 release announcement, not before.

---

## Decisions still open

These are decisions identified but not yet made. Listed here for visibility; each one is also flagged in the task that needs it.

| ID     | Decision                                            | Blocking | Recommendation                                                        |
| ------ | --------------------------------------------------- | -------- | --------------------------------------------------------------------- |
| T-0003 | Crates.io name (`healthwand` or fallback)           | M1       | Check before migration starts                                         |
| T-0004 | MSRV value for v1.0                                 | M1       | Stable minus two minor; verify with `cargo msrv find`                 |
| T-0005 | Python floor                                        | M7       | 3.11+                                                                 |
| T-0009 | GitHub Action wrapper repo location                 | M4       | Separate repo                                                         |
| T-0010 | `phi-detector` post-rename disposition on crates.io | M1       | Release (no significant adoption)                                     |
| T-0501 | DCO sign-off or CLA for contributions               | M5       | DCO is lighter; CLA only if enterprise contributions become a concern |
| T-0620 | Cross-platform binary distribution tooling          | M6       | `cargo-dist` is the current convention; verify in 2026                |
| T-0700 | `healthwand-nlp` PyPI name availability             | M7       | Check before package bootstrapping                                    |
| T-0801 | Transformer NLP fine-tuning vs. off-the-shelf       | M8       | Depends on labeled-data availability at the time                      |
| T-0803 | API server implementation                           | M8       | Implement only if real demand emerges                                 |

---

## Versioning and maintenance of this document

- **v0.1.0-draft.1** (2026-05-13) — Initial atomic TODO derived from 5 design documents. ~160 tasks across 8 milestones. Open decisions and cross-cutting tasks separately enumerated.

Significant additions (new milestone, fundamental restructure) trigger a minor bump. Task additions or refinements within existing structure are patch bumps.

Tasks are append-only by ID. Removed tasks are marked `[-]` (abandoned), never deleted.
