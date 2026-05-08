use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::owned_account_closure::OwnedAccountClosureId;
use serde::{Deserialize, Serialize};

/// Records a successful account close for an owned account closure.
#[command(name = "owned_account_closure_account_close_record")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureAccountCloseRecordCommand {
    pub owned_account_closure_id: OwnedAccountClosureId,
    pub account_id: AccountId,
}
