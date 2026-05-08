use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosureCompleteRejectionReason, OwnedAccountClosureCompleteResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an owned account closure complete request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosureCompleteOutput {
    Completed,
    Rejected {
        reason: OwnedAccountClosureCompleteRejectionReason,
    },
}

impl From<OwnedAccountClosureCompleteResult> for OwnedAccountClosureCompleteOutput {
    fn from(value: OwnedAccountClosureCompleteResult) -> Self {
        match value {
            OwnedAccountClosureCompleteResult::Completed => Self::Completed,
            OwnedAccountClosureCompleteResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
