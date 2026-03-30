use appletheia::command;
use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
use serde::{Deserialize, Serialize};

/// Deactivates the specified currency definition.
#[command(name = "currency_definition_deactivate")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionDeactivateCommand {
    pub currency_definition_id: CurrencyDefinitionId,
}
