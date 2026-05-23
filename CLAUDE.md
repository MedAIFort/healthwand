# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Project Overview

**HealthWand** is a Rust CLI + library for detecting and redacting Indonesian Protected Health Information (PHI) per UU PDP (Law No. 27 of 2022). The codebase is in active revival with a hexagonal/ports-and-adapters architecture. Current phase: **M2 PHASE 1 complete** (error foundation + domain layer, as of 2026-05-23).

**Key links:**

- `POSITIONING.md` — Audience, anti-goals, regulatory claim discipline
- `ARCHITECTURE.md` — Technical design, component boundaries, public API surface
- `TODO.md` — Master atomic task list (v0.1.0-draft.3); single source of truth for phases M0–M8
- `Plans.md` — Current working phase tasks (generated from TODO.md, v2 format with DoD/Depends/Status)
- `README.md` — Quickstart, features, comparison to other tools

**Do not include `Co-Authored-By:` trailers in commit messages.** This applies to all assistant-generated commits, including those produced by Claude Code or any other AI tool. Commit attribution stays with the human author. Boilerplate trailers add noise to the history without conveying meaningful authorship and have been retroactively stripped from past commits.

---

## Build, Test, Lint

**MSRV:** Rust 1.87 (verified via `cargo msrv find`)  
**Edition:** 2024  
**Python floor (for NLP companion, planned v1.x):** 3.11+

### Common commands

```bash
# Build and check
cargo check --lib              # Compile library only (fast)
cargo build --release          # Release binary to target/release/healthwand
cargo install --path .         # Install to ~/.cargo/bin/healthwand

# Test
cargo test                      # Run all tests (28 passing as of 2026-05-23)
cargo test --lib               # Library tests only
cargo test --doc               # Doctests

# Linting and formatting
cargo fmt -- --check           # Check formatting (enforced by git hook)
cargo clippy --all-targets -- -D warnings  # Deny all warnings (enforced by git hook)
cargo msrv verify              # Verify MSRV 1.87

# Single test
cargo test --lib test_name     # Run one test by name

# Dependencies
cargo update                   # Refresh Cargo.lock
cargo deny check                # License/supply-chain audit (planned for M6)
cargo-semver-checks check-release  # Public API stability (M6 gate; currently advisory)
```

### Git hooks

The repository enforces at commit time:

- `cargo fmt` — formats all Rust code
- `cargo clippy` — rejects warnings in clippy pedantic mode
- `cargo test` — all tests must pass

Violations block commit. Pre-commit failures (e.g., unformatted files) trigger automatic formatting and require re-staging.

---

## Architecture & Code Organization

### Hexagonal/Ports-and-Adapters (Domain-Driven)

```
Domain Layer (src/domain/)
  ↑
  ├─ Core types: Severity, Score, MatchSpan, PatternId, Pattern, Finding, Category
  ├─ Detector trait (pluggable strategy for pattern matching)
  └─ Error enum + Result<T> type alias

Driven Adapters (convert external data to domain)
  ├─ src/config/ — YAML pattern loading
  ├─ src/io/ — File walking and reading
  └─ src/scanner/ — Orchestration of detectors over files

Driving Adapters (consume domain output)
  ├─ src/format/ — JSON, Text, SARIF output formatters
  └─ src/bin/healthwand.rs — CLI using clap

Invariant: Domain layer imports ONLY std + serde. Zero imports from io/, format/, config/, scanner/ modules. Domain types are testable in isolation.
```

### Key Files and Their Purpose

| File/Directory             | Purpose                                                                                                                                                                  |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `src/domain/`              | Core business logic (zero I/O). Types: `Severity`, `Score`, `MatchSpan`, `PatternId`, `Pattern`, `Finding`, `Category`. All public API is re-exported from domain/mod.rs |
| `src/error.rs`             | `HealthwandError` enum + `Result<T>` type alias. thiserror-derived. No `.unwrap()` in new code.                                                                          |
| `src/detect/`              | Detector implementations (Regex, Dictionary, stubs for NLP/Combinatorial)                                                                                                |
| `src/scanner/`             | `Scanner` orchestrator, `ScanConfig`, `ScanReport`. Routes patterns to detectors, accumulates findings                                                                   |
| `src/config/`              | YAML deserialization (config/yaml_schema.rs) and parsing (config/parse.rs). Converts DTO → domain::Pattern                                                               |
| `src/io/`                  | File walking (via `ignore` crate) and UTF-8 validation                                                                                                                   |
| `src/format/`              | Formatter trait + JSON/Text/SARIF implementations. No domain logic here.                                                                                                 |
| `src/lib.rs`               | Public API surface (stable from v1.0, enforced by semver-checks)                                                                                                         |
| `src/bin/healthwand.rs`    | CLI entry point (clap derives). Calls library API.                                                                                                                       |
| `config/phi_patterns.yaml` | Embedded YAML pattern catalogue (single source of truth, shared with Python NLP companion planned for v1.x)                                                              |
| `Cargo.toml`               | Dependencies: regex, serde, serde_yaml_ng (0.10), thiserror (2.0), clap, tracing, aho-corasick, rayon, ignore, colored, anyhow                                           |

