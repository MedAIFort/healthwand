/// Confidence score for a detection, normalized to 0.0..=1.0.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Score(f32);

impl Score {
    /// Create a new Score, validating that the value is in [0.0, 1.0].
    pub fn new(value: f32) -> crate::error::Result<Self> {
        if (0.0..=1.0).contains(&value) {
            Ok(Score(value))
        } else {
            Err(crate::error::HealthwandError::ConfigError(format!(
                "Score must be in [0.0, 1.0], got {}",
                value
            )))
        }
    }

    /// Get the underlying f32 value.
    pub fn value(self) -> f32 {
        self.0
    }
}

// Manually implement Eq for Score.
// Score wraps f32 in range [0.0, 1.0] with validation at construction.
// No NaN can exist, so total equality is safe.
impl Eq for Score {}
