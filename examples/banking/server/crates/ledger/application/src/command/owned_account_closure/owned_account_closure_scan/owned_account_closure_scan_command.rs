use appletheia::command;
use banking_ledger_domain::owned_account_closure::OwnedAccountClosureId;
use serde::{Deserialize, Serialize};

/// Scans the next batch of owned accounts and registers a closure request for each.
#[command(name = "owned_account_closure_scan")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureScanCommand {
    pub owned_account_closure_id: OwnedAccountClosureId,
}
