use serde::{Deserialize, Serialize};

/// Represents the OIDC `family_name` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcFamilyName(String);

impl OidcFamilyName {
    /// Creates an OIDC family-name claim value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &str {
        &self.0
    }
}
