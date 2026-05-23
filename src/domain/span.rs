/// Location of a match within source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MatchSpan {
    /// Byte offset (0-based) of match start
    pub start: usize,
    /// Byte offset (0-based) of match end
    pub end: usize,
    /// Line number (1-based)
    pub line: u32,
    /// Column number (1-based)
    pub column: u32,
}

impl MatchSpan {
    /// Create a MatchSpan from byte offsets, computing line/column from text.
    ///
    /// For M2, this is a placeholder implementation that only tracks offsets.
    /// Full line/column tracking will be implemented in a later milestone.
    pub fn from_offsets(_text: &str, start: usize, end: usize) -> Self {
        // TODO: Compute line and column by counting newlines up to `start`
        // For now, use placeholder values
        MatchSpan {
            start,
            end,
            line: 1,
            column: start as u32 + 1,
        }
    }
}
