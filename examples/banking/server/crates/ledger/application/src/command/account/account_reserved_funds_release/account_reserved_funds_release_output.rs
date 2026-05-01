use banking_ledger_domain::account::{
    AccountReservedFundsReleaseRejectionReason, AccountReservedFundsReleaseResult,
};
use serde::{Deserialize, Serialize};

/// Returned after releasing reserved funds in an account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountReservedFundsReleaseOutput {
    Released,
    Rejected {
        reason: AccountReservedFundsReleaseRejectionReason,
    },
}

impl From<AccountReservedFundsReleaseResult> for AccountReservedFundsReleaseOutput {
    fn from(value: AccountReservedFundsReleaseResult) -> Self {
        match value {
            AccountReservedFundsReleaseResult::Released => Self::Released,
            AccountReservedFundsReleaseResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
