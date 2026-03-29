use appletheia::command;
use banking_iam_domain::{UserDisplayName, UserId, Username};
use serde::{Deserialize, Serialize};

/// Marks a user's profile as ready.
#[command(name = "user_profile_ready")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileReadyCommand {
    pub user_id: UserId,
    pub username: Username,
    pub display_name: UserDisplayName,
}