### Design Tenets (Non-Negotiable)

1. **KISS / YAGNI** — Every component justifies existence against real use case
2. **Hexagonal** — Domain isolated from I/O and formatting
3. **Parse-Don't-Validate** — External data becomes typed values exactly once at the boundary
4. **Make-Illegal-States-Unrepresentable** — Newtypes for Severity, Score, PatternId; enums for DetectorType
5. **Library-first, binary-last** — The library is the system; the CLI is one consumer
6. **High cohesion, low coupling** — Modules by _what they do_, not _what they are_
7. **SemVer enforced** — Public API changes require major version bump; `cargo-semver-checks` will fail otherwise
8. **Principle of least privilege** — Components see only what they need
9. **Chesterton's Fence** — Existing code that solves its problem is preserved during migration/refactor
10. **Boy Scout Rule** — Leave code at least as well-organized as you found it

---

## Current Project State & Workflow

### Phase M2 PHASE 1 (Complete as of 2026-05-23)

**Deliverables:**

- Domain layer: 6 modules (severity, score, span, pattern, finding, category)
- Error foundation: HealthwandError enum + Result<T> type alias
- Zero I/O imports in domain/ verified
- All 28 tests passing, cargo check/fmt/clippy clean

**Commits:**

```
d5cc9c2 Add out/ to .gitignore
e611b81 Mark Phase 0 & Phase 1 complete in TODO.md
d570a9d M2 Phase 1 complete: error foundation & domain layer
```

### Task Tracking with Plans.md

This repo uses `Plans.md` for task tracking during development. It's generated from `TODO.md` and uses a v2 format:

```
| Task | Content | DoD | Depends | Status |
|------|---------|-----|---------|--------|
| 1.1  | Description | Definition of Done | Task IDs | cc:TODO / cc:WIP / cc:完了 |
```

**Markers:**

- `cc:TODO` — Not started
- `cc:WIP` — In progress
- `cc:完了` — Done (includes commit hash if applicable)

**Important:** Do NOT manually edit `Plans.md` for phase tracking. It's derived from `TODO.md`, which is the authoritative source. Edit `TODO.md` instead.

### Master Task List (TODO.md)

`TODO.md` is the single source of truth for all work across M0–M8. Tasks are atomic and ID-stable (T-NNNN format, never reused). Current state:

- **M0** (complete): Foundation audit, decisions
- **M1** (complete): Migration from phi-detector → healthwand, dependency modernization
- **M2 PHASE 0** (complete): Dependencies & YAML prep
- **M2 PHASE 1** (complete 2026-05-23): Error foundation & domain layer
- **M2 PHASE 2–11** (pending): YAML config → integration tests → cleanup
- **M3–M8** (planned): Pattern catalogue, CLI surface, docs, Python NLP, transformer NLP

When starting new work, read the relevant task(s) in TODO.md first to understand:

- What problem is being solved
- What definition of done is expected
- What tasks this one depends on

---

## Public API Surface (Stable from v1.0)

The library exports from `src/lib.rs`:

```rust
pub use domain::{Pattern, Finding, Severity, Score, MatchSpan, DetectorType, Category, UuPdpArticle};
pub use scanner::{Scanner, ScanConfig, ScanReport};
pub use detect::Detector;
pub use config::PatternCatalogue;
pub use format::{Format, JsonFormatter, TextFormatter, SarifFormatter};
pub use error::{HealthwandError, Result};
```

This surface is locked from v1.0. Breaking changes require major version bump. `cargo-semver-checks` enforces this in CI (M6 gate).

---

## Common Development Patterns

### Adding a detector

Implement the `Detector` trait in `src/detect/detector_name.rs`:

```rust
pub struct MyDetector;

impl Detector for MyDetector {
    fn detector_type(&self) -> DetectorType { DetectorType::Dictionary }
    fn scan(&self, text: &str, pattern: &Pattern) -> Vec<Finding> {
        // Return findings
    }
    fn handles(&self, pattern: &Pattern) -> bool {
        pattern.detector_type == DetectorType::Dictionary
    }
}
```

Register in `src/detect/mod.rs` and wire into `Scanner` (src/scanner/mod.rs).

### Adding a pattern

Edit `config/phi_patterns.yaml`:

```yaml
patterns:
  - id: custom-pattern-id
    name: Custom pattern name
    detector_type: regex
    category: Personal
    default_severity: High
    score: 0.9
    regex: '\b\d{8}\b'
    context_words: [keyword1, keyword2]
```

