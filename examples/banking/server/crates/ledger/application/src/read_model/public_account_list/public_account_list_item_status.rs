use banking_ledger_domain::account::AccountStatus;

use super::PublicAccountListItemStatusError;

/// Account status tracked by public account list item projections.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PublicAccountListItemStatus {
    Active,
    Frozen,
}

impl TryFrom<AccountStatus> for PublicAccountListItemStatus {
    type Error = PublicAccountListItemStatusError;

    fn try_from(status: AccountStatus) -> Result<Self, Self::Error> {
        match status {
            AccountStatus::Active => Ok(Self::Active),
            AccountStatus::Frozen => Ok(Self::Frozen),
            AccountStatus::Closed => Err(PublicAccountListItemStatusError::Unsupported(status)),
        }
    }
}
