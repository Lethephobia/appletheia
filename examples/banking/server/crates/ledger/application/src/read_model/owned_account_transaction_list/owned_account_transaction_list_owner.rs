use serde::{Deserialize, Serialize};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};

use super::{
    OwnedAccountTransactionListOwnerOrganizationPart, OwnedAccountTransactionListOwnerUserPart,
};

/// Owner shown in an owned account transaction list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OwnedAccountTransactionListOwner {
    User(OwnedAccountTransactionListOwnerUserPart),
    Organization(OwnedAccountTransactionListOwnerOrganizationPart),
}

impl ReadModelObservationSource for OwnedAccountTransactionListOwner {
    fn observations(&self) -> Vec<ReadModelObservation> {
        match self {
            Self::User(owner) => vec![owner.observation],
            Self::Organization(owner) => vec![owner.observation],
        }
    }
}
