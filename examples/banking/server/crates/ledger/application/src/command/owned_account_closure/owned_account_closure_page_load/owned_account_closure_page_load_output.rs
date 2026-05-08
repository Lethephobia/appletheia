use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosurePageLoadRejectionReason, OwnedAccountClosurePageLoadResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an owned account closure page load is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosurePageLoadOutput {
    Loaded,
    Rejected {
        reason: OwnedAccountClosurePageLoadRejectionReason,
    },
}

impl From<OwnedAccountClosurePageLoadResult> for OwnedAccountClosurePageLoadOutput {
    fn from(value: OwnedAccountClosurePageLoadResult) -> Self {
        match value {
            OwnedAccountClosurePageLoadResult::Loaded => Self::Loaded,
            OwnedAccountClosurePageLoadResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
