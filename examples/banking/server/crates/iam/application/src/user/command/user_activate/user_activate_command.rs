use appletheia::command;
use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

/// Activates the specified user.
#[command(name = "user_activate")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserActivateCommand {
    pub user_id: UserId,
}
