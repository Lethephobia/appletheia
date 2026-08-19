use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use banking_iam_application::{OrganizationFragment, UserFragment};
use serde::Serialize;

use crate::projection::{AccountFragment, CurrencyFragment};

mod public_account_list_criteria;
mod public_account_list_cursor;
mod public_account_list_item;
mod public_account_list_item_currency;
mod public_account_list_item_owner;
mod public_account_list_item_owner_organization;
mod public_account_list_item_owner_user;
mod public_account_list_item_status;
mod public_account_list_item_status_error;
mod public_account_list_reader;
mod public_account_list_reader_error;
mod public_account_list_sort_key;

pub use public_account_list_criteria::PublicAccountListCriteria;
pub use public_account_list_cursor::PublicAccountListCursor;
pub use public_account_list_item::PublicAccountListItem;
pub use public_account_list_item_currency::PublicAccountListItemCurrency;
pub use public_account_list_item_owner::PublicAccountListItemOwner;
pub use public_account_list_item_owner_organization::PublicAccountListItemOwnerOrganization;
pub use public_account_list_item_owner_user::PublicAccountListItemOwnerUser;
pub use public_account_list_item_status::PublicAccountListItemStatus;
pub use public_account_list_item_status_error::PublicAccountListItemStatusError;
pub use public_account_list_reader::PublicAccountListReader;
pub use public_account_list_reader_error::PublicAccountListReaderError;
pub use public_account_list_sort_key::PublicAccountListSortKey;

/// Read model for public account list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PublicAccountList {
    pub items: Vec<PublicAccountListItem>,
    pub start_cursor: Option<PublicAccountListCursor>,
    pub end_cursor: Option<PublicAccountListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModelObservationSource for PublicAccountList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.items
            .iter()
            .flat_map(|item| {
                let owner = match &item.owner {
                    PublicAccountListItemOwner::User(owner) => owner.observation,
                    PublicAccountListItemOwner::Organization(owner) => owner.observation,
                };
                [item.observation, item.currency.observation, owner]
            })
            .collect()
    }
}

impl ReadModel for PublicAccountList {
    const NAME: ReadModelName = ReadModelName::new("public_account_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut partitions = Vec::with_capacity(self.items.len() * 3);
        for item in &self.items {
            partitions.push(
                SerializedPartition::try_from_fragment_key::<AccountFragment>(&item.account_id)?,
            );
            partitions.push(SerializedPartition::try_from_fragment_key::<
                CurrencyFragment,
            >(&item.currency.id)?);
            let owner_partition = match &item.owner {
                PublicAccountListItemOwner::User(owner) => {
                    SerializedPartition::try_from_fragment_key::<UserFragment>(&owner.id)?
                }
                PublicAccountListItemOwner::Organization(owner) => {
                    SerializedPartition::try_from_fragment_key::<OrganizationFragment>(&owner.id)?
                }
            };
            partitions.push(owner_partition);
        }
        Ok(partitions)
    }
}
