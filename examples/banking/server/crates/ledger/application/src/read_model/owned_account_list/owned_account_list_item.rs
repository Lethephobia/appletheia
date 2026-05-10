use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::{AccountId, AccountName};
use banking_ledger_domain::core::CurrencyAmount;

use super::{OwnedAccountListItemCurrency, OwnedAccountListItemStatus};

/// Read model for one account list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListItem {
    pub account_id: AccountId,
    pub name: AccountName,
    pub currency: OwnedAccountListItemCurrency,
    pub balance: CurrencyAmount,
    pub reserved_balance: CurrencyAmount,
    pub status: OwnedAccountListItemStatus,
    pub created_at: EventOccurredAt,
}
