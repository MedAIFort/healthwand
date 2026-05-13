# PHI Taxonomy — Indonesia

**Status:** `v0.1.0-draft.1`
**Locked:** 2026-05-13
**Aligned with:** `POSITIONING.md` v0.1.0-draft.1, `README.md` v0.2.0-draft.1, `docs/regulatory-mapping.md` v0.1.0-draft.1
**License of this document:** MIT (same as repo)

---

## 0. Purpose, scope, and conventions

### 0.1 What this document is

This is the complete catalogue of Indonesian PHI detectors HealthWand ships, with for each:

- **Format and structure** of the identifier in real-world Indonesian data.
- **Regex starting point** — calibrated for high precision, not maximum recall. These are _starting points_ for production refinement, not final regexes.
- **Synthetic examples** — never real PII.
- **False-positive vectors** — what looks like this pattern but isn't.
- **Context disambiguation strategy** — context words and adjacent-token cues.
- **Severity and UU PDP category** — cross-referenced to `docs/regulatory-mapping.md`.
- **Detection approach decision** — regex-only, regex + context, dictionary lookup, or NLP-required.

### 0.2 What this document is NOT

- Not a complete legal taxonomy of personal data under Indonesian law. Counsel should be consulted for production deployments.
- Not exhaustive of every Indonesian identifier. §8 lists known adjacent identifiers deliberately out of the current catalogue.
- Not regex-tested against a labeled corpus. The regexes here are designed to be syntactically and semantically correct against published format specifications; benchmark-quality precision and recall numbers will land in v0.2+.

### 0.3 Regex conventions

- All regexes are written in Rust regex flavor (compatible with the `regex` crate used by `phi-detector`). This is a subset of PCRE; notably, no lookaheads/lookbehinds.
- `\b` word boundaries used aggressively to prevent bleed-through.
- Anchors (`^`, `$`) omitted unless line-level matching is intended.
- Character class shorthand (`\d`, `\s`, `\w`) used in preference to bracket expressions for readability.
- Where context words are required, the regex matches the identifier; the context check is implemented separately in the scanner.

### 0.4 Example data ethics

Every example identifier in this document is **synthetic**. NIK examples use province code `00` or `99` (not allocated). NPWP examples use clearly invalid check digits. BPJS, phone, and DOB examples use obviously synthetic patterns. Contributors adding new patterns must follow the same rule — never paste a real identifier into the catalogue, even one's own.

### 0.5 Severity tiers

Tied to `docs/regulatory-mapping.md`:

- **Critical** — Specific personal data under UU PDP Art 4(1). Direct mapping to health, biometric, genetic, criminal-record, child, or financial categories.
- **High** — General personal data with high identifiability or that, in combination, collapses to specific-personal-data severity.
- **Medium** — General personal data with moderate identifiability.
- **Combinatorial** — Severity elevated when multiple detectors fire in the same context window.

---

## 1. Catalogue at a glance

| #   | Detector                   | Type                            | Default severity               | Detection approach                        | Status            |
| --- | -------------------------- | ------------------------------- | ------------------------------ | ----------------------------------------- | ----------------- |
| 1   | BPJS Kesehatan             | Specific (Art 4(1) — health)    | Critical                       | Regex + context                           | Implemented       |
| 2   | No. Rekam Medis            | Specific (Art 4(1) — health)    | Critical                       | Regex + context                           | Implemented       |
| 3   | ICD-10 code                | Specific (Art 4(1) — health)    | Critical (in clinical context) | Regex + context                           | Implemented       |
| 4   | NPWP (15-digit / 16-digit) | Specific (Art 4(1) — financial) | Critical                       | Regex + context                           | Implemented       |
| 5   | Drug names (FORNAS subset) | Specific (Art 4(1) — health)    | Critical (in patient context)  | Dictionary + context                      | Planned (v1.0)    |
| 6   | Children's-data flag       | Specific (Art 4(1) — children)  | Critical                       | Combinatorial                             | Planned (v1.x)    |
| 7   | Diagnosis text (Bahasa)    | Specific (Art 4(1) — health)    | Critical                       | NLP-required                              | Planned (v1.x)    |
| 8   | NIK (KTP)                  | General (Art 4(2))              | High                           | Regex + context + structure               | Implemented       |
| 9   | Indonesian phone           | General                         | High                           | Regex                                     | Implemented       |
| 10  | Indonesian address tokens  | General                         | High                           | Regex + token co-occurrence               | Implemented       |
| 11  | Indonesian DOB             | General                         | Medium                         | Regex                                     | Implemented       |
| 12  | Indonesian names           | General                         | High                           | NLP-required (regex for honorifics today) | Partial / Planned |
| 13  | STR                        | Professional licensing          | Medium                         | Regex + context                           | Planned (v1.0)    |

---

## 2. Specific personal data detectors (UU PDP Art 4(1))

### 2.1 BPJS Kesehatan

**Full name:** Nomor Peserta BPJS Kesehatan (Badan Penyelenggara Jaminan Sosial Kesehatan participant number).

**Regulatory category:** Health data — Specific personal data under UU PDP Art 4(1). Sectoral overlap with Permenkes 24/2022 (RME).

**Format:**

- 13 digits.
- No publicly documented internal structure; assigned sequentially by BPJS Kesehatan.

**Regex (starting point):**

```regex
\b\d{13}\b
```

**Synthetic examples:**

- `0001234567890`
- `0009876543210`

