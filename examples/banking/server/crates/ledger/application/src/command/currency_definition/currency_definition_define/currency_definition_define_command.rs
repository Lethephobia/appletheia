use appletheia::command;
use banking_ledger_domain::core::{CurrencyDecimals, CurrencySymbol};
use banking_ledger_domain::currency_definition::{CurrencyDefinitionOwner, CurrencyName};
use serde::{Deserialize, Serialize};

/// Defines a new currency definition.
#[command(name = "currency_definition_define")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionDefineCommand {
    pub owner: CurrencyDefinitionOwner,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
}
