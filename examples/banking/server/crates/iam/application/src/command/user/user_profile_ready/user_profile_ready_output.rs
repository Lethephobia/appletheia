use banking_iam_domain::{UserBio, UserDisplayName, UserId, Username};
use serde::{Deserialize, Serialize};

/// The output returned after readying a user's profile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileReadyOutput {
    pub user_id: UserId,
    pub username: Username,
    pub display_name: UserDisplayName,
    pub bio: Option<UserBio>,
}

impl UserProfileReadyOutput {
    /// Creates a new user-profile-ready output.
    pub fn new(
        user_id: UserId,
        username: Username,
        display_name: UserDisplayName,
        bio: Option<UserBio>,
    ) -> Self {
        Self {
            user_id,
            username,
            display_name,
            bio,
        }
    }
}
