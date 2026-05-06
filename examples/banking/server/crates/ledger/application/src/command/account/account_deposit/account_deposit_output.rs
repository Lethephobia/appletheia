use banking_ledger_domain::account::{AccountDepositRejectionReason, AccountDepositResult};
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

impl From<AccountDepositResult> for AccountDepositOutput {
    fn from(value: AccountDepositResult) -> Self {
        match value {
            AccountDepositResult::Deposited => Self::Deposited,
            AccountDepositResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
