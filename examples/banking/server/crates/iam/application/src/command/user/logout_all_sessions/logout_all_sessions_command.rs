use appletheia::application::authentication::AuthTokenIssuedAt;
use appletheia::command;
use serde::{Deserialize, Serialize};

/// Revokes all active sessions for the authenticated subject.
#[command(name = "logout_all_sessions")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogoutAllSessionsCommand {
    pub token_issued_at: AuthTokenIssuedAt,
}
