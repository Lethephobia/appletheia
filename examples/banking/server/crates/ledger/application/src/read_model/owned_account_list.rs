use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use banking_iam_application::{OrganizationFragment, UserFragment};
use serde::Serialize;

use crate::projection::{AccountFragment, CurrencyFragment};

mod owned_account_list_criteria;
mod owned_account_list_cursor;
mod owned_account_list_item;
mod owned_account_list_item_currency;
mod owned_account_list_item_status;
mod owned_account_list_item_status_error;
mod owned_account_list_owner;
mod owned_account_list_owner_organization;
mod owned_account_list_owner_user;
mod owned_account_list_reader;
mod owned_account_list_reader_error;
mod owned_account_list_sort_key;

pub use owned_account_list_criteria::OwnedAccountListCriteria;
pub use owned_account_list_cursor::OwnedAccountListCursor;
pub use owned_account_list_item::OwnedAccountListItem;
pub use owned_account_list_item_currency::OwnedAccountListItemCurrency;
pub use owned_account_list_item_status::OwnedAccountListItemStatus;
pub use owned_account_list_item_status_error::OwnedAccountListItemStatusError;
pub use owned_account_list_owner::OwnedAccountListOwner;
pub use owned_account_list_owner_organization::OwnedAccountListOwnerOrganization;
pub use owned_account_list_owner_user::OwnedAccountListOwnerUser;
pub use owned_account_list_reader::OwnedAccountListReader;
pub use owned_account_list_reader_error::OwnedAccountListReaderError;
pub use owned_account_list_sort_key::OwnedAccountListSortKey;

/// Read model for account list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OwnedAccountList {
    pub owner: OwnedAccountListOwner,
    pub items: Vec<OwnedAccountListItem>,
    pub start_cursor: Option<OwnedAccountListCursor>,
    pub end_cursor: Option<OwnedAccountListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModelObservationSource for OwnedAccountList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        let owner = match &self.owner {
            OwnedAccountListOwner::User(owner) => owner.observation,
            OwnedAccountListOwner::Organization(owner) => owner.observation,
        };
        std::iter::once(owner)
            .chain(
                self.items
                    .iter()
                    .flat_map(|item| [item.observation, item.currency.observation]),
            )
            .collect()
    }
}

impl ReadModel for OwnedAccountList {
    const NAME: ReadModelName = ReadModelName::new("owned_account_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut partitions = Vec::with_capacity(1 + self.items.len() * 2);
        let owner_partition = match &self.owner {
            OwnedAccountListOwner::User(owner) => {
                SerializedPartition::try_from_fragment_key::<UserFragment>(&owner.id)?
            }
            OwnedAccountListOwner::Organization(owner) => {
                SerializedPartition::try_from_fragment_key::<OrganizationFragment>(&owner.id)?
            }
        };
        partitions.push(owner_partition);
        for item in &self.items {
            partitions.push(
                SerializedPartition::try_from_fragment_key::<AccountFragment>(&item.account_id)?,
            );
            partitions.push(SerializedPartition::try_from_fragment_key::<
                CurrencyFragment,
            >(&item.currency.id)?);
        }
        Ok(partitions)
    }
}
