use banking_ledger_domain::account::{AccountThawRejectionReason, AccountThawResult};
use serde::{Deserialize, Serialize};

/// Returned after an account thaw request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountThawOutput {
    Thawed,
    Rejected { reason: AccountThawRejectionReason },
}

impl From<AccountThawResult> for AccountThawOutput {
    fn from(value: AccountThawResult) -> Self {
        match value {
            AccountThawResult::Thawed => Self::Thawed,
            AccountThawResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
