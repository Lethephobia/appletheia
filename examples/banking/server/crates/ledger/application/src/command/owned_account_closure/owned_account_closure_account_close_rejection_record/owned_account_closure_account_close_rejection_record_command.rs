use appletheia::command;
use banking_ledger_domain::account::{AccountCloseRejectionReason, AccountId};
use banking_ledger_domain::owned_account_closure::OwnedAccountClosureId;
use serde::{Deserialize, Serialize};

/// Records a rejected account close for an owned account closure.
#[command(name = "owned_account_closure_account_close_rejection_record")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureAccountCloseRejectionRecordCommand {
    pub owned_account_closure_id: OwnedAccountClosureId,
    pub account_id: AccountId,
    pub reason: AccountCloseRejectionReason,
}
