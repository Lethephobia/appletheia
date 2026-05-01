use banking_ledger_domain::account::{
    AccountReservedFundsCommitRejectionReason, AccountReservedFundsCommitResult,
};
use serde::{Deserialize, Serialize};

/// Returned after committing reserved funds in an account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountReservedFundsCommitOutput {
    Committed,
    Rejected {
        reason: AccountReservedFundsCommitRejectionReason,
    },
}

impl From<AccountReservedFundsCommitResult> for AccountReservedFundsCommitOutput {
    fn from(value: AccountReservedFundsCommitResult) -> Self {
        match value {
            AccountReservedFundsCommitResult::Committed => Self::Committed,
            AccountReservedFundsCommitResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
