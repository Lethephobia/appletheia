use appletheia::command;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
use serde::{Deserialize, Serialize};

/// Increases the total supply of a currency definition.
#[command(name = "currency_definition_increase_supply")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionIncreaseSupplyCommand {
    pub currency_definition_id: CurrencyDefinitionId,
    pub amount: CurrencyAmount,
}