Patterns are loaded via `config::PatternCatalogue::load_default()` (built-in YAML) or `load_yaml(bytes)` (custom).

### Error handling (no unwrap/expect)

Use the `?` operator with `Result<T>`. Domain types that fail validation should be parsed at the boundary (config loader, CLI arg parser):

```rust
let score = Score::new(user_input_f32)?;  // Returns Err if out of range
let pattern_id = PatternId::new(user_string)?;  // Returns Err if whitespace
```

Internal code assumes legal states (types are made unrepresentable).

---

## Testing Strategy

- **Unit tests** — Test domain types and detectors in `src/**/*.rs` (use `#[cfg(test)]` modules)
- **Integration tests** — Test end-to-end scanning in `tests/` directory
- **Doctests** — Examples in public API docs (checked by CI)

Run tests frequently:

```bash
cargo test                     # All tests
cargo test --lib              # Library only (fast)
cargo test test_name -- --nocapture  # Run one test, show output
```

28 tests should pass. If a test breaks, investigate before committing.

---

## Dependency Management

**Currently locked dependencies (as of M2 Phase 1):**

- `regex` 1.11 — Pattern matching
- `serde` / `serde_json` 1.0 — Serialization (JSON output)
- `serde_yaml_ng` 0.10 — YAML loading (migrated from archived 0.9)
- `thiserror` 2.0 — Error handling (bumped from 1.0 in M1.7)
- `clap` 4.5 — CLI argument parsing
- `tracing` 0.1 — Structured logging (migrated from `log` in M1.7)
- `aho-corasick` 1.1 — Dictionary matching
- `rayon` 1.10 — Parallel iteration (not yet used, planned for M2.5+)
- `ignore` 0.4 — Filesystem traversal (gitignore-aware)
- `colored` 2.1 — Colored terminal output
- `anyhow` 1.0 — Error context (for binary, not lib public API)

Use `cargo update` after adding/bumping to refresh Cargo.lock. MSRV bumps require re-verification via `cargo msrv verify`.

---

## Documentation

- **README.md** — User-facing quickstart, features, roadmap
- **POSITIONING.md** — Audience, anti-goals, regulatory scope (v0.1.0-draft.1, locked 2026-05-13)
- **ARCHITECTURE.md** — Technical design, component boundaries, public API (v0.1.0-draft.2, locked 2026-05-13)
- **TODO.md** — Master task list, all planned work (v0.1.0-draft.3, living document)
- **CHANGELOG.md** — Release notes (backfilled during M5)

Write docs in Markdown. Render math with `$...$` (KaTeX). Reference specific sections via `[ARCH §2.3]` style.

---

## Regulatory & Compliance Context

HealthWand is built for **UU PDP** (Indonesian Law No. 27 of 2022) compliance. Key context:

- **Target data:** Health/medical information classified as "specific personal data" (data spesifik) under Article 4(1)
- **Breach notification:** 3×24 hours to authorities (Article 46)
- **Criminal penalties:** Up to 5 years imprisonment + IDR 5B fine (Article 67)
- **Scope:** Indonesian orgs + foreign orgs processing Indonesian patient data

Patterns are mapped to UU PDP articles in `docs/regulatory-mapping.md` (forthcoming). Do NOT claim compliance for features that lack mapping. Reference `POSITIONING.md` §7 for claim discipline.

---

## Anti-Goals (Explicit Non-Scope)

These are intentional. PRs moving toward them will be declined with reference to this file:

- **Not an HTTP API server** — CLI-as-subprocess is the integration pattern
- **Not a daemon** — Stateless, single-run execution
- **Not a workflow engine** — Does not orchestrate DPIA, breach notification, audit workflows
- **Not a tokenization vault** — No pseudonymization or synthetic data generation
- **Not an EHR/SIMRS adapter** — No FHIR, HL7, DICOM in v1.0
- **Not a compliance certification** — Tool assists; org is responsible for compliance

---

## Getting Help

1. **For understanding the codebase:** Read `ARCHITECTURE.md` §2 (the Rust core) and walk through `src/lib.rs` → `src/scanner/mod.rs` → `src/domain/mod.rs`
2. **For task context:** Look up the task ID in `TODO.md` (e.g., T-0200) for description, rationale, and definition of done
3. **For regulatory questions:** Check `POSITIONING.md` and `docs/regulatory-mapping.md` (forthcoming)
4. **For design tenets:** Refer to `ARCHITECTURE.md` §0.2

---

**Last updated:** 2026-05-23 | **Aligned with:** TODO.md v0.1.0-draft.3, ARCHITECTURE.md v0.1.0-draft.2, M2 PHASE 1 complete
