use banking_ledger_domain::core::CurrencySymbol;
use banking_ledger_domain::currency_definition::{CurrencyDefinitionId, CurrencyName};
use serde::{Deserialize, Serialize};

/// The output returned after updating a currency definition.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionUpdateOutput {
    pub currency_definition_id: CurrencyDefinitionId,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
}

impl CurrencyDefinitionUpdateOutput {
    /// Creates a new currency-definition-update output.
    pub fn new(
        currency_definition_id: CurrencyDefinitionId,
        symbol: CurrencySymbol,
        name: CurrencyName,
    ) -> Self {
        Self {
            currency_definition_id,
            symbol,
            name,
        }
    }
}
