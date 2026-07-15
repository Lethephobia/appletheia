use appletheia::domain::EventId;

use super::{
    OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganization,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerUser,
};

/// Counterparty account owner shown in a transfer transaction list item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OwnedAccountTransactionListItemCounterpartyAccountOwner {
    User(OwnedAccountTransactionListItemCounterpartyAccountOwnerUser),
    Organization(OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganization),
}

impl OwnedAccountTransactionListItemCounterpartyAccountOwner {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        match self {
            Self::User(owner) => owner.observation.event_ids().collect(),
            Self::Organization(owner) => owner.observation.event_ids().collect(),
        }
    }
}
