use appletheia::application::authentication::oidc::{OidcAuthorizationCode, OidcState};
use appletheia::application::command::{Command, CommandName};
use serde::{Deserialize, Serialize};

/// Completes an OIDC authorization flow for a user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcCompleteCommand {
    pub state: OidcState,
    pub authorization_code: OidcAuthorizationCode,
}

impl Command for OidcCompleteCommand {
    const NAME: CommandName = CommandName::new("oidc_complete");
}
