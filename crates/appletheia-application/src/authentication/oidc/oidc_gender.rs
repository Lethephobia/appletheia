use serde::{Deserialize, Serialize};

/// Represents the OIDC `gender` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcGender(String);

impl OidcGender {
    /// Creates an OIDC gender claim value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &str {
        &self.0
    }
}
