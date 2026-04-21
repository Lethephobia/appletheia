use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Activates the specified currency.
#[command(name = "currency_activate")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyActivateCommand {
    pub currency_id: CurrencyId,
}
