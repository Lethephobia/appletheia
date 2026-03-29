use banking_iam_domain::{UserDisplayName, UserId, Username};
use serde::{Deserialize, Serialize};

/// The output returned after readying a user's profile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileReadyOutput {
    pub user_id: UserId,
    pub username: Username,
    pub display_name: UserDisplayName,
}

impl UserProfileReadyOutput {
    /// Creates a new user-profile-ready output.
    pub fn new(user_id: UserId, username: Username, display_name: UserDisplayName) -> Self {
        Self {
            user_id,
            username,
            display_name,
        }
    }
}
