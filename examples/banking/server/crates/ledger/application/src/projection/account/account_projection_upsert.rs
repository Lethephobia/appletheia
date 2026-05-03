use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner, AccountStatus};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;

/// Attributes required to upsert a normalized account projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountProjectionUpsert {
    pub id: AccountId,
    pub owner: AccountOwner,
    pub name: AccountName,
    pub currency_id: CurrencyId,
    pub balance: CurrencyAmount,
    pub reserved_balance: CurrencyAmount,
    pub status: AccountStatus,
}
