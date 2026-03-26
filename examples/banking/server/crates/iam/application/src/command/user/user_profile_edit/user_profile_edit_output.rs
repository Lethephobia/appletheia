use banking_iam_domain::{UserDisplayName, UserId, Username};
use serde::{Deserialize, Serialize};

/// The output returned after editing a user's profile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileEditOutput {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
}

impl UserProfileEditOutput {
    /// Creates a new user-profile-edit output.
    pub fn new(
        user_id: UserId,
        username: Option<Username>,
        display_name: Option<UserDisplayName>,
    ) -> Self {
        Self {
            user_id,
            username,
            display_name,
        }
    }
}
