use appletheia::command;
use banking_iam_domain::{UserId, Username};
use serde::{Deserialize, Serialize};

/// Changes a user's username.
#[command(name = "user_username_set")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserUsernameSetCommand {
    pub user_id: UserId,
    pub username: Username,
}
