use appletheia::command;
use banking_ledger_domain::account::{AccountName, AccountOwner};
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Opens a new account for the specified owner.
#[command(name = "account_open")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountOpenCommand {
    pub owner: AccountOwner,
    pub name: AccountName,
    pub currency_id: CurrencyId,
}
