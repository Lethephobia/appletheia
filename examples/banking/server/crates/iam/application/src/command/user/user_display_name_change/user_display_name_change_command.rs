use appletheia::command;
use banking_iam_domain::{UserDisplayName, UserId};
use serde::{Deserialize, Serialize};

/// Changes a user's display name.
#[command(name = "user_display_name_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDisplayNameChangeCommand {
    pub user_id: UserId,
    pub display_name: UserDisplayName,
}
