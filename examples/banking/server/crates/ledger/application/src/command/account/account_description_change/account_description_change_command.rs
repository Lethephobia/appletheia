use appletheia::command;
use banking_ledger_domain::account::{AccountDescription, AccountId};
use serde::{Deserialize, Serialize};

#[command(name = "account_description_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountDescriptionChangeCommand {
    pub account_id: AccountId,
    pub description: Option<AccountDescription>,
}
