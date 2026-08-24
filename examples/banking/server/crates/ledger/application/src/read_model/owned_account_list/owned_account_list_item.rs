use serde::Serialize;

use appletheia::domain::{EventId, EventOccurredAt};
use banking_ledger_domain::account::{AccountDescription, AccountId, AccountName};
use banking_ledger_domain::core::CurrencyAmount;

use super::{OwnedAccountListItemCurrency, OwnedAccountListItemStatus};
use appletheia::application::read_model::ReadModelObservation;

/// Read model for one account list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OwnedAccountListItem {
    pub account_id: AccountId,
    pub name: AccountName,
    pub description: Option<AccountDescription>,
    pub currency: OwnedAccountListItemCurrency,
    pub balance: CurrencyAmount,
    pub reserved_balance: CurrencyAmount,
    pub status: OwnedAccountListItemStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl OwnedAccountListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(self.observation.event_ids())
    }
}
