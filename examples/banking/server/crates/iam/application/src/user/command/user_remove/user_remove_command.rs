use appletheia::command;
use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

/// Removes the specified user.
#[command(name = "user_remove")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRemoveCommand {
    pub user_id: UserId,
}
