use serde::{Deserialize, Serialize};

/// Represents the OIDC `nickname` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcNickname(String);

impl OidcNickname {
    /// Creates an OIDC nickname claim value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &str {
        &self.0
    }
}
