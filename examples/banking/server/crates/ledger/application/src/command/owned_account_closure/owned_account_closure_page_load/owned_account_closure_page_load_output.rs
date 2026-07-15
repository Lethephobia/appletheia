use banking_ledger_domain::owned_account_closure::OwnedAccountClosurePageLoadRejectionReason;
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
