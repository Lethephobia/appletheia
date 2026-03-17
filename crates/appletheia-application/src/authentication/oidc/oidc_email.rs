use serde::{Deserialize, Serialize};

/// Represents the OIDC `email` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcEmail(String);

impl OidcEmail {
    /// Creates an OIDC email claim value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &str {
        &self.0
    }
}
