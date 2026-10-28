use appletheia::command;
use banking_ledger_domain::account::AccountId;
use serde::{Deserialize, Serialize};

/// Freezes the specified account.
#[command(name = "account_freeze")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountFreezeCommand {
    pub account_id: AccountId,
}
