use appletheia::command;
use banking_ledger_domain::account::{AccountId, AccountName};
use serde::{Deserialize, Serialize};

/// Changes an account name.
#[command(name = "account_name_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountNameChangeCommand {
    pub account_id: AccountId,
    pub name: AccountName,
}
