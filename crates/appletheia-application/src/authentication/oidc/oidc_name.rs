use serde::{Deserialize, Serialize};

/// Represents the OIDC `name` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcName(String);

impl OidcName {
    /// Creates an OIDC name claim value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &str {
        &self.0
    }
}
