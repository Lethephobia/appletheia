use serde::Serialize;

use appletheia::domain::EventId;

use super::{OwnedAccountTransactionListOwnerOrganization, OwnedAccountTransactionListOwnerUser};

/// Owner shown in an owned account transaction list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum OwnedAccountTransactionListOwner {
    User(OwnedAccountTransactionListOwnerUser),
    Organization(OwnedAccountTransactionListOwnerOrganization),
}

impl OwnedAccountTransactionListOwner {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        match self {
            Self::User(owner) => owner.observation.event_ids().collect(),
            Self::Organization(owner) => owner.observation.event_ids().collect(),
        }
    }
}
