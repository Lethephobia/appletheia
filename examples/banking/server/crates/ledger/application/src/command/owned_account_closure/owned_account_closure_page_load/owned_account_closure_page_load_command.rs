use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::owned_account_closure::OwnedAccountClosureId;
use serde::{Deserialize, Serialize};

/// Loads one page of accounts for an owned account closure.
#[command(name = "owned_account_closure_page_load")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosurePageLoadCommand {
    pub owned_account_closure_id: OwnedAccountClosureId,
    pub cursor: Option<AccountId>,
    pub page_size: u32,
}
