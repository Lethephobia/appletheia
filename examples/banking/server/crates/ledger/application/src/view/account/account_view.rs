use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner, AccountStatus};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;

/// Represents a normalized account view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountView {
    pub id: AccountId,
    pub owner: AccountOwner,
    pub name: AccountName,
    pub currency_id: CurrencyId,
    pub balance: CurrencyAmount,
    pub reserved_balance: CurrencyAmount,
    pub status: AccountStatus,
}
