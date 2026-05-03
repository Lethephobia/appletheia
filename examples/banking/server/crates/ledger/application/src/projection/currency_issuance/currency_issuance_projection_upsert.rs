use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use banking_ledger_domain::currency_issuance::{CurrencyIssuanceId, CurrencyIssuanceStatus};

/// Attributes required to upsert a normalized currency issuance projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyIssuanceProjectionUpsert {
    pub id: CurrencyIssuanceId,
    pub currency_id: CurrencyId,
    pub destination_account_id: AccountId,
    pub amount: CurrencyAmount,
    pub status: CurrencyIssuanceStatus,
}
