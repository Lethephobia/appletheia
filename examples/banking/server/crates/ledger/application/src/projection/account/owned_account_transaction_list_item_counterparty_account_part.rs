use serde::{Deserialize, Serialize};

use banking_ledger_domain::account::AccountId;

use crate::projection::FragmentOwner;
use crate::read_model::OwnedAccountTransactionListItemCounterpartyAccountOwner;
use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};

use super::AccountFragment;

/// Counterparty account shown in a transfer transaction list item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OwnedAccountTransactionListItemCounterpartyAccountPart {
    pub id: AccountId,
    pub owner: OwnedAccountTransactionListItemCounterpartyAccountOwner,
    pub observation: ReadModelObservation,
}

impl From<AccountFragment> for OwnedAccountTransactionListItemCounterpartyAccountPart {
    fn from(fragment: AccountFragment) -> Self {
        let owner = match fragment.owner {
            FragmentOwner::User(user) => {
                OwnedAccountTransactionListItemCounterpartyAccountOwner::User((*user).into())
            }
            FragmentOwner::Organization(organization) => {
                OwnedAccountTransactionListItemCounterpartyAccountOwner::Organization(
                    (*organization).into(),
                )
            }
        };

        Self {
            id: fragment.id,
            owner,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for OwnedAccountTransactionListItemCounterpartyAccountPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        std::iter::once(self.observation)
            .chain(self.owner.observations())
            .collect()
    }
}
