use banking_ledger_domain::account::AccountCloseRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after an account close request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountCloseOutput {
    Closed,
    Rejected { reason: AccountCloseRejectionReason },
}
