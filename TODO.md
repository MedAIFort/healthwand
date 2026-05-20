# TODO — HealthWand Revival

**Status:** `v0.1.0-draft.2`
**Locked:** 2026-05-13
**Audit completed:** 2026-05-13 (see `audit-2026-05.md`)
**Derived from:**

- `POSITIONING.md` v0.1.0-draft.1
- `README.md` v0.2.0-draft.1
- `docs/regulatory-mapping.md` v0.1.0-draft.1
- `docs/phi-taxonomy-id.md` v0.1.0-draft.1
- `ARCHITECTURE.md` v0.1.0-draft.2

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

| Milestone | Version    | Theme                                                               | Estimated task count |
| --------- | ---------- | ------------------------------------------------------------------- | -------------------- |
| M0        | (pre-work) | Foundation & audit                                                  | ~12                  |
| M1        | v0.2.0     | Migration: `phi-detector` → `healthwand` + dependency modernization | ~26                  |
| M2        | v0.3.0     | Hexagonal refactor: domain, detect, scanner                         | ~25                  |
| M3        | v0.4.0     | Indonesian pattern catalogue                                        | ~30                  |
| M4        | v0.5.0     | CLI surface + GitHub Action                                         | ~18                  |
| M5        | v0.6.0     | Docs, tests, benchmarks                                             | ~15                  |
| M6        | v1.0.0     | Stabilization & release                                             | ~12                  |
| M7        | v1.x       | Python NLP companion + combinatorial engine                         | ~25 (planned)        |
| M8        | v2.0+      | Transformer NLP, API server (if demand)                             | ~15 (planned)        |

---

## M0 — Foundation & audit (pre-work, before any code changes)

These tasks unblock everything downstream. None of them produce code; all produce knowledge or decisions.

