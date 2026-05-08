use appletheia::command;
use banking_ledger_domain::account::AccountOwner;
use serde::{Deserialize, Serialize};

/// Requests closing all accounts owned by the owner.
#[command(name = "owned_account_closure_request")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureRequestCommand {
    pub owner: AccountOwner,
}
