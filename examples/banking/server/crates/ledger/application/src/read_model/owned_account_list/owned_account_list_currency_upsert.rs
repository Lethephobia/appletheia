use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListCurrencyUpsert {
    pub id: CurrencyId,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
}
