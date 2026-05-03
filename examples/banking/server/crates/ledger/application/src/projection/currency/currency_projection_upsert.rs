use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyId, CurrencyName, CurrencyOwner, CurrencyStatus, CurrencySymbol,
};

/// Attributes required to upsert a normalized currency projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyProjectionUpsert {
    pub id: CurrencyId,
    pub owner: CurrencyOwner,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub supply: CurrencyAmount,
    pub status: CurrencyStatus,
}
