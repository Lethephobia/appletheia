use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Removes the specified currency.
#[command(name = "currency_remove")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRemoveCommand {
    pub currency_id: CurrencyId,
}
