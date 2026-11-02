use appletheia::command;
use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

/// Deletes the specified user.
#[command(name = "user_delete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDeleteCommand {
    pub user_id: UserId,
}
