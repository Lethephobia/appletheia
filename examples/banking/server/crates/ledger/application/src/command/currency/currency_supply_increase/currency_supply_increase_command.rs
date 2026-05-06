use appletheia::command;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Increases the total supply of a currency.
#[command(name = "currency_supply_increase")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencySupplyIncreaseCommand {
    pub currency_id: CurrencyId,
    pub amount: CurrencyAmount,
}
