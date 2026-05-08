use banking_ledger_domain::owned_account_closure::OwnedAccountClosureId;
use serde::{Deserialize, Serialize};

/// Returned after an owned account closure request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureRequestOutput {
    pub owned_account_closure_id: OwnedAccountClosureId,
}
