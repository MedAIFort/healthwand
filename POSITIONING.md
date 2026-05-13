# POSITIONING — HealthWand

**Status:** `v0.1.0-draft.1` (initial positioning lock for revival)
**Locked:** 2026-05-13
**Supersedes:** Implicit positioning in pre-revival `README.md` (HIPAA/GDPR/HITRUST primary, Indonesian regional support as footnote)
**License of this document:** MIT (same as repo)

---

## 0. Why this document exists

Repositioning is the load-bearing decision in this revival. Every downstream choice — `README.md` tagline, `ARCHITECTURE.md` layering, pattern taxonomy, roadmap milestones, the rationale for the planned Python NLP validator, the design of the planned API server, even the choice of which deps to modernize first — flows from positioning. This document locks positioning before any of those decisions are reopened.

Pre-revival HealthWand led with HIPAA / GDPR / HITRUST and treated Indonesian PHI support as a feature footnote. That ordering was wrong: the regulatory wedge that is most timely, least crowded, and most aligned with the maintainer's domain access is **UU PDP**, not the US/EU framing. This document inverts the ordering.

---

## 1. The wedge — one sentence

> HealthWand is the developer-tooling layer for Indonesian PHI detection — a Rust CLI, GitHub Action, and (planned) Bahasa-aware Python NLP validator that helps engineering teams catch data classified as _"specific personal data"_ under UU PDP Article 4(1) before it ships into code, logs, exports, or AI training data.

---

## 2. What HealthWand is

- **Developer tooling, not a compliance product.** The artifact is a CLI you install, a GitHub Action you add to a workflow, a YAML config you edit, and a Python library you import. It runs _in your pipeline_; it does not certify your organization.
- **Indonesian-first by design.** The pattern catalogue, the regulatory framing, the NLP context model, and the documentation language all assume Indonesian healthcare data as the default case. English/Western PHI patterns are supported but secondary.
- **Open source, MIT-licensed.** Distribution and adoption are designed around `cargo install`, `pip install`, GitHub Action Marketplace, and `git clone`. Not enterprise sales-led.
- **Regulatory-grounded, not regulatory-claiming.** Detectors map explicitly to UU PDP categories (Article 4(1)) and other applicable Indonesian regulations. The mapping is documented in `docs/regulatory-mapping.md`. We _cite_ regulations; we do not _certify_ compliance with them.
- **Composable.** The Rust CLI, YAML patterns, and Python NLP layer are independently usable. Teams that already run Presidio, GCP DLP, Azure Health De-identification Service, or AWS Comprehend Medical can adopt HealthWand for its Indonesian patterns alone without replacing their existing pipeline.

---

## 3. What HealthWand is NOT (anti-goals)

These are explicit anti-goals. Pull requests, feature requests, and roadmap items that move HealthWand toward any of these are out of scope.

- **Not a UU PDP / HIPAA / GDPR / HITRUST compliance certification.** Compliance is an organizational and procedural question, not a detection question. We support compliance workflows; we do not deliver them.
- **Not an EHR, SIMRS, or hospital information system.** HealthWand never stores, manages, or transmits patient records. It scans for PHI patterns in artifacts that are not supposed to contain PHI in the first place.
- **Not a hosted SaaS.** The OSS tool is the product. A future hosted dashboard (the original `medaifort.com` premium concept) is deferred and not part of this positioning.
- **Not a generic English-language PHI detector that ships with some Indonesian patterns.** The shape is inverted: Indonesian-first, with English/Western patterns as supported community contributions.
- **Not a replacement for DPIA, DPO appointment, or governance processes.** These remain organizational responsibilities; HealthWand is one technical control among many.
- **Not a synthetic data generator, tokenization vault, or anonymization-as-a-service.** Tools like Tonic.ai, Skyflow, Privacy Analytics serve those needs; HealthWand is detection-and-redaction.

---

## 4. Primary audience — Indonesian AI/health-tech startups

