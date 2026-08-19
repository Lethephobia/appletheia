use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use serde::Serialize;

use crate::projection::UserFragment;

mod public_user_list_criteria;
mod public_user_list_cursor;
mod public_user_list_item;
mod public_user_list_item_status;
mod public_user_list_reader;
mod public_user_list_reader_error;
mod public_user_list_sort_key;

pub use public_user_list_criteria::PublicUserListCriteria;
pub use public_user_list_cursor::PublicUserListCursor;
pub use public_user_list_item::PublicUserListItem;
pub use public_user_list_item_status::PublicUserListItemStatus;
pub use public_user_list_reader::PublicUserListReader;
pub use public_user_list_reader_error::PublicUserListReaderError;
pub use public_user_list_sort_key::PublicUserListSortKey;

/// Read model for public user list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PublicUserList {
    pub items: Vec<PublicUserListItem>,
    pub start_cursor: Option<PublicUserListCursor>,
    pub end_cursor: Option<PublicUserListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModel for PublicUserList {
    const NAME: ReadModelName = ReadModelName::new("public_user_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        self.items
            .iter()
            .map(|item| SerializedPartition::try_from_fragment_key::<UserFragment>(&item.user_id))
            .collect()
    }
}

impl ReadModelObservationSource for PublicUserList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.items.iter().map(|item| item.observation).collect()
    }
}
