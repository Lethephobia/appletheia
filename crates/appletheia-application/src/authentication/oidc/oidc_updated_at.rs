use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the OIDC `updated_at` standard claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcUpdatedAt(DateTime<Utc>);

impl OidcUpdatedAt {
    /// Creates an OIDC updated-at claim value.
    pub fn new(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}
