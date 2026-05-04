use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner};
use banking_ledger_domain::core::CurrencyAmount;

mod owned_account_list_item_currency;
mod owned_account_list_item_cursor;
mod owned_account_list_item_reader;
mod owned_account_list_item_reader_error;
mod owned_account_list_item_sort_key;
mod owned_account_list_item_status;
mod owned_account_list_item_writer;
mod owned_account_list_item_writer_error;

pub use owned_account_list_item_currency::OwnedAccountListItemCurrency;
pub use owned_account_list_item_cursor::OwnedAccountListItemCursor;
pub use owned_account_list_item_reader::OwnedAccountListItemReader;
pub use owned_account_list_item_reader_error::OwnedAccountListItemReaderError;
pub use owned_account_list_item_sort_key::OwnedAccountListItemSortKey;
pub use owned_account_list_item_status::OwnedAccountListItemStatus;
pub use owned_account_list_item_writer::OwnedAccountListItemWriter;
pub use owned_account_list_item_writer_error::OwnedAccountListItemWriterError;

/// Read model for one account list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListItem {
    pub id: AccountId,
    pub owner: AccountOwner,
    pub name: AccountName,
    pub currency: OwnedAccountListItemCurrency,
    pub balance: CurrencyAmount,
    pub reserved_balance: CurrencyAmount,
    pub status: OwnedAccountListItemStatus,
    pub created_at: EventOccurredAt,
}
