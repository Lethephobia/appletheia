use banking_ledger_domain::account::{AccountNameChangeRejectionReason, AccountNameChangeResult};
use serde::{Deserialize, Serialize};

/// The output returned after changing an account name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountNameChangeOutput {
    Changed,
    Rejected {
        reason: AccountNameChangeRejectionReason,
    },
}

impl From<AccountNameChangeResult> for AccountNameChangeOutput {
    fn from(value: AccountNameChangeResult) -> Self {
        match value {
            AccountNameChangeResult::Changed => Self::Changed,
            AccountNameChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
