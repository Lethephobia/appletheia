use appletheia::application::authentication::AuthTokenIssuedAt;
use appletheia::application::command::{Command, CommandName};
use serde::{Deserialize, Serialize};

/// Revokes all active sessions for the authenticated subject.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogoutAllSessionsCommand {
    pub token_issued_at: AuthTokenIssuedAt,
}

impl Command for LogoutAllSessionsCommand {
    const NAME: CommandName = CommandName::new("logout_all_sessions");
}
