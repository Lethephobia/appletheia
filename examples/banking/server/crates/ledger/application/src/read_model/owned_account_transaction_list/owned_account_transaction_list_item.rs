use appletheia::domain::{EventId, EventOccurredAt};
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;

use super::{
    OwnedAccountTransactionId, OwnedAccountTransactionListItemCurrency,
    OwnedAccountTransactionListItemDirection, OwnedAccountTransactionListItemKind,
    OwnedAccountTransactionListItemStatus,
};
use crate::read_model::ReadModelObservation;

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
    pub observation: ReadModelObservation,
}

impl OwnedAccountTransactionListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.currency.observation.event_ids())
                .chain(self.kind.observed_event_ids()),
        )
    }
}
