use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyId, CurrencyName, CurrencyOwner, CurrencyStatus, CurrencySymbol,
};

/// Represents a normalized currency view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyView {
    pub id: CurrencyId,
    pub owner: CurrencyOwner,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub supply: CurrencyAmount,
    pub status: CurrencyStatus,
}
