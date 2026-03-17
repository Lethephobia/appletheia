use serde::{Deserialize, Serialize};
use url::Url;

/// Represents the OIDC `profile` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcProfileUrl(Url);

impl OidcProfileUrl {
    /// Creates an OIDC profile URL claim value.
    pub fn new(value: Url) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &Url {
        &self.0
    }
}
