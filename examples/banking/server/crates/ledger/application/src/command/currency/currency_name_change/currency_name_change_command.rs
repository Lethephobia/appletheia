use appletheia::command;
use banking_ledger_domain::currency::{CurrencyId, CurrencyName};
use serde::{Deserialize, Serialize};

/// Changes a currency name.
#[command(name = "currency_name_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyNameChangeCommand {
    pub currency_id: CurrencyId,
    pub name: CurrencyName,
}
