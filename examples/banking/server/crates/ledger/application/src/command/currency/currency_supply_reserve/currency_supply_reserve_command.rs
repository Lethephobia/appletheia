use appletheia::command;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Reserves supply for a currency issuance.
#[command(name = "currency_supply_reserve")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencySupplyReserveCommand {
    pub currency_id: CurrencyId,
    pub amount: CurrencyAmount,
}
