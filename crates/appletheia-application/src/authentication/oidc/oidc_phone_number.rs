use serde::{Deserialize, Serialize};

/// Represents the OIDC `phone_number` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcPhoneNumber(String);

impl OidcPhoneNumber {
    /// Creates an OIDC phone-number claim value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &str {
        &self.0
    }
}
