use banking_ledger_domain::account::AccountWithdrawRejectionReason;
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
