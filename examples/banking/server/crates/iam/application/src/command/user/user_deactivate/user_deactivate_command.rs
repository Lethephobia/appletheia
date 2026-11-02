use appletheia::command;
use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

/// Deactivates the specified user.
#[command(name = "user_deactivate")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDeactivateCommand {
    pub user_id: UserId,
}
