use appletheia::domain::EventId;
use banking_ledger_domain::account::AccountId;

use super::OwnedAccountTransactionListItemCounterpartyAccountOwner;
use crate::read_model::ReadModelObservation;

/// Counterparty account shown in a transfer transaction list item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListItemCounterpartyAccount {
    pub id: AccountId,
    pub owner: OwnedAccountTransactionListItemCounterpartyAccountOwner,
    pub observation: ReadModelObservation,
}

impl OwnedAccountTransactionListItemCounterpartyAccount {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.owner.observed_event_ids()),
        )
    }
}
