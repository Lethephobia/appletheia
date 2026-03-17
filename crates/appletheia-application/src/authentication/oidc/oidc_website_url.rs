use serde::{Deserialize, Serialize};
use url::Url;

/// Represents the OIDC `website` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcWebsiteUrl(Url);

impl OidcWebsiteUrl {
    /// Creates an OIDC website URL claim value.
    pub fn new(value: Url) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &Url {
        &self.0
    }
}
