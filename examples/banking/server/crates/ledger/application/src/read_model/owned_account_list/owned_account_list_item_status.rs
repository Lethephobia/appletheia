use banking_ledger_domain::account::AccountStatus;

use super::OwnedAccountListItemStatusError;

/// Status shown in an owned account list item.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OwnedAccountListItemStatus {
    Active,
    Frozen,
}

impl TryFrom<AccountStatus> for OwnedAccountListItemStatus {
    type Error = OwnedAccountListItemStatusError;

    fn try_from(status: AccountStatus) -> Result<Self, Self::Error> {
        match status {
            AccountStatus::Active => Ok(Self::Active),
            AccountStatus::Frozen => Ok(Self::Frozen),
            AccountStatus::Closed => Err(OwnedAccountListItemStatusError::Unsupported(status)),
        }
    }
}
