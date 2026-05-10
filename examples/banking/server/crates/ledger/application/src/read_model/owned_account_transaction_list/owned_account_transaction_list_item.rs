use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;

use super::{
    OwnedAccountTransactionId, OwnedAccountTransactionListItemCurrency,
    OwnedAccountTransactionListItemDirection, OwnedAccountTransactionListItemKind,
    OwnedAccountTransactionListItemStatus,
};

/// Read model for one owned account transaction list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListItem {
    pub transaction_id: OwnedAccountTransactionId,
    pub account_id: AccountId,
    pub currency: OwnedAccountTransactionListItemCurrency,
    pub amount: CurrencyAmount,
    pub direction: OwnedAccountTransactionListItemDirection,
    pub kind: OwnedAccountTransactionListItemKind,
    pub status: OwnedAccountTransactionListItemStatus,
    pub occurred_at: EventOccurredAt,
    pub created_at: EventOccurredAt,
}
