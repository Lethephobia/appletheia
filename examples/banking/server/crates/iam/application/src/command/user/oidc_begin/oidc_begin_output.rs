use serde::{Deserialize, Serialize};

use appletheia::application::authentication::oidc::{
    OidcAuthorizationUrl, OidcContinuationExpiresAt,
};

/// The output returned after an OIDC authorization flow begins.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcBeginOutput {
    pub authorization_url: OidcAuthorizationUrl,
    pub expires_at: OidcContinuationExpiresAt,
}

impl OidcBeginOutput {
    pub fn new(
        authorization_url: OidcAuthorizationUrl,
        expires_at: OidcContinuationExpiresAt,
    ) -> Self {
        Self {
            authorization_url,
            expires_at,
        }
    }
}
