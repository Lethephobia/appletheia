use appletheia::command;
use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
use serde::{Deserialize, Serialize};

/// Removes the specified currency definition.
#[command(name = "currency_definition_remove")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionRemoveCommand {
    pub currency_definition_id: CurrencyDefinitionId,
}
