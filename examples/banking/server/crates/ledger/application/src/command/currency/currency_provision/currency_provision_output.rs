use banking_ledger_domain::currency::{CurrencyMintAccount, CurrencyProvisionRejectionReason};
use serde::{Deserialize, Serialize};

/// Returned after attempting to provision a currency.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyProvisionOutput {
    Provisioned {
        mint_account: CurrencyMintAccount,
    },
    Rejected {
        reason: CurrencyProvisionRejectionReason,
    },
}
