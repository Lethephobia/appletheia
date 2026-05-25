use appletheia::command;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Commits previously reserved supply into confirmed supply.
#[command(name = "currency_supply_commit")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencySupplyCommitCommand {
    pub currency_id: CurrencyId,
    pub amount: CurrencyAmount,
}