**Required context words:** `BPJS`, `JKN`, `BPJSKesehatan`, `Peserta BPJS`, `No. BPJS`, `Nomor BPJS`, `Kartu BPJS`.

**False-positive vectors:**

- Other 13-digit identifiers (some bank reference numbers, transaction IDs).
- 13-digit phone numbers (rare; Indonesian mobile numbers are typically 10–12 digits but 13 occurs).
- 13-digit timestamps or epoch milliseconds.

**Detection approach:** Regex + context. The 13-digit regex alone has high false-positive rate; context word co-occurrence within a configurable window (default ±100 characters) reduces FPs dramatically.

**Notes:** BPJS Ketenagakerjaan (the employment-injury insurance program) is a separate identifier with a different format and is out of scope for the health-PHI catalogue. See §8.

---

### 2.2 No. Rekam Medis

**Full name:** Nomor Rekam Medis (medical record number).

**Regulatory category:** Health data — Specific personal data under UU PDP Art 4(1). Required minimum-field per Permenkes 24/2022 / Kepmenkes HK.01.07/MENKES/1423/2022.

**Format:**

- **Highly variable per fasyankes.** No standardized national format.
- Common observed patterns:
  - Pure sequential digits: `\d{6,10}`
  - Prefixed: `RM-\d+`, `MRN-\d+`, `MR\d+`
  - Date-encoded: `YYYY-NNNN`, `MM-NNNN`
  - Hospital-specific: `RS<code>-<sequence>`

**Regex (starting points — multiple patterns):**

```regex
\b(?:RM|MRN|MR)[-\s]?\d{4,10}\b
\b\d{6,10}\b   # with mandatory context
```

**Synthetic examples:**

- `RM-1234567`
- `MRN-12345678`
- `0001234`

**Required context words:** `RM`, `Rekam Medis`, `No. RM`, `Nomor RM`, `MRN`, `MedRec`, `No. Rekam Medis`, `NoRM`.

**False-positive vectors:**

