use appletheia::command;
use banking_ledger_domain::account::AccountOwner;
use serde::{Deserialize, Serialize};

/// Starts scan and closure of all accounts owned by a removed owner.
#[command(name = "owned_account_closure_start")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureStartCommand {
    pub owner: AccountOwner,
}
