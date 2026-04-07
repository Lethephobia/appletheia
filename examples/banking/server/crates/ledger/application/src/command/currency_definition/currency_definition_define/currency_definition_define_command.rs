use appletheia::command;
use banking_iam_domain::OrganizationId;
use banking_ledger_domain::core::{CurrencyDecimals, CurrencySymbol};
use banking_ledger_domain::currency_definition::CurrencyName;
use serde::{Deserialize, Serialize};

/// Defines a new currency definition.
#[command(name = "currency_definition_define")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionDefineCommand {
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub organization_id: Option<OrganizationId>,
}
