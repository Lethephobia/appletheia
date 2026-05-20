use banking_ledger_domain::owned_account_closure::OwnedAccountClosureRecordRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after recording an account close rejection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosureAccountCloseRejectionRecordOutput {
    Recorded,
    Rejected {
        reason: OwnedAccountClosureRecordRejectionReason,
    },
}
