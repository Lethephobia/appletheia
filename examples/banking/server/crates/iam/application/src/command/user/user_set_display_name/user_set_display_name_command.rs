use appletheia::command;
use banking_iam_domain::{UserDisplayName, UserId};
use serde::{Deserialize, Serialize};

/// Changes a user's display name.
#[command(name = "user_set_display_name")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSetDisplayNameCommand {
    pub user_id: UserId,
    pub display_name: UserDisplayName,
}
