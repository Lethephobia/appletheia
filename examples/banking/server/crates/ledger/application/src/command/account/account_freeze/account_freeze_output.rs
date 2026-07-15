use banking_ledger_domain::account::AccountFreezeRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after an account freeze request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountFreezeOutput {
    Frozen,
    Rejected {
        reason: AccountFreezeRejectionReason,
    },
}