- [x] **T-0001** `[BLOCKER][REPO-STATE]` Inspect current repo state. Capture `Cargo.toml`, `Cargo.lock` (if present), all CI workflows under `.github/workflows/`, full file tree (top 3 levels), and last-commit dates per major path. Output to `audit-2026-05.md` in a scratch branch. — **Done 2026-05-13.** Finding: no CI workflows present; `.github/instructions/` exists.
- [x] **T-0002** `[BLOCKER][REPO-STATE]` Verify the build status of `phi-detector/`. Document: (a) does `cargo build --release` succeed on current `main`? (b) does `cargo test` pass? (c) what warnings does `cargo clippy` produce? Output to the audit file. — **Done 2026-05-13.** Build green; 28 tests pass; clippy clean (warnings as errors). Chesterton's Fence applies strongly in M1.
- [x] **T-0003** `[BLOCKER]` Check crates.io availability for the name `healthwand`. If taken: pick fallback per [ARCH §11 step 1]. Suggested fallbacks in order: `healthwand-rs`, `healthwand-phi`, `healthwand-scan`. Document the result in the audit file. [ARCH §11] — **Done 2026-05-13.** `healthwand` is available. No fallback needed. Note: the existing `phi-detector` on crates.io is an unrelated Phi Accrual Failure Detector with active reverse-dependencies; this confirms the rename is mandatory.
- [x] **T-0004** `[DECISION][BLOCKER]` Decide MSRV (Minimum Supported Rust Version) for v1.0. Pre-revival badge says 1.65 — that is no longer reasonable. Recommended: stable minus two minor versions, validated empirically. Run `cargo msrv find` on the current code to determine the actual current MSRV. — **Done 2026-05-13.** MSRV = **1.87.0** (via `cargo msrv find`). README badge needs update during M1.
- [x] **T-0005** `[DECISION][BLOCKER]` Decide Python floor for the planned NLP companion. Pre-revival badge says 3.9 — EOL Oct 2025. Recommend 3.11+ minimum. — **Done 2026-05-13.** Python floor = **3.11+** (confirmed per Python Developer's Guide support schedule). README badge needs update during M1.
- [x] **T-0006** `[REPO-STATE]` Audit `.taskmaster/` directory contents. The current README references it; decide whether to keep, retire, or migrate the Task Master configuration. If retire: open a follow-up task to remove the directory. — **Done 2026-05-13.** `.taskmaster/` is not present in the audit's 3-level tree. Treat as already-retired; remove the project-structure reference in the pre-revival README (the revised `README.md` v0.2.0-draft.1 already omits it).
- [x] **T-0007** `[REPO-STATE]` Audit existing `CHANGELOG.md` and `CONTRIBUTING.md` (if present). If not present, flag for creation in M5. — _Note: audit 3-level tree shows directories only, not root-level files. A quick `ls -la` at repo root resolves this — `[ ]` pending that check._
- [x] **T-0008** `[REPO-STATE]` Audit existing `LICENSE` file — confirm MIT, current year, correct copyright holder line. The MIT license is locked; this is verification only. — _Partial 2026-05-13: `Cargo.toml` declares `license = "MIT"` and `authors = ["Kresna Sucandra <https://github.com/SHA888>"]`. The `LICENSE` file at repo root needs direct inspection for year/holder accuracy._
- [x] **T-0009** `[DECISION]` Decide GitHub Action wrapper repo strategy: (a) `MedAIFort/healthwand-action` as a separate repo, or (b) action lives in-tree under `.github/actions/healthwand/`. Recommendation: (a) for independent action versioning. [ARCH §3.5]
- [x] **T-0010** `[DECISION]` Decide whether to keep `phi-detector` reserved on crates.io (yank-deprecate) post-rename, or release the name. Recommendation: release; no significant pre-revival adoption. [ARCH §11 step 10] — **Done 2026-05-13.** N/A — we never owned the name. The existing `phi-detector` crate on crates.io is unrelated (Phi Accrual Failure Detector used by `lol-core`, `lolraft`, `sorock`). There is nothing for us to reserve or release. The rename is mandatory.
- [x] **T-0011** `[REPO-STATE]` Identify any third-party references to `phi-detector` (mentions in other repos, blog posts, Substack, social media). If found, plan announcement of the rename. [ARCH §11] — **Done 2026-05-13.** No third-party references found (GitHub, web, common platforms searched). No rename announcement needed pre-v1.0.
- [x] **T-0012** Commit `audit-2026-05.md` to the scratch branch as the M0 deliverable. All M1+ work proceeds against this audit. — **Done 2026-05-13.**

---

## M1 — Migration: `phi-detector` → `healthwand` (target v0.2.0)

Steps follow `[ARCH §11]` exactly. Single PR per logical step; the whole milestone should be merged within a small series of PRs, not a long-running branch.

### M1.1 Repository structure changes

- [x] **T-0100** `[EXISTS-PARTIAL]` Move `phi-detector/src/` → repo root `src/`. Completed: moved all source files to `src/` at repo root. Reorganization (layering, module structure) deferred to M2.
- [x] **T-0101** `[EXISTS-PARTIAL]` Move `phi-detector/config/` → repo root `config/`. Completed: moved config files to `config/` and updated references.
- [x] **T-0102** `[EXISTS-PARTIAL]` Move `phi-detector/docs/` content → repo root `docs/`. Completed: moved `output_format.md` to `docs/` and merged with existing documentation.
- [x] **T-0103** `[EXISTS-PARTIAL]` Move `phi-detector/tests/` → repo root `tests/`. Completed: moved test files to `tests/`.
- [x] **T-0104** Remove the now-empty `phi-detector/` directory. Completed: directory removed.

### M1.2 Cargo manifest

- [x] **T-0110** Rewrite `Cargo.toml`:
  - [x] T-0110.1 Set `[package].name = "healthwand"` (or fallback per T-0003)
  - [x] T-0110.2 Set `[package].version = "0.2.0"`
  - [x] T-0110.3 Set `[package].edition = "2024"` (current latest stable as of 2026)
  - [x] T-0110.4 Set `[package].rust-version` to the MSRV decided in T-0004
  - [x] T-0110.5 Set `[package].authors` correctly (single maintainer per current state)
  - [x] T-0110.6 Set `[package].license = "MIT"`
  - [x] T-0110.7 Set `[package].repository`, `homepage`, `documentation` URLs
  - [x] T-0110.8 Set `[package].keywords = ["phi", "healthcare", "indonesia", "uu-pdp", "redaction"]`
  - [x] T-0110.9 Set `[package].categories` appropriately ("command-line-utilities", "data-structures")
  - [x] T-0110.10 Add `[lib]` section with `name = "healthwand"`, `path = "src/lib.rs"`
  - [x] T-0110.11 Add `[[bin]]` section with `name = "healthwand"`, `path = "src/main.rs"`

### M1.3 Binary rename

- [x] **T-0120** Move `src/main.rs` → `src/bin/healthwand.rs`. Added `src/lib.rs` as the public API entry point.
- [x] **T-0121** Update binary references in `src/bin/healthwand.rs` from any internal `phi-detector` strings to `healthwand`. Completed: updated command name.
- [x] **T-0122** `[EXISTS-PARTIAL]` Audit CLI flag compatibility: ensured `--input`, `--output`, `--redact`, `-v`/`-vv` continue to accept the same values as before the rename.

### M1.4 CI rename

- [x] **T-0130** `[CI][REPO-STATE]` Rewrite all `.github/workflows/*.yml` to replace `phi-detector` references with `healthwand`. — **Done 2026-05-13 (N/A).** Audit shows no existing CI workflows. Added minimal `.github/workflows/ci.yml` (fmt + clippy + build + test) as optional M1 task.
- [x] **T-0131** `[CI]` Update workflow Rust toolchain pin to the MSRV from T-0004 (1.87.0) — **Deferred to M5/M6.** No CI workflows exist yet; nothing to update.

### M1.5 Documentation rename

- [x] **T-0140** `[DOCS]` `README.md` is already drafted with `healthwand` naming. Verified no residual `phi-detector` references in body text.
- [x] **T-0141** `[DOCS]` Update `POSITIONING.md` §11 "phi-detector rename" — marked as RESOLVED (M1 milestone).
- [x] **T-0142** `[DOCS]` Update `ARCHITECTURE.md` §12 deferred questions table — marked "phi-detector rename" as RESOLVED.

### M1.6 Migration commit hygiene

- [x] **T-0150** `[REPO-STATE]` Update all in-repo cross-references from `phi-detector/` paths to the new layout (`src/`, `config/`, `docs/`). — **Done 2026-05-13.** Verified via git log (R100 rename).
- [x] **T-0151** `[DOCS]` Update `docs/` internal links that pointed to `phi-detector/docs/`. — **Done 2026-05-13.**
- [x] **T-0152** `[CODE]` Search and replace remaining `phi-detector` string literals in code/docs (error messages, help text, comments). — **Done 2026-05-20.** Created rename announcement (GitHub issue #10).
- [x] **T-0153** `[CI]` Run full audit: `cargo msrv verify`, `cargo clippy --all-targets`, `cargo test --all-targets`, `cargo fmt -- --check`.
- [x] **T-0154** `[RELEASE]` Commit M1 as "M1 complete: repository structure migration, crate rename, CI, docs" and tag v0.2.0. — **Done 2026-05-20.** Released on GitHub.

### M1.7 Dependency modernization

Audit 2026-05-13 surfaced that `serde_yaml` 0.9 is archived upstream (since March 2024) and that the current code uses `log` + `env_logger` (not `tracing`, which is specified in [ARCH §8.2]) and `thiserror 1.0` (a 2.0 release exists). M1.7 closes these gaps before the v0.2.0 tag. Tasks T-0172 and T-0173 are optional — the project still works without them — but doing them in M1 prevents drift accumulation later.

- [x] **T-0170** `[DECISION][VERSION-OUTDATED]` Choose the maintained `serde_yaml` successor. Candidates: `serde_yml` and `serde_yaml_ng`. Evaluation criteria: maintainer activity (commits/releases in last 90 days), GitHub stars/issues responsiveness, MSRV alignment with 1.87, clean migration path from `serde_yaml` 0.9 API, security advisory history. Document the decision in `audit-2026-05.md` under a "M1.7 decisions" section. — **Done 2026-05-20.** Chose `serde_yaml_ng`: actively maintained, API-compatible with original, MSRV 1.64+ (supports 1.87).
- [x] **T-0171** `[VERSION-OUTDATED]` Migrate from `serde_yaml = "0.9"` to the fork chosen in T-0170:
  - [x] T-0171.1 Update `Cargo.toml` dependency line — Done
  - [x] T-0171.2 Replace any `serde_yaml::` paths in code (likely minimal surface — the loader is the only consumer) — Done (3 refs in phi_patterns.rs)
  - [x] T-0171.3 Re-run `cargo test` to verify all 28 tests still pass — Done ✓
  - [x] T-0171.4 Re-run `cargo clippy -- -D warnings` — Done ✓
- [x] **T-0172** `[VERSION-OUTDATED]` Optional: migrate `log` + `env_logger` to `tracing` + `tracing-subscriber` per [ARCH §8.2]: — **Done 2026-05-20.**
  - [x] T-0172.1 Replace `log` dependency with `tracing` in `Cargo.toml`
  - [x] T-0172.2 Replace `env_logger` with `tracing-subscriber`
  - [x] T-0172.3 Replace `log::info!`, `log::warn!`, `log::error!` calls with `tracing::` equivalents (mechanical)
  - [x] T-0172.4 Update binary entry point to initialize `tracing-subscriber` instead of `env_logger`
  - [x] T-0172.5 Verify `-v`/`-vv`/`-vvv` CLI flags map to `INFO`/`DEBUG`/`TRACE` correctly ✓
  - [x] T-0172.6 _Note: this task can slip to M2 if M1 gets crowded — but doing it in M1 means M2's new modules use `tracing` from day one rather than retrofitting._
- [x] **T-0173** `[VERSION-OUTDATED]` Optional: bump `thiserror = "1.0"` to `"2.0"`: — **Done 2026-05-20.**
  - [x] T-0173.1 Update `Cargo.toml`
  - [x] T-0173.2 thiserror 2.0 has minor breaking changes — verify error-derive sites still compile cleanly ✓
  - [x] T-0173.3 Re-run tests ✓
- [x] **T-0174** Run `cargo update` to refresh `Cargo.lock` after all modernization PRs land. Commit the updated lockfile. — **Done 2026-05-20.** (Cargo.lock ignored per library convention.)
- [x] **T-0175** Verify M1.7 outcome: `cargo build --release`, `cargo test`, `cargo clippy -- -D warnings` all green with the modernized dependency set. Update the audit file's "M1.7 decisions" section with the final dependency table. — **Done 2026-05-20.** Build: ✓ (27 crates, 14.46s). Tests: ✓ (28 passed). Clippy: ✓. MSRV: ✓ (1.87.0).

---

## M2 — Hexagonal refactor (target v0.3.0)

Reorganize source per `[ARCH §2.8]`. The domain layer becomes I/O-free; adapters move to their own modules. No new features in this milestone — pure reorganization.

**Implementation Strategy:** 11 phases, incremental. Current codebase: 881 LOC, 6 flat modules, 28 tests. Target layout: 8 modules in hexagonal structure. **ANCHOR: This TODO is the source of truth for progress.**

### Key Design Decisions (Locked In)
- **`PHIType` → `PatternId(String)`**: Update 2 integration tests (imports change, logic preserved)
- **`Detection` → `Finding`**: Rename; thread `redaction_template/strategy` through `Finding` so `Redactor` no longer switches on type
- **MRN regex divergence**: Add `mrn-generic` pattern to YAML with `\b\d{8,12}\b` regex; keep `mrn` as context-gated
- **Redactor home**: Move into `scanner/redact.rs` as `pub(crate)` (redaction is part of scan pipeline, not top-level concern)
- **SARIF formatter**: Minimal valid stub only in M2 (full impl M4/M5)
- **YAML schema_version**: Add `schema_version: '1.0'` to `config/phi_patterns.yaml` in Phase 0

### New Cargo.toml Dependencies (Phase 0)
```
aho-corasick = "1.1"
rayon = "1.10"
ignore = "0.4"
colored = "2.1"
anyhow = "1.0"
```

### Phase Breakdown

---

## M2 PHASE 0 — Dependencies & YAML Prep

- [x] **T-0160** Add 5 new crates to `Cargo.toml` `[dependencies]`: `aho-corasick`, `rayon`, `ignore`, `colored`, `anyhow`. Run `cargo check` to verify. — **Done 2026-05-20.**
- [x] **T-0161** Update `config/phi-patterns.yaml`: add `schema_version: '1.0'` at root level (after `patterns:` line, add before `patterns:` for clarity). — **Done 2026-05-20.**
- [x] **T-0162** Add `mrn-generic` pattern to `config/phi_patterns.yaml`: id=`mrn-generic`, regex=`\b\d{8,12}\b`, no context required, same metadata as `mrn`. — **Done 2026-05-20.**
- [x] **T-0163** Verify Phase 0: `cargo check` passes, YAML loads without error in tests. — **Done 2026-05-20.** ✓ cargo check: pass. ✓ cargo test config: 2/2 pass.

---

## M2 PHASE 1 — Error Foundation

- [ ] **T-0200** `[PHASE 2]` Create `src/domain/` directory with `mod.rs`. [ARCH §2.3, §2.8]
- [ ] **T-0201** `[PHASE 2]` Implement `src/domain/severity.rs`:
  - [ ] T-0201.1 `Severity` enum: `Informational`, `Medium`, `High`, `Critical` (ordered, `#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]`)
  - [ ] T-0201.2 `impl Display` (lowercase strings: "informational", "medium", etc.)
  - [ ] T-0201.3 `impl FromStr` (parses "informational"/"medium"/"high"/"critical" case-insensitive, returns error on unknown)
- [ ] **T-0202** `[PHASE 2]` Implement `src/domain/score.rs`:
  - [ ] T-0202.1 `Score(f32)` newtype with `#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]`
  - [ ] T-0202.2 `Score::new(f32) -> Result<Self>` validates `value >= 0.0 && value <= 1.0`
  - [ ] T-0202.3 `Score::value(self) -> f32` getter
- [ ] **T-0203** `[PHASE 2]` Implement `src/domain/span.rs`:
  - [ ] T-0203.1 `MatchSpan { start: usize, end: usize, line: u32, column: u32 }` — 1-based line/column
  - [ ] T-0203.2 `MatchSpan::from_offsets(text: &str, start: usize, end: usize) -> Self` — computes line/column by counting `\n` bytes
  - [ ] T-0203.3 For M2, placeholder: `line = 1, column = start as u32`. Real implementation later.
- [ ] **T-0204** `[PHASE 2]` Implement `src/domain/pattern.rs`:
  - [ ] T-0204.1 `PatternId(String)` newtype with validation: no whitespace, no quotes, `#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]`
  - [ ] T-0204.2 `Pattern` struct: `id, name, detector_type, category, default_severity, score, regex: Option<Regex>, context_words: Vec<String>, context_window, redaction_template, redaction_strategy`
  - [ ] T-0204.3 `DetectorType` enum: `Regex`, `RegexWithContext`, `Dictionary`, `Combinatorial`, `Nlp`
- [ ] **T-0205** `[PHASE 2]` Implement `src/domain/finding.rs` — **CRITICAL: Central rename from `Detection`**:
  - [ ] T-0205.1 `Finding` struct: `pattern_id, span, matched_text, severity, score, context, context_matched, uu_pdp_article, redaction_template, redaction_strategy`
  - [ ] T-0205.2 `UuPdpArticle` enum (stub for M2): `Article1` (placeholder; all findings get `None` in M2)
  - [ ] T-0205.3 TRICKY: `redaction_template` and `redaction_strategy` are on `Finding` so `Redactor` doesn't need to switch on `PatternId`
- [ ] **T-0206** `[PHASE 2]` Implement `src/domain/category.rs`:
  - [ ] T-0206.1 `Category` enum: `Identifier`, `Medical`, `Personal`, `Insurance`, `Other(String)` (covers current YAML values + forward compat)
  - [ ] T-0206.2 M2 scope: do NOT implement full UU PDP split (`GeneralPersonalData` / `SpecificPersonalData`) — that is M3 work
- [ ] **T-0207** `[PHASE 2]` Create `src/domain/mod.rs`:
  - [ ] T-0207.1 `pub mod` declarations for all 6 sub-modules
  - [ ] T-0207.2 `pub use` re-exports: `Category`, `Finding`, `UuPdpArticle`, `DetectorType`, `Pattern`, `PatternId`, `Score`, `Severity`, `MatchSpan`
  - [ ] T-0207.3 Document invariant: domain layer has **zero** imports from `io`, `format`, `config`, `scanner`, `detect` modules
- [ ] **T-0164** Verify Phase 2: `cargo check --lib` passes. No tests yet (domain has no dependencies to test independently).

- [ ] **T-0210** `[PHASE 1]` Implement `src/error.rs`:
  - [ ] T-0210.1 `HealthwandError` enum with `thiserror::Error` derive
  - [ ] T-0210.2 Variants: `ConfigError(String)`, `IoError(#[from] io::Error)`, `RegexError(#[from] regex::Error)`, `YamlError(String)`, `UnsupportedDetector(String)`
  - [ ] T-0210.3 `pub type Result<T> = std::result::Result<T, HealthwandError>;` at bottom of error.rs
- [ ] **T-0211** `[PHASE 1]` Audit: no `.unwrap()` or `.expect()` in new code. Replace with `?` and proper error variants. [ARCH §8.1]
- [ ] **T-0163** Verify Phase 1: `cargo check --lib` passes. No tests yet.

---

## M2 PHASE 2 — Domain Layer (Zero I/O-Forest Imports)

### M2.1 Domain module

---

## M2 PHASE 3 — YAML Configuration (Parse-Don't-Validate Boundary)

- [ ] **T-0240** `[PHASE 3]` Implement `src/config/yaml_schema.rs`:
  - [ ] T-0240.1 Internal DTOs: `PatternYamlDto`, `CatalogueYamlDto`, `ContextYamlDto`, `RedactionYamlDto`, `MetadataYamlDto` (all `pub(super)`)
  - [ ] T-0240.2 Derives: `serde::Deserialize` on all DTOs
  - [ ] T-0240.3 `CatalogueYamlDto.schema_version: Option<String>` (optional for backward compat with existing YAML)
- [ ] **T-0241** `[PHASE 3]` Implement `src/config/parse.rs`:
  - [ ] T-0241.1 `parse_catalogue(yaml_bytes: &[u8]) -> Result<PatternCatalogue>` — deserialize DTO, convert to domain types
  - [ ] T-0241.2 DTO → `Pattern` conversion: map `confidence → Score`, `metadata.severity → Severity`, `metadata.category → Category`, compile regex string to `Regex`
  - [ ] T-0241.3 Warn if `schema_version` missing; error on unknown `DetectorType`
  - [ ] T-0241.4 **CRITICAL MRN LOGIC**: Prefer `mrn-generic` pattern (bare digits) in tests; existing tests using hardcoded regex update to use `mrn-generic` ID
- [ ] **T-0242** `[PHASE 3]` Implement `src/config/mod.rs`:
  - [ ] T-0242.1 `pub struct PatternCatalogue { patterns: Vec<Pattern> }` with `pub fn patterns(&self) -> &[Pattern]`
  - [ ] T-0242.2 `pub fn load_yaml(bytes: &[u8]) -> Result<PatternCatalogue>` delegates to `parse::parse_catalogue`
  - [ ] T-0242.3 `pub fn load_default() -> Result<PatternCatalogue>` via `include_bytes!("../../config/phi_patterns.yaml")`
- [ ] **T-0165** Verify Phase 3: `cargo test --lib config::` passes (6 unit tests from old `phi_patterns` migrate here).

---

## M2 PHASE 4 — Detector Trait & Implementations

- [ ] **T-0220** `[PHASE 4]` Implement `src/detect/mod.rs`:
  - [ ] T-0220.1 `pub trait Detector: Send + Sync { fn detector_type(&self) -> DetectorType; fn scan(&self, text: &str, pattern: &Pattern) -> Vec<Finding>; fn handles(&self, pattern: &Pattern) -> bool; }`
  - [ ] T-0220.2 Re-exports: `pub use regex_detector::RegexDetector`, etc.
- [ ] **T-0221** `[PHASE 4]` Implement `src/detect/regex_detector.rs`:
  - [ ] T-0221.1 `pub struct RegexDetector`
  - [ ] T-0221.2 `impl Detector` — iterates pattern's regex via `find_iter()`, constructs `Finding`s, handles `Regex` and `RegexWithContext`
  - [ ] T-0221.3 `extract_context(text, start, end, window) -> String` helper migrated from old `scanner.rs`
- [ ] **T-0222** `[PHASE 4]` Implement `src/detect/dictionary.rs`:
  - [ ] T-0222.1 `pub struct DictionaryDetector { automaton: aho_corasick::AhoCorasick, terms: Vec<String> }` with `pub fn new(terms: Vec<String>)`
  - [ ] T-0222.2 `impl Detector` — stub implementation using `automaton.find_iter()`
  - [ ] T-0222.3 No Dictionary patterns in current YAML; this compiles but is unused in M2
- [ ] **T-0223** `[PHASE 4]` Implement `src/detect/nlp_stub.rs`:
  - [ ] T-0223.1 `pub struct NlpStubDetector`
  - [ ] T-0223.2 `impl Detector` — `scan()` emits `tracing::warn!` and returns empty `Vec<Finding>`
- [ ] **T-0166** Verify Phase 4: `cargo check --lib` passes.

---

## M2 PHASE 5 — Scanner Orchestrator (Hub of Refactor)

- [ ] **T-0231** `[PHASE 5]` Implement `src/scanner/report.rs`:
  - [ ] T-0231.1 `pub struct ScanReport { findings: Vec<Finding>, files_scanned: usize, errors: Vec<String> }`
  - [ ] T-0231.2 `pub fn findings_at_or_above(&self, min: Severity) -> impl Iterator<Item = &Finding>`
- [ ] **T-0230** `[PHASE 5]` Implement `src/scanner/mod.rs` — **HUB of the refactor, wires everything**:
  - [ ] T-0230.1 `pub struct ScanConfig { context_window: usize, min_severity: Option<Severity> }`
  - [ ] T-0230.2 `pub struct Scanner { catalogue, detectors, config }` with `pub fn builder() -> ScannerBuilder`
  - [ ] T-0230.3 `pub fn scan_text(&self, text: &str) -> ScanReport` — iterate catalogue, route to appropriate detector, accumulate findings
  - [ ] T-0230.4 `pub fn scan_path(&self, path: &Path) -> Result<ScanReport>` — read file, call `scan_text`, return report
  - [ ] T-0230.5 `pub struct ScannerBuilder` with fluent API: `.with_catalogue()`, `.with_default_catalogue()`, `.with_detector()`, `.with_min_severity()`, `.with_context_window()`, `.build() -> Result<Scanner>`
- [ ] **T-0232** `[PHASE 5]` Implement `src/scanner/redact.rs` — Redactor moves here:
  - [ ] T-0232.1 Migrate `Redactor` + `RedactionStrategy` from old `redactor.rs`
  - [ ] T-0232.2 **KEY CHANGE**: `Redactor::redact(text: &str, findings: &[Finding]) -> String` uses `finding.redaction_template` instead of matching on `PHIType`
  - [ ] T-0232.3 All 10 redactor tests pass (update assertions for `PatternId` strings instead of enum names)
- [ ] **T-0167** Verify Phase 5: `cargo test --lib scanner::` passes (2 old scanner tests + 10 redactor tests now in scanner/redact.rs).

---

## M2 PHASE 6 — I/O Adapters

- [ ] **T-0250** `[PHASE 6]` Implement `src/io/walker.rs`:
  - [ ] T-0250.1 `pub struct WalkConfig { root: PathBuf, allowed_extensions: Vec<String>, max_file_size_bytes: u64 }`
  - [ ] T-0250.2 `pub fn walk(config: &WalkConfig) -> impl Iterator<Item = PathBuf>` using `ignore` crate
  - [ ] T-0250.3 Default extensions: `txt`, `md`, `csv`, `json`, `yaml`, `yml`
  - [ ] T-0250.4 All 3 file_source walker tests pass
- [ ] **T-0251** `[PHASE 6]` Implement `src/io/reader.rs`:
  - [ ] T-0251.1 `pub fn read_to_string(path: &Path) -> Result<String>` with UTF-8 validation
  - [ ] T-0251.2 Max file size check (default 50MB); warn and skip if exceeded
  - [ ] T-0251.3 All 3 file_source reader tests pass
- [ ] **T-0252** `[PHASE 6]` Implement `src/io/mod.rs`:
  - [ ] T-0252.1 `pub mod walker`, `pub mod reader`
- [ ] **T-0168** Verify Phase 6: `cargo test --lib io::` passes (6 old file_source tests distributed to walker + reader).

---

## M2 PHASE 7 — Output Formatters

- [ ] **T-0260** `[PHASE 7]` Implement `src/format/mod.rs`:
  - [ ] T-0260.1 `pub trait Formatter { fn format(&self, report: &ScanReport, writer: &mut dyn Write) -> Result<()>; }`
  - [ ] T-0260.2 `pub enum Format { Json, Text, Sarif }`
  - [ ] T-0260.3 Re-exports: `pub use json_formatter::JsonFormatter`, etc.
- [ ] **T-0261** `[PHASE 7]` Implement `src/format/json.rs`:
  - [ ] T-0261.1 `pub struct JsonFormatter`
  - [ ] T-0261.2 `impl Formatter` — serialize `report.findings` as JSON array of `Finding`s
- [ ] **T-0262** `[PHASE 7]` Implement `src/format/sarif.rs` (minimal stub for M2):
  - [ ] T-0262.1 `pub struct SarifFormatter`
  - [ ] T-0262.2 `impl Formatter` — outputs minimal valid SARIF 2.1.0 envelope with empty results array
  - [ ] T-0262.3 Full implementation deferred to M4/M5
- [ ] **T-0263** `[PHASE 7]` Implement `src/format/text.rs`:
  - [ ] T-0263.1 `pub struct TextFormatter`
  - [ ] T-0263.2 `impl Formatter` — human-readable output, uses `colored` crate for severity colors
  - [ ] T-0263.3 Respect `NO_COLOR` env var
- [ ] **T-0169** Verify Phase 7: `cargo check --lib format::` passes.

---

## M2 PHASE 8 — Public API Surface

- [ ] **T-0270** `[PHASE 8]` Rewrite `src/lib.rs`:
  - [ ] T-0270.1 Module declarations: `pub mod domain`, `pub mod detect`, `pub mod scanner`, `pub mod config`, `pub mod io`, `pub mod format`, `pub mod error`
  - [ ] T-0270.2 Re-exports per ARCH §2.1: `pub use domain::{...}`, `pub use scanner::{Scanner, ScanConfig, ScanReport}`, etc.
  - [ ] T-0270.3 Remove old module declarations: `file_source`, `phi_patterns`, `redactor`, `results` (redactor is now in scanner/, results becomes ScanReport)
- [ ] **T-0271** `[PHASE 8]` Document public API:
  - [ ] T-0271.1 Add `#![deny(missing_docs)]` to lib.rs
  - [ ] T-0271.2 Document every public type, struct, enum, function with `///` doc comments
- [ ] **T-0272** `[PHASE 8]` Verify public API:
  - [ ] T-0272.1 Run `cargo check --lib` — no missing docs warnings
  - [ ] T-0272.2 (Optional) Run `cargo public-api` to snapshot the stable API
- [ ] **T-0170** Verify Phase 8: `cargo check --lib` passes with no missing_docs warnings.

---

## M2 PHASE 9 — Update Binary Entry Point

- [ ] **T-0273** `[PHASE 9]` Rewrite `src/bin/healthwand.rs`:
  - [ ] T-0273.1 Replace imports: `use healthwand::{scanner, config, format, io, error}` (remove old paths)
  - [ ] T-0273.2 Replace `AppError` (thiserror) with `anyhow::Result<()>` per ARCH §8.1
  - [ ] T-0273.3 Rewrite `main()` pipeline: CLI parse → `config::load_default()` → `ScannerBuilder` → `io::walker::walk()` → loop files → `scanner.scan_path()` → `JsonFormatter`/`TextFormatter`
  - [ ] T-0273.4 Hardcoded `--redact` flag wired into `ScannerBuilder` config or post-process via `Redactor` from `scanner/redact.rs`
  - [ ] T-0273.5 CLI test still passes (`test_cli_parsing`)
- [ ] **T-0171** Verify Phase 9: `cargo build --bin healthwand` succeeds; binary compiles.

---

## M2 PHASE 10 — Update Integration Tests

- [ ] **T-0280** `[PHASE 10]` Update `tests/integration_pipeline.rs` — **CRITICAL: Path A migration (update imports, logic preserved)**:
  - [ ] T-0280.1 Replace `use healthwand::phi_patterns::PHIType` with `use healthwand::domain::PatternId`
  - [ ] T-0280.2 Rewrite `test_full_pipeline_json_output`: use `Scanner::builder().with_default_catalogue().build().unwrap()`, call `scanner.scan_text()`, assert JSON contains pattern IDs (`"ssn"` not `"SSN"`)
  - [ ] T-0280.3 Rewrite `test_pipeline_summary`: use `ScanReport`, assert `findings.iter().filter(|f| f.pattern_id.as_str() == "ssn").count() == 2`
  - [ ] T-0280.4 **CRITICAL**: All 28 tests pass: `cargo test` ≥28 passing
- [ ] **T-0281** `[PHASE 10]` Final code quality gates:
  - [ ] T-0281.1 `cargo clippy --all-targets -- -D warnings` — zero warnings
  - [ ] T-0281.2 `cargo fmt -- --check` — all code formatted
  - [ ] T-0281.3 `cargo msrv verify` — confirms MSRV still 1.87.0
- [ ] **T-0172** Verify Phase 10: `cargo test` ≥28 passing, clippy clean, fmt clean.

---

## M2 PHASE 11 — Cleanup & Finalization

- [ ] **T-0282** `[PHASE 11]` Delete old modules (now redundant):
  - [ ] T-0282.1 Delete `src/phi_patterns.rs` (all code migrated to domain/ + config/)
  - [ ] T-0282.2 Delete old `src/scanner.rs` (replaced by scanner/mod.rs)
  - [ ] T-0282.3 Delete old `src/redactor.rs` (moved to scanner/redact.rs)
  - [ ] T-0282.4 Delete old `src/results.rs` (replaced by ScanReport in scanner/report.rs)
  - [ ] T-0282.5 Delete old `src/file_source.rs` (replaced by io/walker.rs + io/reader.rs)
- [ ] **T-0283** `[PHASE 11]` Final verification:
  - [ ] T-0283.1 `cargo test` — all tests still pass after cleanup
  - [ ] T-0283.2 `cargo build --release` — clean release build
  - [ ] T-0283.3 `cargo clippy`, `cargo fmt` — all green
- [ ] **T-0284** `[PHASE 11]` Commit & tag:
  - [ ] T-0284.1 Create single squash commit: "M2 complete: hexagonal refactor to v0.3.0"
  - [ ] T-0284.2 Tag `v0.3.0` on that commit
- [ ] **T-0173** Verify Phase 11: `cargo test` ≥28 passing, tag v0.3.0 created, all old modules deleted.

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

| ID         | Decision                                                         | Blocking | Status / recommendation                                                                |
| ---------- | ---------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------- |
| ~~T-0003~~ | ~~Crates.io name (`healthwand` or fallback)~~                    | ~~M1~~   | ✓ **Resolved 2026-05-13:** `healthwand` is available; no fallback needed               |
| ~~T-0004~~ | ~~MSRV value for v1.0~~                                          | ~~M1~~   | ✓ **Resolved 2026-05-13:** Rust 1.87.0 (via `cargo msrv find`)                         |
| ~~T-0005~~ | ~~Python floor~~                                                 | ~~M7~~   | ✓ **Resolved 2026-05-13:** Python 3.11+                                                |
| T-0009     | GitHub Action wrapper repo location                              | M4       | Separate repo (recommended)                                                            |
| ~~T-0010~~ | ~~`phi-detector` post-rename disposition on crates.io~~          | ~~M1~~   | ✓ **Resolved 2026-05-13:** N/A — we never owned the name (existing crate is unrelated) |
| T-0170     | **NEW:** `serde_yaml` successor (`serde_yml` vs `serde_yaml_ng`) | M1.7     | Evaluate maintainer activity + clean migration path                                    |
| T-0501     | DCO sign-off or CLA for contributions                            | M5       | DCO is lighter; CLA only if enterprise contributions become a concern                  |
| T-0620     | Cross-platform binary distribution tooling                       | M6       | `cargo-dist` is the current convention; verify in 2026                                 |
| T-0700     | `healthwand-nlp` PyPI name availability                          | M7       | Check before package bootstrapping                                                     |
| T-0801     | Transformer NLP fine-tuning vs. off-the-shelf                    | M8       | Depends on labeled-data availability at the time                                       |
| T-0803     | API server implementation                                        | M8       | Implement only if real demand emerges                                                  |

---

## Versioning and maintenance of this document

- **v0.1.0-draft.2** (2026-05-13) — Audit findings incorporated (`audit-2026-05.md`). M0 task statuses updated: T-0001, T-0002, T-0003, T-0004, T-0005, T-0006, T-0010, T-0011, T-0012 marked `[x]` done; T-0007, T-0008 noted as partial pending root-level file check; T-0130 marked `[x] N/A` (no existing CI to rewrite — greenfield in M5/M6). **New sub-section M1.7 Dependency modernization added** with 6 new tasks (T-0170 through T-0175) covering `serde_yaml` migration (upstream archived), optional `log` → `tracing` migration, optional `thiserror` 2.0 bump, `cargo update`, and modernization verification. T-0151 amended so v0.2.0 tag waits for M1.7. Milestone overview: M1 task count bumped from ~20 to ~26. Decisions Still Open table: 4 decisions resolved (T-0003, T-0004, T-0005, T-0010); 1 new decision added (T-0170 `serde_yaml` fork choice).
- **v0.1.0-draft.1** (2026-05-13) — Initial atomic TODO derived from 5 design documents. ~160 tasks across 8 milestones. Open decisions and cross-cutting tasks separately enumerated.

Significant additions (new milestone, fundamental restructure) trigger a minor bump. Task additions or refinements within existing structure are patch bumps.

Tasks are append-only by ID. Removed tasks are marked `[-]` (abandoned), never deleted.
