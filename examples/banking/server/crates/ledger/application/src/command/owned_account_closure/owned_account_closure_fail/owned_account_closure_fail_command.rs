use appletheia::command;
use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosureFailureReason, OwnedAccountClosureId,
};
use serde::{Deserialize, Serialize};

/// Fails the specified owned account closure.
#[command(name = "owned_account_closure_fail")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureFailCommand {
    pub owned_account_closure_id: OwnedAccountClosureId,
    pub reason: OwnedAccountClosureFailureReason,
}
