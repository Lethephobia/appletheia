use banking_ledger_domain::currency::CurrencyStatus;

use super::CurrencyListItemStatusError;

/// Status shown in a currency list item.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CurrencyListItemStatus {
    Active,
    Inactive,
}

impl TryFrom<CurrencyStatus> for CurrencyListItemStatus {
    type Error = CurrencyListItemStatusError;

    fn try_from(status: CurrencyStatus) -> Result<Self, Self::Error> {
        match status {
            CurrencyStatus::Active => Ok(Self::Active),
            CurrencyStatus::Inactive => Ok(Self::Inactive),
            CurrencyStatus::Removed => Err(CurrencyListItemStatusError::Unsupported(status)),
        }
    }
}
