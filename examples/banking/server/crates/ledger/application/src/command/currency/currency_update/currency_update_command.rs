use appletheia::command;
use banking_ledger_domain::currency::{CurrencyId, CurrencyName, CurrencySymbol};
use serde::{Deserialize, Serialize};

/// Applies a partial update to a currency.
#[command(name = "currency_update")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyUpdateCommand {
    pub currency_id: CurrencyId,
    pub symbol: Option<CurrencySymbol>,
    pub name: Option<CurrencyName>,
}
