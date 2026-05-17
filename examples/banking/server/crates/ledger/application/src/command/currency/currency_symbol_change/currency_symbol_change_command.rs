use appletheia::command;
use banking_ledger_domain::currency::{CurrencyId, CurrencySymbol};
use serde::{Deserialize, Serialize};

/// Changes a currency symbol.
#[command(name = "currency_symbol_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencySymbolChangeCommand {
    pub currency_id: CurrencyId,
    pub symbol: CurrencySymbol,
}
