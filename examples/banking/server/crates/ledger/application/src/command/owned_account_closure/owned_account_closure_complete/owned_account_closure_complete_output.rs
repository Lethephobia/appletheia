use banking_ledger_domain::owned_account_closure::OwnedAccountClosureCompleteRejectionReason;
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
