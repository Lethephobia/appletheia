use appletheia::command;
use banking_iam_domain::{UserId, UserProfile};
use serde::{Deserialize, Serialize};

/// Replaces a user's profile.
#[command(name = "user_profile_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileChangeCommand {
    pub user_id: UserId,
    pub profile: UserProfile,
}
