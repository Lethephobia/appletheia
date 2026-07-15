use banking_iam_domain::UserStatus;

use super::UserPrivateInfoStatusError;

/// Lifecycle status visible in user-private information.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum UserPrivateInfoStatus {
    Active,
    Inactive,
}

impl TryFrom<UserStatus> for UserPrivateInfoStatus {
    type Error = UserPrivateInfoStatusError;

    fn try_from(status: UserStatus) -> Result<Self, Self::Error> {
        match status {
            UserStatus::Active => Ok(Self::Active),
            UserStatus::Inactive => Ok(Self::Inactive),
            UserStatus::Removed => Err(UserPrivateInfoStatusError::Unsupported(status)),
        }
    }
}
