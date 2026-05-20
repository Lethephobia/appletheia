use banking_ledger_domain::account::AccountDepositRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after an account deposit request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountDepositOutput {
    Deposited,
    Rejected {
        reason: AccountDepositRejectionReason,
    },
}
