use appletheia::application::authentication::{AuthTokenExpiresAt, AuthTokenId};
use appletheia::application::command::{Command, CommandName};
use serde::{Deserialize, Serialize};

/// Revokes a single access token.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogoutCommand {
    pub token_id: AuthTokenId,
    pub token_expires_at: AuthTokenExpiresAt,
}

impl Command for LogoutCommand {
    const NAME: CommandName = CommandName::new("logout");
}
