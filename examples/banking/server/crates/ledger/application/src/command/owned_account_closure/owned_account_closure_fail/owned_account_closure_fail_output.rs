use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosureFailRejectionReason, OwnedAccountClosureFailResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an owned account closure fail request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosureFailOutput {
    Failed,
    Rejected {
        reason: OwnedAccountClosureFailRejectionReason,
    },
}

impl From<OwnedAccountClosureFailResult> for OwnedAccountClosureFailOutput {
    fn from(value: OwnedAccountClosureFailResult) -> Self {
        match value {
            OwnedAccountClosureFailResult::Failed => Self::Failed,
            OwnedAccountClosureFailResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