- Order numbers, invoice numbers, ticket numbers (anything that's a sequential 6–10 digit identifier).
- The bare-digit pattern is dangerous without context; this is the highest-FP detector in the catalogue.

**Detection approach:** Regex + context, with mandatory context word presence for the bare-digit case. The prefixed patterns (`RM-`, `MRN-`) can match without context.

**Notes:** Hospital-specific MRN formats are accepted as community-contributed YAML patterns. Per `POSITIONING.md` §5, fasyankes-specific patterns are accepted but not solicited; the catalogue ships a reasonable default and lets organizations extend it.

---

### 2.3 ICD-10 code

**Full name:** International Statistical Classification of Diseases and Related Health Problems, 10th revision. WHO standard.

**Regulatory category:** Health data — Specific personal data under UU PDP Art 4(1) when associated with a patient. Standalone ICD-10 codes (e.g., in a medical textbook) are not PHI; ICD-10 codes adjacent to patient identifiers are.

**Format:**

- One letter (A–Z) + 2 digits, optionally followed by `.` + 1–2 digits for subcategories.
- Examples: `I21.0` (acute myocardial infarction, anterior wall), `A09` (other gastroenteritis), `J45.901`.

**Regex (starting point):**

```regex
\b[A-TV-Z]\d{2}(?:\.\d{1,2})?\b
```

(`U` excluded because U00–U85 is reserved for special purposes; uncommon in clinical use.)

**Synthetic examples:**

- `I21.0`
- `A09`
- `J45.901`

**Required context words (for severity escalation):** `diagnosis`, `Dx`, `ICD`, `kode ICD`, `kode diagnosis`. Without context, severity defaults to Medium; with context, escalates to Critical.

**False-positive vectors:**

- Aircraft model designators (`A320`, `B737`) — pattern can match if shorter codes are allowed.
- Chemical compounds in scientific text.
- Equation references in academic papers.
- Phonetic alphabet sequences.

**Detection approach:** Regex + optional context for severity escalation. Standalone code matches are reported as informational; co-occurrence with patient identifiers (NIK, No. RM, BPJS) elevates severity.

---

### 2.4 NPWP (Nomor Pokok Wajib Pajak)

**Full name:** Indonesian Tax Identification Number.

**Regulatory category:** Personal financial data — Specific personal data under UU PDP Art 4(1)e.

**Format:**

- **Legacy (15-digit), pre-July 2024:** `XX.XXX.XXX.X-XXX.XXX`
  - Digits 1–9: Taxpayer identifier.
  - Digit 10: Check digit.
  - Digits 11–13: KPP (Kantor Pelayanan Pajak / tax office) code.
  - Digits 14–15: Branch code (`000` for headquarters).
- **Current (16-digit), effective 1 July 2024:** Per DGT Regulation PER-6/PJ/2024, implementing MoFR 112/PMK.03/2022 as amended by MoFR 136/2023.
  - **For Indonesian resident individuals:** NPWP = NIK (the 16-digit national ID is used directly as the tax ID).
  - **For non-resident individuals, corporate taxpayers, government agencies:** A separate 16-digit NPWP is assigned, typically formed by prepending `0` to the legacy 15-digit number.
- **Transition:** The 15-digit format remains legally valid (per PER-6/PJ/2024) for documents issued after 1 July 2024; "other parties" were given until 31 December 2024 to adopt the new format.

**Regex (starting points — both formats):**

```regex
# Legacy 15-digit with dot/dash punctuation
\b\d{2}\.\d{3}\.\d{3}\.\d-\d{3}\.\d{3}\b
# Legacy 15-digit unformatted
\b\d{15}\b
# Current 16-digit
\b\d{16}\b
```

**Synthetic examples:**

- Legacy formatted: `00.000.000.0-000.000`
- Legacy unformatted: `000000000000000`
- 16-digit: `0000000000000000`

**Required context words:** `NPWP`, `Nomor Pokok Wajib Pajak`, `Wajib Pajak`, `WP`, `Pajak`, `pajak`.

**False-positive vectors:**

- **The 16-digit NPWP regex collides with the NIK regex** — both are `\b\d{16}\b`. For Indonesian resident individuals, the same 16-digit number is both NIK and NPWP. Context determines categorization. See §4.1.
- 15-digit numbers are also used elsewhere (some bank account numbers, transaction IDs).
- Without the dot/dash formatting, the 15-digit form is high-FP.

**Detection approach:** Regex + context. Context word presence is mandatory for severity escalation.

**Notes:**

- For Indonesian resident individuals, the same 16-digit value is both NIK and NPWP. HealthWand reports it under the **higher-severity** category based on context: NPWP context → Specific (financial); NIK context with no financial context → General. When both contexts are present, severity is the maximum.
- NITKU (Nomor Identitas Tempat Kegiatan Usaha), the 22-digit business-location identifier introduced by PER-6/PJ/2024, is out of catalogue (see §8). It is not personal data — it is a location identifier.

---

### 2.5 Drug names (FORNAS subset)

**Full name:** Indonesian National Formulary drug names and common Indonesian-market brand names.

**Regulatory category:** Health data — Specific personal data under UU PDP Art 4(1) when associated with a patient's prescription or treatment record. Standalone drug names (e.g., in pharmaceutical product listings) are not PHI.

**Format:**

- Dictionary lookup, not regex.
- Two pattern sets:
  - **Generic names** from FORNAS (Formularium Nasional) and INN (International Nonproprietary Name) lists: `parasetamol`, `amoksisilin`, `metformin`, `simvastatin`, `omeprazol`, `ranitidin`, `ceftriakson`, `furosemid`, `bisoprolol`, etc.
  - **Brand names** for the Indonesian market: dictionary maintained as community-extensible YAML.

**Implementation:**

```yaml
patterns:
  - name: Drug name (FORNAS generic subset)
    type: dictionary
    dictionary: drugs_fornas_id.yaml
    score: 0.85
    context_words:
      [
        obat,
        resep,
        dosis,
        mg,
        tablet,
        kapsul,
        sirup,
        salep,
        oles,
        suntik,
        infus,
      ]
```

**Synthetic examples:**

- `Pasien diberi parasetamol 500mg 3x sehari.`
- `Resep: amoksisilin 500mg PO TID.`

**Required context words:** `obat`, `resep`, `dosis`, `mg`, `mL`, `tablet`, `kapsul`, `sirup`, `salep`, `infus`, `IV`, `IM`, `PO`, `SC`.

**False-positive vectors:**

- Pharmaceutical company materials, pharmacology textbooks, drug-information leaflets — where drug names appear without patient association.
- News articles about pharmaceuticals.

**Detection approach:** Dictionary lookup + context. Severity escalates only when both the drug-name match and a patient-context indicator co-occur.

**Notes:** Status: planned for v1.0. The FORNAS dictionary is maintained externally; HealthWand ships a subset (top ~200 commonly used generic names) and accepts community-extensible patterns.

---

### 2.6 Children's-data flag (combinatorial)

**Full name:** Detection of personal data that pertains to a minor.

**Regulatory category:** Children's data (data anak) — Specific personal data under UU PDP Art 4(1)d. Subject to PP TUNAS (GR 17/2025) for electronic systems serving children. Per the Draft GR PDP, a child is an individual under 18 years old and unmarried.

**Format:**

- Combinatorial. Triggered when one of:
  - A DOB detector matches with a year that places the data subject under 18.
  - Age context words (`umur <N>`, `usia <N>`, `tahun`) match with `<N>` under 18.
  - Pediatric-care context words (`anak`, `pediatri`, `bayi`, `balita`, `neonatus`, `remaja`) co-occur with patient-identifying detectors.

**Implementation:**

```yaml
patterns:
  - name: Children's-data flag
    type: combinatorial
    triggers:
      - dob_year: { since_now_lt: 18 }
      - age_text: { lt: 18 }
      - context_words: [anak, pediatri, bayi, balita, neonatus, remaja]
    score: 1.0
```

**Synthetic examples:**

- `Pasien anak usia 8 tahun, BPJS: 0001234567890`
- `Bayi laki-laki, lahir 12/01/2025`

**Detection approach:** Combinatorial. Status: planned for v1.x — depends on the combinatorial-severity engine that isn't in v1.0.

**Notes:** PP TUNAS imposes additional obligations on Electronic System Operators serving children, including a DPIA at least three months before launch. The children's-data flag is the highest-severity finding in the catalogue.

---

### 2.7 Diagnosis text (Bahasa Indonesia)

**Full name:** Free-text diagnosis descriptions in Bahasa Indonesia.

**Regulatory category:** Health data — Specific personal data under UU PDP Art 4(1) when associated with a patient. Permenkes 36/2012 (Rahasia Kedokteran) applies.

**Format:**

- Free-text. Not regex-amenable.
- Examples (synthetic): "Pasien menderita diabetes melitus tipe 2 dengan komplikasi nefropati", "Riwayat hipertensi grade II, dislipidemia, obesitas".

**Detection approach:** NLP-required. This is the strongest case for the planned Python NLP validator. A regex-only approach has unacceptable FN rate; a dictionary-only approach has unacceptable FP rate.

**Status:** Planned for v1.x. The Python NLP validator (see `POSITIONING.md` §11 and forthcoming `ARCHITECTURE.md`) is the natural place for this detector. Indonesian-language medical NER models (IndoNLU-derived or fine-tuned) are the most likely implementation path; specific model selection is deferred to `ARCHITECTURE.md`.

**Notes:** Until the NLP validator lands, organizations can approximate this detector with dictionary-based lookup of common ICD-10 plain-language equivalents (`diabetes`, `hipertensi`, `asma`, `tuberkulosis`, `HIV`, etc.) plus context words. This is a v1.0-shippable interim.

---

## 3. General personal data detectors (UU PDP Art 4(2))

### 3.1 NIK (KTP)

**Full name:** Nomor Induk Kependudukan — the 16-digit Indonesian national identity number printed on the KTP (Kartu Tanda Penduduk).

**Regulatory category:** General personal data under UU PDP Art 4(2). Required minimum field per Permenkes 24/2022. **For Indonesian resident individuals, the NIK is also their NPWP** (see §2.4).

**Format:**

- 16 digits with the following structure:

| Positions | Length | Meaning                                                                        |
| --------- | ------ | ------------------------------------------------------------------------------ |
| 1–2       | 2      | Province code (per Kemendagri)                                                 |
| 3–4       | 2      | Regency/city code                                                              |
| 5–6       | 2      | District (kecamatan) code                                                      |
| 7–8       | 2      | Day of birth (`DD`, with `DD + 40` for women, so women's day digits are 41–71) |
| 9–10      | 2      | Month of birth (`MM`, 01–12)                                                   |
| 11–12     | 2      | Year of birth (`YY`, last two digits)                                          |
| 13–16     | 4      | Sequential number within the same registration unit (0001–9999)                |

**Regex (starting point):**

```regex
\b\d{16}\b
```

A structurally validated regex would constrain province codes and birth digits, but is more brittle to false-negative; the catalogue ships the permissive form and uses context to disambiguate.

**Synthetic examples:**

- `0000000000000000` (invalid province code 00 — clearly synthetic)
- `9999999999999999` (invalid province code 99)

**Required context words:** `NIK`, `KTP`, `Nomor Induk Kependudukan`, `No. KTP`, `Nomor KTP`, `Identitas`, `Kependudukan`.

**False-positive vectors:**

- **NPWP-2024** (same length, same regex). Context determines categorization.
- 16-digit credit card numbers (Visa/Mastercard PANs). The pattern looks identical without context; Luhn validation could distinguish but is not currently implemented.
- 16-digit bank account numbers (some Indonesian banks use 16-digit accounts).
- Arbitrary 16-digit sequences in logs, IDs, hashes.

**Detection approach:** Regex + context + (optional) structural validation. Structural validation is opt-in via YAML config; default ships permissive.

**Notes:**

- The DOB encoding inside NIK means a successful NIK match also implicitly contains a DOB. Combinatorial rules in §5 elevate severity accordingly.
- The `DD + 40` convention for women is per the Kemendagri NIK specification.

---

### 3.2 Indonesian phone numbers

**Format:**

- Mobile numbers in Indonesia use either international prefix `+62` or domestic prefix `0`, followed by `8`, followed by a 1- or 2-digit operator code, followed by 7–10 digits.
- Total digit count (excluding `+`): 10–13.
- Operator prefixes:

| Prefix (after 8)                   | Operator (typical)     |
| ---------------------------------- | ---------------------- |
| 11, 12, 13, 21, 22, 23, 51, 52, 53 | Telkomsel              |
| 14, 15, 16, 55, 56, 57, 58         | Indosat                |
| 17, 18, 19, 31, 32, 33, 38, 59     | XL Axiata              |
| 81–89 (variable)                   | Various (with overlap) |
| 81–87 (subset)                     | Smartfren              |
| 95–99                              | Three / 3              |

Note: Prefix-to-operator mapping has shifted over time due to mergers (Indosat-Hutchison, XL-Smart, etc.). Detection should treat any `8` + 1- or 2-digit prefix as valid.

**Regex (starting point):**

```regex
(?:\+62|62|0)8\d{8,11}
```

**Synthetic examples:**

- `+6281234567890`
- `081234567890`
- `+62 812 3456 7890` (with separators — additional regex needed)

**Required context words:** Optional. Phone numbers are recognizable without context due to the `+62` / `08` prefix; context elevates severity but is not required for detection.

**False-positive vectors:**

- Test fixtures and example phone numbers (`+6281234567890` is a common placeholder).
- 11- to 13-digit transaction IDs starting with `0`.

**Detection approach:** Regex. The prefix structure makes this one of the highest-precision detectors in the catalogue.

**Notes:** Separator handling (`-`, ` `, `.`) requires a more permissive regex with optional separators. The catalogue ships both a strict and a permissive variant.

---

### 3.3 Indonesian address tokens

**Format:**
Indonesian addresses are composed of recognizable token markers, each of which is highly distinctive:

| Token                | Indonesian     | English              | Pattern          |
| -------------------- | -------------- | -------------------- | ---------------- | -------------- |
| `Jl.` / `Jalan`      | Jalan          | Street               | `\b(?:Jl\.?      | Jalan)\s+`     |
| `Gg.` / `Gang`       | Gang           | Alley                | `\b(?:Gg\.?      | Gang)\s+`      |
| `RT`                 | Rukun Tetangga | Neighborhood unit    | `\bRT\s*\d{1,3}` |
| `RW`                 | Rukun Warga    | Citizen association  | `\bRW\s*\d{1,3}` |
| `Kel.` / `Kelurahan` | Kelurahan      | Sub-district (urban) | `\b(?:Kel\.?     | Kelurahan)\s+` |
| `Desa`               | Desa           | Village (rural)      | `\bDesa\s+`      |
| `Kec.` / `Kecamatan` | Kecamatan      | District             | `\b(?:Kec\.?     | Kecamatan)\s+` |
| `Kab.` / `Kabupaten` | Kabupaten      | Regency              | `\b(?:Kab\.?     | Kabupaten)\s+` |
| `Kota`               | Kota           | City                 | `\bKota\s+`      |
| `Prov.` / `Provinsi` | Provinsi       | Province             | `\b(?:Prov\.?    | Provinsi)\s+`  |

**Regex (composite — match any one token):**

```regex
\b(?:Jl\.?|Jalan|Gg\.?|Gang|RT|RW|Kel\.?|Kelurahan|Desa|Kec\.?|Kecamatan|Kab\.?|Kabupaten|Kota|Prov\.?|Provinsi)\b
```

**Synthetic examples:**

- `Jl. Sudirman No. 1, RT 01/RW 02, Kel. Senayan, Kec. Tanah Abang, Kota Jakarta Pusat`
- `Desa Sukamaju, Kec. Sukamulya, Kab. Bandung`

**Required context words:** None — these tokens are themselves the context.

**False-positive vectors:**

- `Jl.` as an unintended initialism in code or formatted text (rare).
- `RT` as a register-transfer or routing-table mnemonic in technical contexts.
- `Kota` as a personal name component (some Indonesian names contain `Kota`).

**Detection approach:** Regex + token co-occurrence. A single `Jl.` match is informational; co-occurrence of `Jl.` + `RT` + `Kec.` within a sliding window is High severity (a full Indonesian address).

**Notes:** Address detection in Indonesian is a precision-favorable problem because the tokens are highly distinctive and rarely appear coincidentally outside actual addresses.

---

### 3.4 Indonesian DOB

**Format:**
Common Indonesian date formats:

| Format                         | Example           |
| ------------------------------ | ----------------- |
| `dd/mm/yyyy`                   | `12/01/1985`      |
| `dd-mm-yyyy`                   | `12-01-1985`      |
| `dd.mm.yyyy`                   | `12.01.1985`      |
| `d Month yyyy` (Bahasa months) | `12 Januari 1985` |
| `dd MMM yyyy` (abbreviated)    | `12 Jan 1985`     |

Bahasa month names: Januari, Februari, Maret, April, Mei, Juni, Juli, Agustus, September, Oktober, November, Desember.

Bahasa month abbreviations (3-letter): Jan, Feb, Mar, Apr, Mei, Jun, Jul, Agu (or Ags), Sep, Okt, Nov, Des.

**Regex (starting points):**

```regex
# Numeric format with / or - separator
\b(0?[1-9]|[12][0-9]|3[01])[/-](0?[1-9]|1[0-2])[/-](19\d{2}|20\d{2})\b

# Bahasa month name format
\b(0?[1-9]|[12][0-9]|3[01])\s+(Januari|Februari|Maret|April|Mei|Juni|Juli|Agustus|September|Oktober|November|Desember)\s+(19\d{2}|20\d{2})\b
```

**Synthetic examples:**

- `01/01/1990`
- `12 Januari 1985`
- `25 Des 2010` (potential children's-data trigger)

**Required context words:** `lahir`, `tanggal lahir`, `DOB`, `tgl. lahir`, `kelahiran`. Without context, a date is informational; with context, it's a DOB detection.

**False-positive vectors:**

- Any other date in the document (admission date, discharge date, today's date, document creation date).
- Without context, the regex matches every date.

**Detection approach:** Regex + context. The context word requirement is non-optional for severity.

**Notes:** DOB is also implicitly encoded in NIK (positions 7–12). When a NIK match fires, the embedded DOB is extractable; the catalogue does not currently double-report.

---

### 3.5 Indonesian names (Bahasa patterns)

**Format:**
Names themselves do not pattern-match in Bahasa Indonesia — there is no Indonesian-specific name token. Detection relies on **honorifics and titles** that precede the name.

Common honorifics:

| Honorific           | Meaning                              | Usage                                                          |
| ------------------- | ------------------------------------ | -------------------------------------------------------------- |
| `Pak` / `Bapak`     | Mr. / Sir                            | Adult male, formal/informal                                    |
| `Bu` / `Ibu`        | Mrs. / Ma'am                         | Adult female, formal/informal                                  |
| `Mas`               | older brother, informal              | Younger to slightly older male, informal                       |
| `Mbak`              | older sister, informal               | Younger to slightly older female, informal                     |
| `Sdr.` / `Saudara`  | brother / Mr.                        | Adult male, semi-formal                                        |
| `Sdri.` / `Saudari` | sister / Ms.                         | Adult female, semi-formal                                      |
| `Tn.` / `Tuan`      | Mr. / Sir                            | Adult male, formal                                             |
| `Ny.` / `Nyonya`    | Mrs.                                 | Adult female (married), formal                                 |
| `Nn.` / `Nona`      | Ms.                                  | Adult female (unmarried), formal                               |
| `Dr.` / `dr.`       | Doctor (academic) / Doctor (medical) | Lowercase `dr.` is medical doctor; uppercase `Dr.` is doctoral |

**Regex (starting point):**

```regex
\b(?:Pak|Bapak|Bu|Ibu|Mas|Mbak|Sdr\.?|Saudara|Sdri\.?|Saudari|Tn\.?|Tuan|Ny\.?|Nyonya|Nn\.?|Nona|Dr\.?|dr\.?|Prof\.?|H\.?|Hj\.?)\s+[A-Z][a-zA-ZÀ-ÿ'\-]+(?:\s+[A-Z][a-zA-ZÀ-ÿ'\-]+){0,3}\b
```

**Synthetic examples:**

- `Pak Budi Santoso`
- `Ibu Sari Wulandari`
- `dr. Adi Permana, SpAn`

**Required context words:** None — the honorific is itself the context.

**False-positive vectors:**

- Generic mentions in non-patient context: `Pak Budi karyawan kebersihan` (the cleaning staff Pak Budi) versus `Pak Budi pasien diabetes` (the patient Pak Budi).
- This is the canonical case where regex alone is insufficient. **The Bahasa-aware NLP validator (planned, v1.x) is specifically motivated by this detector.**

**Detection approach:** Regex (honorific + capitalized tokens) for v1.0; NLP for v1.x.

**Notes:** Indonesian names often include religious or cultural prefixes (`H.`, `Hj.` for Hajj-completed individuals, `Raden` / `R.` / `R.A.` for Javanese aristocratic titles). The regex above covers `H.` and `Hj.` but not the aristocratic prefixes — those are accepted as community contributions.

---

### 3.6 STR (Surat Tanda Registrasi)

**Full name:** Surat Tanda Registrasi — the registration certificate issued to Indonesian health workers by the relevant licensing council (KKI for physicians, MTKI for other health workers).

**Regulatory category:** Professional licensing data. Identifies the health worker (controller-side, not patient). Lower severity than patient identifiers but still personal data under UU PDP Art 1.

**Format:**

- Issued by the relevant council with specialty-specific numbering.
- Physician STR (KKI) typically encodes specialty/branch information and is 14+ digits in length, often presented with separators.
- Other health worker STR formats vary.

**Regex (starting point — conservative):**

```regex
\bSTR[-\s:]*\d{6,20}\b
```

**Synthetic examples:**

- `STR-12345678901234`
- `STR: 1234567890`

**Required context words:** `STR`, `Surat Tanda Registrasi`, `Nomor STR`, `KKI`, `MTKI`, `Konsil Kedokteran`.

**False-positive vectors:**

- The bare digit pattern is highly FP-prone without `STR` prefix or context.
- Conservative approach is to require the `STR` token explicitly.

**Detection approach:** Regex requiring the `STR` token. Status: planned for v1.0. The exact STR format specification varies by council; the catalogue ships a permissive regex and accepts council-specific patterns as community contributions.

**Notes:** STR is controller-side personal data — it identifies the clinician, not the patient. Leakage of STR is a personal-data concern but not strictly a PHI concern. Severity is Medium by default; organizations handling STR data should configure higher severity if their threat model warrants.

---

## 4. Cross-pattern collisions and disambiguation

Some Indonesian identifiers share regex shape, requiring context-based disambiguation.

### 4.1 NIK ↔ NPWP-2024

**Collision:** Both are 16-digit numeric strings. For Indonesian resident individuals, the same 16-digit value is _both_ the NIK and the NPWP (per PER-6/PJ/2024).

**Disambiguation strategy:**

1. Detect the 16-digit pattern.
2. Look for context words in a window (default ±100 characters):
   - NIK context (`NIK`, `KTP`, `Kependudukan`) → categorize as NIK (General personal data, High severity).
   - NPWP context (`NPWP`, `Wajib Pajak`, `Pajak`) → categorize as NPWP (Specific financial, Critical severity).
   - Both contexts present → categorize as **the higher severity** (NPWP / Critical).
   - Neither context → categorize as ambiguous 16-digit identifier (Medium severity, informational).

**Implementation note:** The current `phi-detector` reports both findings if both contexts are present; the SARIF-style consumer is expected to deduplicate by position. The combinatorial-severity engine planned for v1.x will collapse these to a single finding with maximum severity.

### 4.2 BPJS Kesehatan ↔ Indonesian phone (13-digit edge case)

**Collision:** A 13-digit BPJS number can syntactically resemble a 13-digit phone number with country code (`62` + 11 digits), though phones in practice start with `+62` or `08` rather than bare `62`.

**Disambiguation strategy:**

1. Phone-number regex requires `+62` or `08` prefix → no overlap with bare-13-digit BPJS.
2. If a 13-digit sequence appears without `+`, `08`, or BPJS context → ambiguous, default to informational.

### 4.3 NIK ↔ DOB (embedded)

**Collision:** A successful NIK match implicitly contains a DOB at positions 7–12.

**Disambiguation strategy:**

- Do not double-report. NIK is the canonical detection; the embedded DOB is extractable for combinatorial-severity purposes but not surfaced as a separate finding.

### 4.4 Bare digit-sequence ambiguity (NIK / NPWP / BPJS / credit card / phone / arbitrary)

**Strategy:** All bare-digit detectors require context word presence for severity escalation. The regex match is informational by default; context determines severity.

---

## 5. Combinatorial sensitivity rules

UU PDP Article 1 defines personal data broadly — _"identified or capable of being identified independently or in combination with other information."_ Combinatorial detection rules implement this:

| Co-occurring detectors                             | Default elevated severity                                    | Rationale                               |
| -------------------------------------------------- | ------------------------------------------------------------ | --------------------------------------- |
| NIK + BPJS + No. RM                                | Critical (specific personal data — health context confirmed) | Health record linkage                   |
| Name (honorific match) + BPJS / No. RM / drug name | Critical                                                     | Patient identification with health data |
| Name + address + DOB                               | Critical (combinatorial identifiability)                     | Re-identification risk                  |
| NIK + DOB (explicit, redundant)                    | High                                                         | Confirms NIK encodes a real birth date  |
| BPJS + drug name                                   | Critical                                                     | Treatment record linkage                |
| DOB < 18 years + any identifier                    | Critical (children's-data flag)                              | PP TUNAS / UU PDP Art 4(1)d             |

**Status:** The combinatorial-severity engine is planned for v1.x. v1.0 reports each match individually; downstream tooling (SARIF consumers, security dashboards) can implement combinatorial rules until the engine lands in HealthWand itself.

---

## 6. Detection approach decisions

Each detector falls into one of four approaches. The choice is calibrated against the precision/recall tradeoff and the cost of false positives versus false negatives.

| Approach                        | Detectors using it                                                        | Precision | Recall        | Notes                                        |
| ------------------------------- | ------------------------------------------------------------------------- | --------- | ------------- | -------------------------------------------- |
| **Regex-only**                  | Indonesian phone (high prefix discriminator), ICD-10 (without escalation) | High      | Moderate      | When the format itself is highly distinctive |
| **Regex + context**             | NIK, BPJS, NPWP, No. RM, DOB, address tokens, STR, drug name (text)       | High      | High          | The default approach; most detectors         |
| **Dictionary lookup + context** | Drug names (FORNAS), Indonesian-market brand names                        | High      | Moderate      | When the pattern is a finite enumeration     |
| **NLP-required**                | Indonesian names (production-quality), diagnosis text (Bahasa)            | High      | High (target) | When pattern matching alone is insufficient  |

**Implication for the planned Python NLP validator:** Two detectors hard-depend on NLP (Indonesian names beyond honorific match, and free-text Bahasa diagnoses). Others (ICD-10 escalation, combinatorial severity) benefit from NLP but degrade gracefully. The NLP validator's load-bearing use case is the two hard-dependents; everything else is enhancement.

---

## 7. False-positive mitigation playbook

For organizations integrating HealthWand, the following FP mitigation strategies are recommended:

1. **Always require context for bare-digit detectors** unless the format is highly distinctive (phone numbers with `+62` prefix). Configure `context_words_required: true` in YAML.
2. **Calibrate severity thresholds** for the deployment environment. A research data pipeline can tolerate lower precision; a production CI gate cannot.
3. **Use confidence scores**, not just match/no-match. The YAML `score` field is per-detector; CI gates should set a threshold (e.g., `--min-score 0.85`) that filters out the lowest-confidence matches.
4. **Whitelist known fixtures.** Test data with intentional fake PHI (e.g., `+6281234567890` placeholder) should be allowlisted via path or pattern exclusion, not by lowering global precision.
5. **Tune the context window.** Default ±100 characters; for very long lines (logs, JSON blobs), increase to ±500. For very short lines (CSV row exports), narrow to ±50.
6. **Layer with structural validation** for high-stakes detectors. NIK structural validation (province code in 11–94, valid date encoding) reduces FPs at the cost of some recall.

---

## 8. Out of catalogue — adjacent identifiers

The following Indonesian identifiers are **deliberately not** in the v0.1.0 catalogue. They are documented here so that contributors and users know the scope boundary.

| Identifier                                                            | Type                                     | Reason for exclusion                                                    |
| --------------------------------------------------------------------- | ---------------------------------------- | ----------------------------------------------------------------------- |
| **NITKU** (22-digit business location ID)                             | Business identifier                      | Not personal data — location-of-business identifier                     |
| **BPJS Ketenagakerjaan**                                              | Employment-injury insurance              | Different program from BPJS Kesehatan; not health-PHI                   |
| **SIM** (Surat Izin Mengemudi — driver's license)                     | General personal data                    | Out of health-focused scope; community contribution welcome             |
| **Paspor** (passport number)                                          | General personal data                    | Out of health-focused scope; community contribution welcome             |
| **KK** (Kartu Keluarga family card number)                            | General personal data                    | 16-digit format; would collide with NIK; community contribution welcome |
| **VA** (Virtual Account numbers for payment)                          | Financial transaction                    | Not PHI; out of scope                                                   |
| **NIP** / **NIDN** (employee identification, lecturer identification) | Professional/employment                  | Controller-side, niche; community contribution welcome                  |
| **Biometric data** (fingerprints, iris scans, photo identifiers)      | Specific personal data Art 4(1)          | Specialized tooling required; HealthWand is text-focused                |
| **Genetic data**                                                      | Specific personal data Art 4(1)          | Specialized tooling required                                            |
| **Medical image burned-in metadata**                                  | Specific personal data Art 4(1) — health | Planned v2.0; DICOM-specific tooling exists today (CTP, Posda)          |

Adding any of these as a community-contributed YAML pattern is welcome. The catalogue's design is extensible by intent.

---

## 9. Maintenance and versioning

This catalogue is versioned alongside the repository. Trigger events for an update:

- New Indonesian regulation creating a new identifier class.
- Format change in an existing identifier (e.g., the NPWP-NIK integration in 2024).
- Community contribution of a new pattern that should ship by default.
- Regex precision/recall measurement against a labeled corpus that materially changes a regex.

---

## 10. Change log

- **v0.1.0-draft.1** (2026-05-13) — Initial catalogue. 13 detectors documented (7 specific, 6 general). Cross-pattern collisions, combinatorial rules, detection approaches, FP mitigation playbook, and out-of-catalogue boundary all established. NPWP-NIK integration documented per PER-6/PJ/2024.

---

## Appendix A — Mobile operator prefixes (8XX)

For phone-number false-positive reduction, here is the (non-exhaustive) prefix-to-operator mapping. Operator assignments shift over time due to mergers; treat any `8` + 1- or 2-digit prefix as potentially valid.

| Prefix                                      | Operator (current/historical) |
| ------------------------------------------- | ----------------------------- |
| 811, 812, 813, 821, 822, 823, 851, 852, 853 | Telkomsel                     |
| 814, 815, 816, 855, 856, 857, 858           | Indosat Ooredoo Hutchison     |
| 817, 818, 819, 831, 832, 833, 838, 859      | XL Axiata                     |
| 881, 882, 883, 884, 885, 886, 887, 888, 889 | Smartfren                     |
| 895, 896, 897, 898, 899                     | Three (3)                     |

---

## Appendix B — Bahasa month names

| #   | Full      | 3-letter abbreviation |
| --- | --------- | --------------------- |
| 1   | Januari   | Jan                   |
| 2   | Februari  | Feb                   |
| 3   | Maret     | Mar                   |
| 4   | April     | Apr                   |
| 5   | Mei       | Mei                   |
| 6   | Juni      | Jun                   |
| 7   | Juli      | Jul                   |
| 8   | Agustus   | Agu / Ags             |
| 9   | September | Sep                   |
| 10  | Oktober   | Okt                   |
| 11  | November  | Nov                   |
| 12  | Desember  | Des                   |

---

## Appendix C — Common honorifics quick-reference

| Honorific       | Form     | Gender             | Register                  |
| --------------- | -------- | ------------------ | ------------------------- |
| Pak / Bapak     | Pre-name | Male               | Informal/formal           |
| Bu / Ibu        | Pre-name | Female             | Informal/formal           |
| Mas             | Pre-name | Male               | Informal                  |
| Mbak            | Pre-name | Female             | Informal                  |
| Sdr. / Saudara  | Pre-name | Male               | Semi-formal               |
| Sdri. / Saudari | Pre-name | Female             | Semi-formal               |
| Tn. / Tuan      | Pre-name | Male               | Formal                    |
| Ny. / Nyonya    | Pre-name | Female (married)   | Formal                    |
| Nn. / Nona      | Pre-name | Female (unmarried) | Formal                    |
| dr. (lowercase) | Pre-name | Any                | Medical doctor (clinical) |
| Dr. (uppercase) | Pre-name | Any                | Doctoral (academic)       |
| Prof.           | Pre-name | Any                | Professor (academic)      |
| H.              | Pre-name | Male               | Hajj-completed            |
| Hj.             | Pre-name | Female             | Hajj-completed            |

---

## Appendix D — NIK province codes (selected)

NIK positions 1–2 encode the province per Kemendagri allocation. Selected codes for FP-reduction validation:

| Code | Province            |
| ---- | ------------------- |
| 11   | Aceh                |
| 12   | Sumatera Utara      |
| 13   | Sumatera Barat      |
| 14   | Riau                |
| 15   | Jambi               |
| 16   | Sumatera Selatan    |
| 17   | Bengkulu            |
| 18   | Lampung             |
| 19   | Bangka Belitung     |
| 21   | Kepulauan Riau      |
| 31   | DKI Jakarta         |
| 32   | Jawa Barat          |
| 33   | Jawa Tengah         |
| 34   | DI Yogyakarta       |
| 35   | Jawa Timur          |
| 36   | Banten              |
| 51   | Bali                |
| 52   | Nusa Tenggara Barat |
| 53   | Nusa Tenggara Timur |
| 61   | Kalimantan Barat    |
| 62   | Kalimantan Tengah   |
| 63   | Kalimantan Selatan  |
| 64   | Kalimantan Timur    |
| 65   | Kalimantan Utara    |
| 71   | Sulawesi Utara      |
| 72   | Sulawesi Tengah     |
| 73   | Sulawesi Selatan    |
| 74   | Sulawesi Tenggara   |
| 75   | Gorontalo           |
| 76   | Sulawesi Barat      |
| 81   | Maluku              |
| 82   | Maluku Utara        |
| 91   | Papua Barat         |
| 94   | Papua               |

The province code range 11–94 is the FP-reduction validation window. Any 16-digit number whose first two digits fall outside this range is highly likely to be a false positive for NIK. The DOB-allocation province codes have been further subdivided since the 2022 Papua expansion (Papua Selatan, Papua Tengah, Papua Pegunungan); new codes 95–96 may apply. Community contribution welcome to keep this list current.
