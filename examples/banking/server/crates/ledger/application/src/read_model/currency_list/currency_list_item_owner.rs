use serde::{Deserialize, Serialize};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};

use super::{CurrencyListItemOwnerOrganizationPart, CurrencyListItemOwnerUserPart};

/// Owner shown in a currency list item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CurrencyListItemOwner {
    User(CurrencyListItemOwnerUserPart),
    Organization(CurrencyListItemOwnerOrganizationPart),
}

impl ReadModelObservationSource for CurrencyListItemOwner {
    fn observations(&self) -> Vec<ReadModelObservation> {
        match self {
            Self::User(owner) => vec![owner.observation],
            Self::Organization(owner) => vec![owner.observation],
        }
    }
}
