use appletheia::domain::EventId;

use super::{CurrencyListItemOwnerOrganization, CurrencyListItemOwnerUser};

/// Owner shown in a currency list item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CurrencyListItemOwner {
    User(CurrencyListItemOwnerUser),
    Organization(CurrencyListItemOwnerOrganization),
}

impl CurrencyListItemOwner {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        match self {
            Self::User(owner) => owner.observation.event_ids().collect(),
            Self::Organization(owner) => owner.observation.event_ids().collect(),
        }
    }
}
