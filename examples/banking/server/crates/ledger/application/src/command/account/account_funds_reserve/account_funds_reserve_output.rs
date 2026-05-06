use banking_ledger_domain::account::{
    AccountFundsReserveRejectionReason, AccountFundsReserveResult,
};
use serde::{Deserialize, Serialize};

/// Returned after reserving funds in an account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountFundsReserveOutput {
    Reserved,
    Rejected {
        reason: AccountFundsReserveRejectionReason,
    },
}

impl From<AccountFundsReserveResult> for AccountFundsReserveOutput {
    fn from(value: AccountFundsReserveResult) -> Self {
        match value {
            AccountFundsReserveResult::Reserved => Self::Reserved,
            AccountFundsReserveResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
