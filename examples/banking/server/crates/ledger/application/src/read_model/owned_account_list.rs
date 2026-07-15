mod owned_account_list_account_upsert;
mod owned_account_list_criteria;
mod owned_account_list_currency_upsert;
mod owned_account_list_cursor;
mod owned_account_list_item;
mod owned_account_list_item_currency;
mod owned_account_list_item_status;
mod owned_account_list_item_status_error;
mod owned_account_list_owner;
mod owned_account_list_owner_organization;
mod owned_account_list_owner_organization_upsert;
mod owned_account_list_owner_user;
mod owned_account_list_owner_user_upsert;
mod owned_account_list_reader;
mod owned_account_list_reader_error;
mod owned_account_list_sort_key;
mod owned_account_list_writer;
mod owned_account_list_writer_error;

pub use owned_account_list_account_upsert::OwnedAccountListAccountUpsert;
pub use owned_account_list_criteria::OwnedAccountListCriteria;
pub use owned_account_list_currency_upsert::OwnedAccountListCurrencyUpsert;
pub use owned_account_list_cursor::OwnedAccountListCursor;
pub use owned_account_list_item::OwnedAccountListItem;
pub use owned_account_list_item_currency::OwnedAccountListItemCurrency;
pub use owned_account_list_item_status::OwnedAccountListItemStatus;
pub use owned_account_list_item_status_error::OwnedAccountListItemStatusError;
pub use owned_account_list_owner::OwnedAccountListOwner;
pub use owned_account_list_owner_organization::OwnedAccountListOwnerOrganization;
pub use owned_account_list_owner_organization_upsert::OwnedAccountListOwnerOrganizationUpsert;
pub use owned_account_list_owner_user::OwnedAccountListOwnerUser;
pub use owned_account_list_owner_user_upsert::OwnedAccountListOwnerUserUpsert;
pub use owned_account_list_reader::OwnedAccountListReader;
pub use owned_account_list_reader_error::OwnedAccountListReaderError;
pub use owned_account_list_sort_key::OwnedAccountListSortKey;
pub use owned_account_list_writer::OwnedAccountListWriter;
pub use owned_account_list_writer_error::OwnedAccountListWriterError;

/// Read model for account list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountList {
    pub owner: OwnedAccountListOwner,
    pub items: Vec<OwnedAccountListItem>,
    pub next_cursor: Option<OwnedAccountListCursor>,
}
