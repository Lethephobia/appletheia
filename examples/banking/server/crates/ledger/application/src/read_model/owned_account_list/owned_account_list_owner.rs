use serde::Serialize;

use appletheia::domain::EventId;

use super::{OwnedAccountListOwnerOrganization, OwnedAccountListOwnerUser};

/// Owner shown in an owned account list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum OwnedAccountListOwner {
    User(OwnedAccountListOwnerUser),
    Organization(OwnedAccountListOwnerOrganization),
}

impl OwnedAccountListOwner {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        match self {
            Self::User(owner) => owner.observation.event_ids().collect(),
            Self::Organization(owner) => owner.observation.event_ids().collect(),
        }
    }
}
