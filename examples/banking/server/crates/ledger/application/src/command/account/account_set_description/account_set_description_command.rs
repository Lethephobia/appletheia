use appletheia::command;
use banking_ledger_domain::account::{AccountDescription, AccountId};
use serde::{Deserialize, Serialize};

#[command(name = "account_set_description")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountSetDescriptionCommand {
    pub account_id: AccountId,
    pub description: Option<AccountDescription>,
}
