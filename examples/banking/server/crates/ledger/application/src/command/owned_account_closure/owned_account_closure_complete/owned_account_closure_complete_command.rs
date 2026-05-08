use appletheia::command;
use banking_ledger_domain::owned_account_closure::OwnedAccountClosureId;
use serde::{Deserialize, Serialize};

/// Completes the specified owned account closure.
#[command(name = "owned_account_closure_complete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureCompleteCommand {
    pub owned_account_closure_id: OwnedAccountClosureId,
}
