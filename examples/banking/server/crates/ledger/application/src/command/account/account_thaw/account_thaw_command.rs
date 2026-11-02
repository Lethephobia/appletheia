use appletheia::command;
use banking_ledger_domain::account::AccountId;
use serde::{Deserialize, Serialize};

/// Thaws the specified account.
#[command(name = "account_thaw")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountThawCommand {
    pub account_id: AccountId,
}
