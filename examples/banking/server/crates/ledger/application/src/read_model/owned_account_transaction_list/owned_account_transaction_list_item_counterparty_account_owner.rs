use serde::{Deserialize, Serialize};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};

use super::{
    OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganizationPart,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerUserPart,
};

/// Counterparty account owner shown in a transfer transaction list item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OwnedAccountTransactionListItemCounterpartyAccountOwner {
    User(OwnedAccountTransactionListItemCounterpartyAccountOwnerUserPart),
    Organization(OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganizationPart),
}

impl ReadModelObservationSource for OwnedAccountTransactionListItemCounterpartyAccountOwner {
    fn observations(&self) -> Vec<ReadModelObservation> {
        match self {
            Self::User(owner) => vec![owner.observation],
            Self::Organization(owner) => vec![owner.observation],
        }
    }
}
