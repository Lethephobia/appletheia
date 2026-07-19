mod public_user_list_criteria;
mod public_user_list_cursor;
mod public_user_list_item;
mod public_user_list_item_status;
mod public_user_list_reader;
mod public_user_list_reader_error;
mod public_user_list_sort_key;
mod public_user_list_upsert;
mod public_user_list_writer;
mod public_user_list_writer_error;

pub use public_user_list_criteria::PublicUserListCriteria;
pub use public_user_list_cursor::PublicUserListCursor;
pub use public_user_list_item::PublicUserListItem;
pub use public_user_list_item_status::PublicUserListItemStatus;
pub use public_user_list_reader::PublicUserListReader;
pub use public_user_list_reader_error::PublicUserListReaderError;
pub use public_user_list_sort_key::PublicUserListSortKey;
pub use public_user_list_upsert::PublicUserListUpsert;
pub use public_user_list_writer::PublicUserListWriter;
pub use public_user_list_writer_error::PublicUserListWriterError;

/// Read model for public user list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicUserList {
    pub items: Vec<PublicUserListItem>,
    pub next_cursor: Option<PublicUserListCursor>,
}
