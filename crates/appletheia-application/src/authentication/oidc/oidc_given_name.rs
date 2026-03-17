use serde::{Deserialize, Serialize};

/// Represents the OIDC `given_name` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcGivenName(String);

impl OidcGivenName {
    /// Creates an OIDC given-name claim value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &str {
        &self.0
    }
}
