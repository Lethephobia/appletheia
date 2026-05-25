use serde::{Deserialize, Serialize};

use super::CurrencyMintAccount;

/// Represents whether a `Currency` has completed on-chain provisioning.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyProvisioningStatus {
    Pending,
    Provisioned { mint_account: CurrencyMintAccount },
    Failed,
}

impl CurrencyProvisioningStatus {
    /// Returns whether the currency is still provisioning.
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns whether the currency has completed provisioning.
    pub fn is_provisioned(&self) -> bool {
        matches!(self, Self::Provisioned { .. })
    }

    /// Returns whether the currency provisioning has failed.
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns the provisioned mint account when present.
    pub fn mint_account(&self) -> Option<&CurrencyMintAccount> {
        match self {
            Self::Provisioned { mint_account } => Some(mint_account),
            Self::Pending | Self::Failed => None,
        }
    }
}
