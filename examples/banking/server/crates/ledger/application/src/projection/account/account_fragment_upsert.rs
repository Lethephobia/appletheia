use banking_ledger_domain::account::{AccountDescription, AccountId, AccountName, AccountOwner};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;

use super::MaterializedAccountStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountFragmentUpsert {
    pub id: AccountId,
    pub owner: AccountOwner,
    pub name: AccountName,
    pub description: Option<AccountDescription>,
    pub currency_id: CurrencyId,
    pub balance: CurrencyAmount,
    pub reserved_balance: CurrencyAmount,
    pub status: MaterializedAccountStatus,
}
