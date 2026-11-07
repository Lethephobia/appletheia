use appletheia::command;
use banking_iam_domain::{UserId, Username};
use serde::{Deserialize, Serialize};

/// Changes a user's username.
#[command(name = "user_set_username")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSetUsernameCommand {
    pub user_id: UserId,
    pub username: Username,
}
