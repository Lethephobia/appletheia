use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};

use crate::read_model::ReadModelObservation;

/// Currency part of an owned account list item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListItemCurrency {
    pub id: CurrencyId,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub observation: ReadModelObservation,
}
