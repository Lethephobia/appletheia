use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};

/// Currency part of a transfer recipient list item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferRecipientListItemCurrency {
    pub id: CurrencyId,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
}
