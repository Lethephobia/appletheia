use appletheia::command;
use banking_iam_domain::{UserId, Username};
use serde::{Deserialize, Serialize};

/// Changes a user's username.
#[command(name = "user_username_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserUsernameChangeCommand {
    pub user_id: UserId,
    pub username: Username,
}
