use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};

use banking_shared_kernel_application::read_model::ReadModelObservation;

/// Currency snapshot for owned account transaction list rows.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListItemCurrency {
    pub id: CurrencyId,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub observation: ReadModelObservation,
}
