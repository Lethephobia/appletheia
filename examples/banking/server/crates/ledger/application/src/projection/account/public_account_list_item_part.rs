use serde::{Deserialize, Serialize};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::AccountId;

use crate::projection::{FragmentOwner, PublicAccountListItemCurrencyPart};
use crate::read_model::PublicAccountListItemOwner;

use super::{AccountFragment, MaterializedAccountStatus};

/// Read model for one public account list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicAccountListItemPart {
    pub account_id: AccountId,
    pub owner: PublicAccountListItemOwner,
    pub currency: PublicAccountListItemCurrencyPart,
    pub status: MaterializedAccountStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl From<AccountFragment> for PublicAccountListItemPart {
    fn from(fragment: AccountFragment) -> Self {
        let owner = match fragment.owner {
            FragmentOwner::User(user) => PublicAccountListItemOwner::User((*user).into()),
            FragmentOwner::Organization(organization) => {
                PublicAccountListItemOwner::Organization((*organization).into())
            }
        };

        Self {
            account_id: fragment.id,
            owner,
            currency: fragment.currency.into(),
            status: fragment.status,
            created_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for PublicAccountListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        std::iter::once(self.observation)
            .chain(std::iter::once(self.currency.observation))
            .chain(self.owner.observations())
            .collect()
    }
}
