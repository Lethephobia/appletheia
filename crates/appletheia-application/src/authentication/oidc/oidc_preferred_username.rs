use serde::{Deserialize, Serialize};

/// Represents the OIDC `preferred_username` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcPreferredUsername(String);

impl OidcPreferredUsername {
    /// Creates an OIDC preferred-username claim value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &str {
        &self.0
    }
}
