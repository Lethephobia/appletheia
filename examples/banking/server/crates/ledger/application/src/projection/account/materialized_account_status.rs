use banking_ledger_domain::account::AccountStatus;
use serde::{Deserialize, Serialize};

use super::MaterializedAccountStatusError;

/// Status materialized by an account fragment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterializedAccountStatus {
    Active,
    Frozen,
}

impl TryFrom<AccountStatus> for MaterializedAccountStatus {
    type Error = MaterializedAccountStatusError;

    fn try_from(status: AccountStatus) -> Result<Self, Self::Error> {
        match status {
            AccountStatus::Active => Ok(Self::Active),
            AccountStatus::Frozen => Ok(Self::Frozen),
            AccountStatus::Closed => Err(MaterializedAccountStatusError::Unsupported(status)),
        }
    }
}
