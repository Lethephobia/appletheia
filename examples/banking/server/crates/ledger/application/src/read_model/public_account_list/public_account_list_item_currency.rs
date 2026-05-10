use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};

/// Currency fields exposed in public account list items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListItemCurrency {
    pub id: CurrencyId,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
}
