use appletheia::application::read_model::{ReadModel, ReadModelName};
use serde::Serialize;

mod owned_account_list_criteria;
mod owned_account_list_cursor;
mod owned_account_list_item;
mod owned_account_list_item_currency;
mod owned_account_list_item_status;
mod owned_account_list_item_status_error;
mod owned_account_list_owner;
mod owned_account_list_owner_organization;
mod owned_account_list_owner_user;
mod owned_account_list_reader;
mod owned_account_list_reader_error;
mod owned_account_list_sort_key;

pub use owned_account_list_criteria::OwnedAccountListCriteria;
pub use owned_account_list_cursor::OwnedAccountListCursor;
pub use owned_account_list_item::OwnedAccountListItem;
pub use owned_account_list_item_currency::OwnedAccountListItemCurrency;
pub use owned_account_list_item_status::OwnedAccountListItemStatus;
pub use owned_account_list_item_status_error::OwnedAccountListItemStatusError;
pub use owned_account_list_owner::OwnedAccountListOwner;
pub use owned_account_list_owner_organization::OwnedAccountListOwnerOrganization;
pub use owned_account_list_owner_user::OwnedAccountListOwnerUser;
pub use owned_account_list_reader::OwnedAccountListReader;
pub use owned_account_list_reader_error::OwnedAccountListReaderError;
pub use owned_account_list_sort_key::OwnedAccountListSortKey;

/// Read model for account list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OwnedAccountList {
    pub owner: OwnedAccountListOwner,
    pub items: Vec<OwnedAccountListItem>,
    pub start_cursor: Option<OwnedAccountListCursor>,
    pub end_cursor: Option<OwnedAccountListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModel for OwnedAccountList {
    const NAME: ReadModelName = ReadModelName::new("owned_account_list");
}
