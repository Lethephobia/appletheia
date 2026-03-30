use appletheia::command;
use banking_ledger_domain::core::CurrencySymbol;
use banking_ledger_domain::currency_definition::{CurrencyDefinitionId, CurrencyName};
use serde::{Deserialize, Serialize};

/// Applies a partial update to a currency definition.
#[command(name = "currency_definition_update")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionUpdateCommand {
    pub currency_definition_id: CurrencyDefinitionId,
    pub symbol: Option<CurrencySymbol>,
    pub name: Option<CurrencyName>,
}
