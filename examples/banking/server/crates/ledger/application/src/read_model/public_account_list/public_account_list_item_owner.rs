use serde::Serialize;

use appletheia::domain::EventId;

use super::{PublicAccountListItemOwnerOrganization, PublicAccountListItemOwnerUser};

/// Owner fields exposed in public account list items.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum PublicAccountListItemOwner {
    User(PublicAccountListItemOwnerUser),
    Organization(PublicAccountListItemOwnerOrganization),
}

impl PublicAccountListItemOwner {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        match self {
            Self::User(owner) => owner.observation.event_ids().collect(),
            Self::Organization(owner) => owner.observation.event_ids().collect(),
        }
    }
}
