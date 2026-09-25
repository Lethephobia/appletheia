use serde::Serialize;

use banking_iam_domain::UserStatus;

use super::UserPublicProfileStatusError;

/// Lifecycle status tracked by public user profile projections.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum UserPublicProfileStatus {
    Active,
    Inactive,
}

impl TryFrom<UserStatus> for UserPublicProfileStatus {
    type Error = UserPublicProfileStatusError;

    fn try_from(status: UserStatus) -> Result<Self, Self::Error> {
        match status {
            UserStatus::Active => Ok(Self::Active),
            UserStatus::Inactive => Ok(Self::Inactive),
            UserStatus::Removed => Err(UserPublicProfileStatusError::Unsupported(status)),
        }
    }
}
