use appletheia::application::authentication::oidc::{OidcAuthorizationCode, OidcState};
use appletheia::command;
use serde::{Deserialize, Serialize};

/// Completes an OIDC authorization flow for a user.
#[command(name = "oidc_complete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcCompleteCommand {
    pub state: OidcState,
    pub authorization_code: OidcAuthorizationCode,
}
