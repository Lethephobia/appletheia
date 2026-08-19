use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use banking_iam_application::{OrganizationFragment, UserFragment};
use serde::Serialize;

use crate::projection::CurrencyFragment;

mod currency_list_criteria;
mod currency_list_cursor;
mod currency_list_item;
mod currency_list_item_owner;
mod currency_list_item_owner_organization;
mod currency_list_item_owner_user;
mod currency_list_item_status;
mod currency_list_item_status_error;
mod currency_list_reader;
mod currency_list_reader_error;
mod currency_list_sort_key;

pub use currency_list_criteria::CurrencyListCriteria;
pub use currency_list_cursor::CurrencyListCursor;
pub use currency_list_item::CurrencyListItem;
pub use currency_list_item_owner::CurrencyListItemOwner;
pub use currency_list_item_owner_organization::CurrencyListItemOwnerOrganization;
pub use currency_list_item_owner_user::CurrencyListItemOwnerUser;
pub use currency_list_item_status::CurrencyListItemStatus;
pub use currency_list_item_status_error::CurrencyListItemStatusError;
pub use currency_list_reader::CurrencyListReader;
pub use currency_list_reader_error::CurrencyListReaderError;
pub use currency_list_sort_key::CurrencyListSortKey;

/// Read model for public currency list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CurrencyList {
    pub items: Vec<CurrencyListItem>,
    pub start_cursor: Option<CurrencyListCursor>,
    pub end_cursor: Option<CurrencyListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModelObservationSource for CurrencyList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.items
            .iter()
            .flat_map(|item| {
                let owner = match &item.owner {
                    CurrencyListItemOwner::User(owner) => owner.observation,
                    CurrencyListItemOwner::Organization(owner) => owner.observation,
                };
                [item.observation, owner]
            })
            .collect()
    }
}

impl ReadModel for CurrencyList {
    const NAME: ReadModelName = ReadModelName::new("currency_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut partitions = Vec::with_capacity(self.items.len() * 2);
        for item in &self.items {
            partitions.push(SerializedPartition::try_from_fragment_key::<
                CurrencyFragment,
            >(&item.currency_id)?);
            let owner_partition = match &item.owner {
                CurrencyListItemOwner::User(owner) => {
                    SerializedPartition::try_from_fragment_key::<UserFragment>(&owner.id)?
                }
                CurrencyListItemOwner::Organization(owner) => {
                    SerializedPartition::try_from_fragment_key::<OrganizationFragment>(&owner.id)?
                }
            };
            partitions.push(owner_partition);
        }
        Ok(partitions)
    }
}
