use appletheia::application::read_model::{ReadModel, ReadModelName};
use serde::Serialize;

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
}
