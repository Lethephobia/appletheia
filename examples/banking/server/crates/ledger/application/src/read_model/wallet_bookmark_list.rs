mod wallet_bookmark_list_criteria;
mod wallet_bookmark_list_cursor;
mod wallet_bookmark_list_item;
mod wallet_bookmark_list_reader;
mod wallet_bookmark_list_reader_error;
mod wallet_bookmark_list_sort_key;
mod wallet_bookmark_list_upsert;
mod wallet_bookmark_list_writer;
mod wallet_bookmark_list_writer_error;

use banking_ledger_domain::wallet_bookmark::WalletBookmarkOwner;
pub use wallet_bookmark_list_criteria::WalletBookmarkListCriteria;
pub use wallet_bookmark_list_cursor::WalletBookmarkListCursor;
pub use wallet_bookmark_list_item::WalletBookmarkListItem;
pub use wallet_bookmark_list_reader::WalletBookmarkListReader;
pub use wallet_bookmark_list_reader_error::WalletBookmarkListReaderError;
pub use wallet_bookmark_list_sort_key::WalletBookmarkListSortKey;
pub use wallet_bookmark_list_upsert::WalletBookmarkListUpsert;
pub use wallet_bookmark_list_writer::WalletBookmarkListWriter;
pub use wallet_bookmark_list_writer_error::WalletBookmarkListWriterError;

/// Read model for wallet bookmark list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletBookmarkList {
    pub owner: WalletBookmarkOwner,
    pub items: Vec<WalletBookmarkListItem>,
    pub next_cursor: Option<WalletBookmarkListCursor>,
}
