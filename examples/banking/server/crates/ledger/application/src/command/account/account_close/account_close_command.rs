use appletheia::command;
use banking_ledger_domain::account::AccountId;
use serde::{Deserialize, Serialize};

/// Closes the specified account.
#[command(name = "account_close")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountCloseCommand {
    pub account_id: AccountId,
}
