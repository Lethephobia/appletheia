use appletheia::command;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
use serde::{Deserialize, Serialize};

/// Decreases the total supply of a currency definition.
#[command(name = "currency_definition_decrease_supply")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionDecreaseSupplyCommand {
    pub currency_definition_id: CurrencyDefinitionId,
    pub amount: CurrencyAmount,
}
