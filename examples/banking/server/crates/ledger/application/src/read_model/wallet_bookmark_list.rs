use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use banking_iam_application::{OrganizationFragment, UserFragment};
use banking_ledger_domain::wallet_bookmark::WalletBookmarkOwner;
use serde::Serialize;

use crate::projection::WalletBookmarkFragment;

mod wallet_bookmark_list_criteria;
mod wallet_bookmark_list_cursor;
mod wallet_bookmark_list_item;
mod wallet_bookmark_list_reader;
mod wallet_bookmark_list_reader_error;
mod wallet_bookmark_list_sort_key;

pub use wallet_bookmark_list_criteria::WalletBookmarkListCriteria;
pub use wallet_bookmark_list_cursor::WalletBookmarkListCursor;
pub use wallet_bookmark_list_item::WalletBookmarkListItem;
pub use wallet_bookmark_list_reader::WalletBookmarkListReader;
pub use wallet_bookmark_list_reader_error::WalletBookmarkListReaderError;
pub use wallet_bookmark_list_sort_key::WalletBookmarkListSortKey;

/// Read model for wallet bookmark list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WalletBookmarkList {
    pub owner: WalletBookmarkOwner,
    pub items: Vec<WalletBookmarkListItem>,
    pub start_cursor: Option<WalletBookmarkListCursor>,
    pub end_cursor: Option<WalletBookmarkListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModelObservationSource for WalletBookmarkList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.items.iter().map(|item| item.observation).collect()
    }
}

impl ReadModel for WalletBookmarkList {
    const NAME: ReadModelName = ReadModelName::new("wallet_bookmark_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut partitions = Vec::with_capacity(self.items.len() * 2);
        for item in &self.items {
            partitions.push(SerializedPartition::try_from_fragment_key::<
                WalletBookmarkFragment,
            >(&item.wallet_bookmark_id)?);
            let owner_partition = match item.owner {
                WalletBookmarkOwner::User(user_id) => {
                    SerializedPartition::try_from_fragment_key::<UserFragment>(&user_id)?
                }
                WalletBookmarkOwner::Organization(organization_id) => {
                    SerializedPartition::try_from_fragment_key::<OrganizationFragment>(
                        &organization_id,
                    )?
                }
            };
            partitions.push(owner_partition);
        }
        Ok(partitions)
    }
}
