use serde::{Deserialize, Serialize};

/// Represents the OIDC `middle_name` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcMiddleName(String);

impl OidcMiddleName {
    /// Creates an OIDC middle-name claim value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &str {
        &self.0
    }
}
