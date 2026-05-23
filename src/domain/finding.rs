use crate::domain::{MatchSpan, PatternId, Score, Severity};

/// A single PHI finding: detected pattern + location + metadata.
///
/// **CRITICAL M2 CHANGE**: Renamed from `Detection` to `Finding`.
/// The key architectural change is that `redaction_template` and `redaction_strategy`
/// are now fields on Finding, so the Redactor doesn't need to look them up from PatternId.
#[derive(Debug, Clone)]
pub struct Finding {
    pub pattern_id: PatternId,
    pub span: MatchSpan,
    pub matched_text: String,
    pub severity: Severity,
    pub score: Score,
    pub context: Option<String>,
    pub context_matched: bool,
    pub uu_pdp_article: Option<UuPdpArticle>,
    pub redaction_template: Option<String>,
    pub redaction_strategy: Option<String>,
}

/// UU PDP article classification (Indonesian regulatory).
///
/// Stub for M2: all findings get `None`.
/// Full implementation in M3 when pattern catalogue is complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UuPdpArticle {
    // TODO: Add actual article variants in M3
    Article1,
    // ... more articles
}
