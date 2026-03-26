use appletheia::command;
use banking_iam_domain::{UserDisplayName, UserId, Username};
use serde::{Deserialize, Serialize};

/// Applies a partial update to a user's profile.
#[command(name = "user_profile_edit")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileEditCommand {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
}
