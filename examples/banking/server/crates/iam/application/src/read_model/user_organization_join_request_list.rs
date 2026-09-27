use appletheia::application::read_model::{ReadModel, ReadModelName};
use serde::Serialize;

mod user_organization_join_request_list_criteria;
mod user_organization_join_request_list_cursor;
mod user_organization_join_request_list_item;
mod user_organization_join_request_list_item_status;
mod user_organization_join_request_list_organization;
mod user_organization_join_request_list_reader;
mod user_organization_join_request_list_reader_error;
mod user_organization_join_request_list_sort_key;
mod user_organization_join_request_list_user;

pub use user_organization_join_request_list_criteria::UserOrganizationJoinRequestListCriteria;
pub use user_organization_join_request_list_cursor::UserOrganizationJoinRequestListCursor;
pub use user_organization_join_request_list_item::UserOrganizationJoinRequestListItem;
pub use user_organization_join_request_list_item_status::UserOrganizationJoinRequestListItemStatus;
pub use user_organization_join_request_list_organization::UserOrganizationJoinRequestListOrganization;
pub use user_organization_join_request_list_reader::UserOrganizationJoinRequestListReader;
pub use user_organization_join_request_list_reader_error::UserOrganizationJoinRequestListReaderError;
pub use user_organization_join_request_list_sort_key::UserOrganizationJoinRequestListSortKey;
pub use user_organization_join_request_list_user::UserOrganizationJoinRequestListUser;

/// Read model for user-scoped organization join request list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationJoinRequestList {
    pub user: UserOrganizationJoinRequestListUser,
    pub items: Vec<UserOrganizationJoinRequestListItem>,
    pub start_cursor: Option<UserOrganizationJoinRequestListCursor>,
    pub end_cursor: Option<UserOrganizationJoinRequestListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModel for UserOrganizationJoinRequestList {
    const NAME: ReadModelName = ReadModelName::new("user_organization_join_request_list");
}