**Shape of the user:** 5–100-person engineering teams shipping health-tech products that touch Indonesian patient data. Examples of categories (not endorsements): telemedicine platforms, clinical decision support tools, hospital information systems built for the Indonesian market, AI-driven diagnostic tools, mental health platforms, patient engagement apps, clinical research pipelines.

**Why this audience is the right anchor:**

- **Shape-of-product match.** HealthWand is developer tooling; these teams are developer-led by default. They have CI/CD on day one, they `cargo install` without procurement friction, and they can adopt OSS in an afternoon.
- **Pain alignment with UU PDP enforcement.** The 3×24-hour breach notification window, the Article 67 criminal penalties (up to 5 years and IDR 5 billion), and the extraterritorial scope hit small teams hardest. A single leak is existentially expensive for a 15-person company.
- **Native domain fluency.** These teams already know what NIK, BPJS, NPWP, STR, No. Rekam Medis, RT/RW, and BPJS Kesehatan claim codes look like. They don't need to be taught the data shape — they need a tool that respects it.
- **Distribution-friendly.** GitHub Stars, npm/PyPI/crates.io downloads, GitHub Action Marketplace installs are all natural metrics for this audience. None of them work for hospital procurement.
- **Community generation.** Startups produce open-source contributors, conference talks, and case studies. Hospitals don't.

**Concrete adoption path:** A startup engineer finds HealthWand via GitHub search, Substack/Bluesky/LinkedIn write-ups, or a referral. They add it to their CI workflow in under an hour. They hit a pattern gap (e.g., a hospital-specific medical record number format), open a PR with a new YAML pattern, and become a contributor.

---

## 5. Secondary audience — Indonesian healthcare orgs (RS, klinik, fasyankes)

**Shape of the user:** Hospitals (Rumah Sakit, both public RSUP/RSUD and private), clinics, primary care facilities (Puskesmas), and other licensed health service facilities (fasilitas pelayanan kesehatan / fasyankes). Their IT teams are typically operations-focused rather than software-engineering-focused, and they buy software more than they build it.

**Explicit boundary — how this audience is reached:**

- **HealthWand reaches hospitals through their vendors, not directly.** Hospital IT departments don't typically adopt a Rust CLI. But the vendors who sell SIMRS, telemedicine platforms, HMIS modules, and AI tools to those hospitals are exactly the primary audience above. When those vendors use HealthWand in their CI, the downstream benefit reaches the hospital.
- **Direct hospital adoption is welcome but not the design center.** A large academic medical center's research division, or a hospital with an internal data engineering team, may use HealthWand directly. That's fine, but the documentation, distribution, and feature priorities are not optimized for them.
- **No vendor lock-in framing.** HealthWand does not position itself as a "hospital compliance tool" because compliance for hospitals is much broader than what any detection tool can cover (Permenkes obligations, BPJS data handling, accreditation standards like KARS/JCI). Hospitals procuring "compliance tools" need legal, GRC, and governance products — not a CI scanner.

**What this means for the product:**

- Documentation is engineer-facing, not procurement-facing.
- No formal "BAA-equivalent" agreements offered. (MIT license + GitHub disclaimer is the legal frame.)
- Hospital-specific patterns (e.g., institutional No. Rekam Medis formats) are accepted as community contributions but not solicited.

---

## 6. Tertiary / expansion audience — multinationals touching Indonesian patient data

**Shape of the user:** Multinational hospital chains operating in Indonesia, international insurance companies covering Indonesian citizens, medical tourism providers in Singapore/Thailand/Malaysia serving Indonesian patients, global health-tech companies with Indonesian operations.

**Explicit framing:**

- This is an **expansion play, not an anchor play.** These organizations already run mature PHI detection — Microsoft Presidio at scale, Google Cloud DLP, Azure Health De-identification Service, AWS Comprehend Medical. Those are well-engineered products serving their target markets.
- **The wedge for this audience is the Indonesian-pattern gap in their existing tools.** A multinational running Presidio doesn't need HealthWand to replace it — they need HealthWand alongside it for NIK, BPJS, NPWP, STR, Indonesian address tokens, and Bahasa context-aware detection that Presidio doesn't ship.
- **Reached when proven, not before.** Multinationals adopt OSS tooling that has demonstrated reliability in the wild. Anchoring on startups generates that demonstration; pursuing multinationals before it exists wastes effort.

