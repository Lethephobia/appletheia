use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner, AccountStatus};
use banking_ledger_domain::core::CurrencyAmount;

use super::OwnedAccountListItemCurrency;

/// Read model for one account list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListItem {
    pub id: AccountId,
    pub created_at: EventOccurredAt,
    pub owner: AccountOwner,
    pub name: AccountName,
    pub currency: OwnedAccountListItemCurrency,
    pub balance: CurrencyAmount,
    pub reserved_balance: CurrencyAmount,
    pub status: AccountStatus,
}
