use serde::{Deserialize, Serialize};

use super::{OidcAuthorizationUrl, OidcLoginAttemptExpiresAt, OidcState};

/// The result returned when an OIDC login flow begins.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcBeginResult {
    pub authorization_url: OidcAuthorizationUrl,
    pub state: OidcState,
    pub expires_at: OidcLoginAttemptExpiresAt,
}
