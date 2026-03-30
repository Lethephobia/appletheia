use appletheia::command;
use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
use serde::{Deserialize, Serialize};

/// Activates the specified currency definition.
#[command(name = "currency_definition_activate")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionActivateCommand {
    pub currency_definition_id: CurrencyDefinitionId,
}