---

## 7. The honest regulatory claim

This section defines exactly what HealthWand says, and exactly what it does not say, about each regulatory framework. Honesty here is load-bearing — over-claiming compliance is both legally risky and erodes trust with the technical audience HealthWand needs to win.

### UU PDP (Law No. 27 of 2022)

**What HealthWand says:** HealthWand detects categories of data classified as _"specific personal data"_ (data spesifik) under UU PDP Article 4(1) — including health and medical information, biometric and genetic data, children's data, criminal records, and personal financial data — to support DPIA workflows, pre-deployment scanning, and the 3×24-hour breach-prevention posture required under Article 46.

**What HealthWand does NOT say:**

- HealthWand does not "make you UU PDP compliant." Compliance is procedural and organizational.
- HealthWand does not satisfy the DPO appointment requirements clarified by Constitutional Court Decision No 151/PUU-XXII/2024.
- HealthWand does not satisfy the DPIA requirement; it is a _tool that supports_ DPIA execution.
- HealthWand does not provide breach notification to MOCD or the (forthcoming) PDP Body.

**Implementing regulation status (as of 2026-05-13):** The Draft GR PDP completed its harmonization process in October 2025 and is awaiting Presidential approval; the dedicated PDP Body has not yet been established. HealthWand documentation will be updated to reference specific GR PDP articles once the regulation is enacted.

### HIPAA, GDPR, HITRUST

**What HealthWand says:** HealthWand ships patterns that are useful for organizations subject to HIPAA (US SSNs, US medical record number formats), GDPR (EU patterns are community-contributed), and HITRUST-aligned controls. These are supported as a secondary feature.

**What HealthWand does NOT say:**

- HealthWand is not "HIPAA compliant," "HIPAA certified," or "HITRUST certified." Software does not get HIPAA-certified; organizations do.
- The pre-revival `README.md` claim of "compliance with HIPAA, GDPR, and HITRUST standards" will be rewritten. The honest claim is _"useful within organizations that maintain compliance under these frameworks."_

### Permenkes, BSSN guidance, sectoral health regulations

Documented in `docs/regulatory-mapping.md` (to be drafted). HealthWand detectors will reference specific Permenkes articles where applicable; the mapping is the source of truth, not marketing copy.

---

## 8. Competitive context (neutral)

The PHI-detection landscape is mature for English-language data and underserved for Indonesian-language data. The following are well-engineered tools serving their target markets — HealthWand's position is complementary, not competitive.

| Tool                                 | Strength                                        | Indonesian coverage    |
| ------------------------------------ | ----------------------------------------------- | ---------------------- |
| Microsoft Presidio                   | Mature OSS, broad English entity coverage       | Not shipped by default |
| Google Cloud DLP                     | 150+ entity types, custom detectors, infoTypes  | Not shipped by default |
| Azure Health De-identification       | Healthcare-specific, 27 entity categories       | Not shipped by default |
| AWS Comprehend Medical               | Medical-NER focus, FHIR integration             | Not shipped by default |
| Privacy Analytics, Tonic.ai, Skyflow | De-identification, tokenization, synthetic data | Different category     |

**HealthWand's complementary position:**

- Ships Indonesian patterns (NIK, BPJS, NPWP-16, STR, No. RM, RT/RW, Bahasa address tokens, +62 phone format) as first-class detectors.
- Maps detectors to UU PDP Article 4(1) categories explicitly.
- Designed to run _alongside_ the tools above when an organization needs Indonesian coverage on top of its existing English/Western pipeline.

---

## 9. Sequencing — why startups now, hospitals later, multinationals last

This is the Build → Works → Community sequencing applied to audience:

