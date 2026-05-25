use appletheia::command;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Releases previously reserved currency supply.
#[command(name = "currency_supply_release")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencySupplyReleaseCommand {
    pub currency_id: CurrencyId,
    pub amount: CurrencyAmount,
}
