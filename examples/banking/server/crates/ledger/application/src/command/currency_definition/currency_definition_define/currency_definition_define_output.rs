use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
use serde::{Deserialize, Serialize};

/// The output returned after defining a currency definition.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionDefineOutput {
    pub currency_definition_id: CurrencyDefinitionId,
}

impl CurrencyDefinitionDefineOutput {
    /// Creates a new currency-definition-define output.
    pub fn new(currency_definition_id: CurrencyDefinitionId) -> Self {
        Self {
            currency_definition_id,
        }
    }
}
