use serde::{Deserialize, Serialize};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};

use super::{OwnedAccountListOwnerOrganizationPart, OwnedAccountListOwnerUserPart};

/// Owner shown in an owned account list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OwnedAccountListOwner {
    User(OwnedAccountListOwnerUserPart),
    Organization(OwnedAccountListOwnerOrganizationPart),
}

impl ReadModelObservationSource for OwnedAccountListOwner {
    fn observations(&self) -> Vec<ReadModelObservation> {
        match self {
            Self::User(owner) => vec![owner.observation],
            Self::Organization(owner) => vec![owner.observation],
        }
    }
}
