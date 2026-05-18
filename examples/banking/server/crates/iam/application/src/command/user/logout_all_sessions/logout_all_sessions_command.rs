use appletheia::application::authentication::AuthTokenIssuedAt;
use appletheia::command;
use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

/// Revokes all active sessions for a user.
#[command(name = "logout_all_sessions")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogoutAllSessionsCommand {
    pub user_id: UserId,
    pub token_issued_at: AuthTokenIssuedAt,
}
