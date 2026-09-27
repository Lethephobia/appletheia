use appletheia::application::read_model::{ReadModel, ReadModelName};
use banking_ledger_domain::wallet_bookmark::WalletBookmarkOwner;
use serde::Serialize;

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

impl ReadModel for WalletBookmarkList {
    const NAME: ReadModelName = ReadModelName::new("wallet_bookmark_list");
}
