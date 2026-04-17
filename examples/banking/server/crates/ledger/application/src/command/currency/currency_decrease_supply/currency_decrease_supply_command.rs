use appletheia::command;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Decreases the total supply of a currency.
#[command(name = "currency_decrease_supply")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDecreaseSupplyCommand {
    pub currency_id: CurrencyId,
    pub amount: CurrencyAmount,
}
