use serde::{Deserialize, Serialize};
use url::Url;

/// Represents the OIDC `userinfo_endpoint` URL.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcUserInfoEndpointUrl(Url);

impl OidcUserInfoEndpointUrl {
    /// Creates a new user-info endpoint URL.
    pub fn new(value: Url) -> Self {
        Self(value)
    }

    /// Returns the endpoint URL.
    pub fn value(&self) -> &Url {
        &self.0
    }
}
