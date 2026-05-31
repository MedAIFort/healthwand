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
    /// Create a MatchSpan from byte offsets, validating bounds and computing line/column.
    ///
    /// Validates that `start <= end` and both offsets are within text bounds.
    /// For M2, line/column computation is a placeholder (always line 1, column is byte offset + 1).
    /// Full line/column tracking will be implemented in a later milestone.
    pub fn from_offsets(text: &str, start: usize, end: usize) -> crate::error::Result<Self> {
        if start > end {
            return Err(crate::error::HealthwandError::ConfigError(format!(
                "MatchSpan: start ({}) must be <= end ({})",
                start, end
            )));
        }
        if end > text.len() {
            return Err(crate::error::HealthwandError::ConfigError(format!(
                "MatchSpan: end ({}) exceeds text length ({})",
                end,
                text.len()
            )));
        }
        Ok(MatchSpan {
            start,
            end,
            line: 1,
            column: start as u32 + 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_offsets_rejects_inverted_range() {
        let text = "hello world";
        let result = MatchSpan::from_offsets(text, 7, 3);
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::error::HealthwandError::ConfigError(msg) => {
                assert!(msg.contains("start (7) must be <= end (3)"));
            }
            _ => panic!("Expected ConfigError"),
        }
    }

    #[test]
    fn test_from_offsets_rejects_end_out_of_bounds() {
        let text = "hello";
        let result = MatchSpan::from_offsets(text, 1, 10);
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::error::HealthwandError::ConfigError(msg) => {
                assert!(msg.contains("end (10) exceeds text length (5)"));
            }
            _ => panic!("Expected ConfigError"),
        }
    }

    #[test]
    fn test_from_offsets_accepts_valid_range() {
        let text = "hello world";
        let result = MatchSpan::from_offsets(text, 0, 5);
        assert!(result.is_ok());
        let span = result.unwrap();
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 5);
        assert_eq!(span.line, 1);
        assert_eq!(span.column, 1);
    }

    #[test]
    fn test_from_offsets_accepts_equal_start_end() {
        let text = "test";
        let result = MatchSpan::from_offsets(text, 2, 2);
        assert!(result.is_ok());
        let span = result.unwrap();
        assert_eq!(span.start, 2);
        assert_eq!(span.end, 2);
    }

    #[test]
    fn test_from_offsets_accepts_end_at_text_boundary() {
        let text = "abc";
        let result = MatchSpan::from_offsets(text, 0, 3);
        assert!(result.is_ok());
        let span = result.unwrap();
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 3);
    }
}
