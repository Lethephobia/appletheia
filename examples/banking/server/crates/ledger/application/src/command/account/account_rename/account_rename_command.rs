use appletheia::command;
use banking_ledger_domain::account::{AccountId, AccountName};
use serde::{Deserialize, Serialize};

/// Renames an account.
#[command(name = "account_rename")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountRenameCommand {
    pub account_id: AccountId,
    pub name: AccountName,
}
