use serde::{Deserialize, Serialize};

use super::{OidcAuthorizationUrl, OidcLoginAttemptExpiresAt};

/// The result returned when an OIDC login flow begins.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcBeginResult {
    pub authorization_url: OidcAuthorizationUrl,
    pub expires_at: OidcLoginAttemptExpiresAt,
}
