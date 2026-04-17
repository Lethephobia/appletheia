use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Deactivates the specified currency.
#[command(name = "currency_deactivate")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDeactivateCommand {
    pub currency_id: CurrencyId,
}
