use banking_ledger_domain::currency::{
    CurrencyMintAccount, CurrencyMintAccountRecordRejectionReason,
};
use serde::{Deserialize, Serialize};

/// Returned after attempting to create a currency mint account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyMintAccountCreateOutput {
    Created {
        mint_account: CurrencyMintAccount,
    },
    Rejected {
        reason: CurrencyMintAccountRecordRejectionReason,
    },
}
