use banking_ledger_domain::account::{AccountWithdrawRejectionReason, AccountWithdrawResult};
use serde::{Deserialize, Serialize};

/// Returned after an account withdraw request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountWithdrawOutput {
    Withdrawn,
    Rejected {
        reason: AccountWithdrawRejectionReason,
    },
}

impl From<AccountWithdrawResult> for AccountWithdrawOutput {
    fn from(value: AccountWithdrawResult) -> Self {
        match value {
            AccountWithdrawResult::Withdrawn => Self::Withdrawn,
            AccountWithdrawResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
