use appletheia::application::authentication::{AuthTokenExpiresAt, AuthTokenId};
use appletheia::command;
use serde::{Deserialize, Serialize};

/// Revokes a single access token.
#[command(name = "logout")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogoutCommand {
    pub token_id: AuthTokenId,
    pub token_expires_at: AuthTokenExpiresAt,
}
