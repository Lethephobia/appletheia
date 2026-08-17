use serde::{Deserialize, Serialize};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};

use super::{PublicAccountListItemOwnerOrganizationPart, PublicAccountListItemOwnerUserPart};

/// Owner fields exposed in public account list items.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PublicAccountListItemOwner {
    User(PublicAccountListItemOwnerUserPart),
    Organization(PublicAccountListItemOwnerOrganizationPart),
}

impl ReadModelObservationSource for PublicAccountListItemOwner {
    fn observations(&self) -> Vec<ReadModelObservation> {
        match self {
            Self::User(owner) => vec![owner.observation],
            Self::Organization(owner) => vec![owner.observation],
        }
    }
}