1. **Build phase (now → v1.0):** Anchor entirely on startups. Documentation, distribution, examples, and roadmap all optimize for the developer-led adoption case.
2. **Works phase (v1.0 → v1.x):** As startup adoption produces real-world deployments and case studies, hospital-vendor adoption becomes natural. Documentation expands to address vendor-facing concerns (audit logs, deployment patterns, supply-chain attestations).
3. **Community / expansion phase (v2.0+):** Once HealthWand has demonstrated reliability and ecosystem depth, multinational adoption becomes the expansion vector. At that point — and not before — features like enterprise SSO, attested releases, signed pattern bundles, and SLA-backed support become reasonable considerations.

Reversing this sequence (chasing multinationals or hospital procurement first) is the most common failure mode for OSS health-tech tools. We're explicitly not doing that.

---

## 10. Success criteria for this repositioning

The repositioning is working if, in the 6 months following its lock:

- The `README.md` tagline change measurably shifts search-discovery (GitHub search for "Indonesian PHI", "UU PDP", "Bahasa NER" begins surfacing the repo).
- At least 3 community pattern contributions land (Indonesian-context patterns that the maintainer would not have prioritized).
- At least 1 external write-up (blog, conference talk, Substack post) references HealthWand by name in an Indonesian health-tech context.
- The repository receives its first non-maintainer issue describing a real deployment scenario.

The repositioning is not working if, in the same window:

- All inbound interest is about HIPAA compliance and US PHI.
- Discussion centers on enterprise/SaaS features rather than OSS adoption.
- Hospital procurement teams (not engineers) are the primary inbound channel.

These are observable signals, not vanity metrics. If they trend the wrong way, the positioning is revisited — not the artifact.

---

## 11. Open questions — deliberately deferred

These are real unknowns the positioning does not pretend to resolve:

- **Whether the planned Python NLP validator is still the right shape.** The original README listed it as a planned feature. With Bahasa-first context-aware detection as the rationale, it has a defensible role — but specific implementation choices (spaCy with Indonesian models? IndoNLU-derived models? A lighter regex+context-rules layer first?) are deferred to `ARCHITECTURE.md`.
- **Whether the planned API server is still needed.** Its original rationale (real-time PHI detection for platforms) is weaker now that the GitHub Action covers most of the pre-deployment use case. Deferred.
- **Whether `phi-detector` should rename to something Indonesian-context-signaling.** **RESOLVED** — renamed to `healthwand` (M1 migration milestone).
- **How to handle the broader `MedAIFort` org branding.** The org-level positioning is separate from this repo's positioning; deferred until after this repo's revival is shipped.

---

## Appendix A — Indonesian regulatory references cited in this document

- **UU PDP**: Law No. 27 of 2022 on Personal Data Protection (Undang-Undang Pelindungan Data Pribadi). Enacted 17 October 2022; transition period ended 17 October 2024.
- **Draft GR PDP / RPP PDP**: Government Regulation implementing UU PDP. Harmonization completed October 2025; awaiting Presidential approval as of early 2026.
- **Constitutional Court Decision No 151/PUU-XXII/2024** (30 July 2025): Clarification on DPO appointment criteria.
- **PP TUNAS / GR 17/2025**: Government Regulation on Governance of Electronic Systems for Child Protection.
- **MOCDA Reg 5/2025**: Public Scope Electronic System Operator regulation.
- **GR 71/2019**: Operation of Electronic Systems and Transactions.
- **EIT Law**: Law No. 11 of 2008 on Electronic Information and Transactions, as amended by Law No. 1 of 2024 (Second Amendment).
- **Law No. 1 of 2026** (2 January 2026): Adjustment of Criminal Provisions.
- **Permenkes** and sectoral health regulations: to be enumerated in `docs/regulatory-mapping.md`.

---

## Change log

- **v0.1.0-draft.1** (2026-05-13) — Initial positioning lock. Primary anchor: Indonesian AI/health-tech startups. Secondary anchor: hospitals/fasyankes (reached via vendors). Tertiary: multinationals (expansion only).
