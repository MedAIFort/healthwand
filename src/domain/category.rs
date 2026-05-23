use serde::{Deserialize, Serialize};

/// Data classification category.
///
/// M2 scope: does NOT implement the full UU PDP split
/// (GeneralPersonalData / SpecificPersonalData).
/// That is M3 work when the full pattern catalogue is implemented.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Identifier,
    Medical,
    Personal,
    Insurance,
    Other(String),
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Identifier => write!(f, "identifier"),
            Category::Medical => write!(f, "medical"),
            Category::Personal => write!(f, "personal"),
            Category::Insurance => write!(f, "insurance"),
            Category::Other(s) => write!(f, "{}", s),
        }
    }
}
