use banking_ledger_domain::core::{CurrencyCode, CurrencyDecimals};
use banking_ledger_domain::currency::{CurrencyDescription, CurrencyId, CurrencyStatus};
use banking_ledger_domain::currency_registrar::CurrencyRegistrarId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyFragmentUpsert {
    pub id: CurrencyId,
    pub currency_registrar_id: CurrencyRegistrarId,
    pub code: CurrencyCode,
    pub decimals: CurrencyDecimals,
    pub description: Option<CurrencyDescription>,
    pub status: CurrencyStatus,
}
