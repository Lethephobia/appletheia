use serde::Serialize;

use banking_ledger_domain::currency::CurrencyStatus;

use super::CurrencyListItemStatusError;

/// Status shown in a currency list item.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum CurrencyListItemStatus {
    Provisioning,
    Active,
    Inactive,
    ProvisioningFailed,
}

impl TryFrom<CurrencyStatus> for CurrencyListItemStatus {
    type Error = CurrencyListItemStatusError;

    fn try_from(status: CurrencyStatus) -> Result<Self, Self::Error> {
        match status {
            CurrencyStatus::Provisioning => Ok(Self::Provisioning),
            CurrencyStatus::Active => Ok(Self::Active),
            CurrencyStatus::Inactive => Ok(Self::Inactive),
            CurrencyStatus::ProvisioningFailed => Ok(Self::ProvisioningFailed),
            CurrencyStatus::Removed => Err(CurrencyListItemStatusError::Unsupported(status)),
        }
    }
}
