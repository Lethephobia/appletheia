use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosureRecordRejectionReason, OwnedAccountClosureRecordResult,
};
use serde::{Deserialize, Serialize};

/// Returned after recording an account close result.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosureAccountCloseRecordOutput {
    Recorded,
    Rejected {
        reason: OwnedAccountClosureRecordRejectionReason,
    },
}

impl From<OwnedAccountClosureRecordResult> for OwnedAccountClosureAccountCloseRecordOutput {
    fn from(value: OwnedAccountClosureRecordResult) -> Self {
        match value {
            OwnedAccountClosureRecordResult::Recorded => Self::Recorded,
            OwnedAccountClosureRecordResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
