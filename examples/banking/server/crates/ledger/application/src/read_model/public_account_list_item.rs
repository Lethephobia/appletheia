use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::{AccountId, AccountOwner};

mod public_account_list_item_criteria;
mod public_account_list_item_currency;
mod public_account_list_item_cursor;
mod public_account_list_item_reader;
mod public_account_list_item_reader_error;
mod public_account_list_item_sort_key;
mod public_account_list_item_status;
mod public_account_list_item_writer;
mod public_account_list_item_writer_error;

pub use public_account_list_item_criteria::PublicAccountListItemCriteria;
pub use public_account_list_item_currency::PublicAccountListItemCurrency;
pub use public_account_list_item_cursor::PublicAccountListItemCursor;
pub use public_account_list_item_reader::PublicAccountListItemReader;
pub use public_account_list_item_reader_error::PublicAccountListItemReaderError;
pub use public_account_list_item_sort_key::PublicAccountListItemSortKey;
pub use public_account_list_item_status::PublicAccountListItemStatus;
pub use public_account_list_item_writer::PublicAccountListItemWriter;
pub use public_account_list_item_writer_error::PublicAccountListItemWriterError;

/// Read model for one public account list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListItem {
    pub account_id: AccountId,
    pub owner: AccountOwner,
    pub currency: PublicAccountListItemCurrency,
    pub created_at: EventOccurredAt,
}
