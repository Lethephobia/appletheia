use banking_iam_domain::UserStatus;
use serde::{Deserialize, Serialize};

use super::MaterializedUserStatusError;

/// Lifecycle status materialized by the public user fragment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterializedUserStatus {
    Active,
    Inactive,
}

impl TryFrom<UserStatus> for MaterializedUserStatus {
    type Error = MaterializedUserStatusError;

    fn try_from(status: UserStatus) -> Result<Self, Self::Error> {
        match status {
            UserStatus::Active => Ok(Self::Active),
            UserStatus::Inactive => Ok(Self::Inactive),
            UserStatus::Removed => Err(MaterializedUserStatusError::Unsupported(status)),
        }
    }
}
