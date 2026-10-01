use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::owned_account_closure::OwnedAccountClosureId;
use serde::{Deserialize, Serialize};

/// Records a terminal account-close failure without stopping other pending closures.
#[command(name = "owned_account_closure_failed_record")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureFailedRecordCommand {
    pub owned_account_closure_id: OwnedAccountClosureId,
    pub account_id: AccountId,
}
