use banking_ledger_domain::currency::CurrencyStatus;
use serde::{Deserialize, Serialize};

use super::MaterializedCurrencyStatusError;

/// Status materialized by a currency fragment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterializedCurrencyStatus {
    Provisioning,
    Active,
    Inactive,
    ProvisioningFailed,
}

impl TryFrom<CurrencyStatus> for MaterializedCurrencyStatus {
    type Error = MaterializedCurrencyStatusError;

    fn try_from(status: CurrencyStatus) -> Result<Self, Self::Error> {
        match status {
            CurrencyStatus::Provisioning => Ok(Self::Provisioning),
            CurrencyStatus::Active => Ok(Self::Active),
            CurrencyStatus::Inactive => Ok(Self::Inactive),
            CurrencyStatus::ProvisioningFailed => Ok(Self::ProvisioningFailed),
            CurrencyStatus::Removed => Err(MaterializedCurrencyStatusError::Unsupported(status)),
        }
    }
}
