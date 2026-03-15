use serde::{Deserialize, Serialize};

/// Represents a PKCE code challenge derived from a verifier.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PkceCodeChallenge(String);

impl PkceCodeChallenge {
    /// Creates a new challenge from a precomputed value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the raw challenge string.
    pub fn value(&self) -> &str {
        &self.0
    }
}
