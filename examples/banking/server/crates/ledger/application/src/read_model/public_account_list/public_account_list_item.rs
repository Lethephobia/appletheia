use serde::Serialize;

use appletheia::domain::{EventId, EventOccurredAt};
use banking_ledger_domain::account::AccountId;

use super::{PublicAccountListItemCurrency, PublicAccountListItemOwner};
use appletheia::application::read_model::ReadModelObservation;

/// Read model for one public account list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PublicAccountListItem {
    pub account_id: AccountId,
    pub owner: PublicAccountListItemOwner,
    pub currency: PublicAccountListItemCurrency,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl PublicAccountListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.owner.observed_event_ids()),
        )
    }
}
