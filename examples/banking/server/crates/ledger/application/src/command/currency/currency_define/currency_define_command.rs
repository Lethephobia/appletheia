use appletheia::command;
use banking_ledger_domain::core::{CurrencyDecimals, CurrencySymbol};
use banking_ledger_domain::currency::{CurrencyName, CurrencyOwner};
use serde::{Deserialize, Serialize};

/// Defines a new currency.
#[command(name = "currency_define")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefineCommand {
    pub owner: CurrencyOwner,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
}
