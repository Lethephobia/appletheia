use appletheia::application::read_model::ReadModelObservation;
use banking_ledger_domain::core::{CurrencyCode, CurrencyDecimals};
use banking_ledger_domain::currency::CurrencyId;
use serde::Serialize;

/// Currency projection joined to a public account list item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PublicAccountListItemCurrency {
    pub id: CurrencyId,
    pub code: CurrencyCode,
    pub decimals: CurrencyDecimals,
    pub observation: ReadModelObservation,
}
