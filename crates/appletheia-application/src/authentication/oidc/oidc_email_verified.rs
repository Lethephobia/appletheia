use serde::{Deserialize, Serialize};

/// Represents the OIDC `email_verified` standard claim.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcEmailVerified(bool);

impl OidcEmailVerified {
    /// Creates an OIDC email-verified claim value.
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> bool {
        self.0
    }
}
