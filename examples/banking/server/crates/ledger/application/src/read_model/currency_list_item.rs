use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};

mod currency_list_item_criteria;
mod currency_list_item_cursor;
mod currency_list_item_owner;
mod currency_list_item_owner_organization;
mod currency_list_item_owner_user;
mod currency_list_item_reader;
mod currency_list_item_reader_error;
mod currency_list_item_sort_key;
mod currency_list_item_status;
mod currency_list_item_writer;
mod currency_list_item_writer_error;

pub use currency_list_item_criteria::CurrencyListItemCriteria;
pub use currency_list_item_cursor::CurrencyListItemCursor;
pub use currency_list_item_owner::CurrencyListItemOwner;
pub use currency_list_item_owner_organization::CurrencyListItemOwnerOrganization;
pub use currency_list_item_owner_user::CurrencyListItemOwnerUser;
pub use currency_list_item_reader::CurrencyListItemReader;
pub use currency_list_item_reader_error::CurrencyListItemReaderError;
pub use currency_list_item_sort_key::CurrencyListItemSortKey;
pub use currency_list_item_status::CurrencyListItemStatus;
pub use currency_list_item_writer::CurrencyListItemWriter;
pub use currency_list_item_writer_error::CurrencyListItemWriterError;

/// Read model for one public currency list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListItem {
    pub id: CurrencyId,
    pub owner: CurrencyListItemOwner,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub supply: CurrencyAmount,
    pub status: CurrencyListItemStatus,
    pub created_at: EventOccurredAt,
}
