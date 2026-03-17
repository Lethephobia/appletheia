use serde::{Deserialize, Serialize};

/// Represents the OIDC `phone_number_verified` standard claim.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcPhoneNumberVerified(bool);

impl OidcPhoneNumberVerified {
    /// Creates an OIDC phone-number-verified claim value.
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> bool {
        self.0
    }
}
