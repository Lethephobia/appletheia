use appletheia::application::read_model::{ReadModel, ReadModelName};
use serde::Serialize;

mod organization_join_request_list_criteria;
mod organization_join_request_list_cursor;
mod organization_join_request_list_item;
mod organization_join_request_list_item_status;
mod organization_join_request_list_organization;
mod organization_join_request_list_reader;
mod organization_join_request_list_reader_error;
mod organization_join_request_list_requester;
mod organization_join_request_list_sort_key;

pub use organization_join_request_list_criteria::OrganizationJoinRequestListCriteria;
pub use organization_join_request_list_cursor::OrganizationJoinRequestListCursor;
pub use organization_join_request_list_item::OrganizationJoinRequestListItem;
pub use organization_join_request_list_item_status::OrganizationJoinRequestListItemStatus;
pub use organization_join_request_list_organization::OrganizationJoinRequestListOrganization;
pub use organization_join_request_list_reader::OrganizationJoinRequestListReader;
pub use organization_join_request_list_reader_error::OrganizationJoinRequestListReaderError;
pub use organization_join_request_list_requester::OrganizationJoinRequestListRequester;
pub use organization_join_request_list_sort_key::OrganizationJoinRequestListSortKey;

/// Read model for organization-scoped join request list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationJoinRequestList {
    pub organization: OrganizationJoinRequestListOrganization,
    pub items: Vec<OrganizationJoinRequestListItem>,
    pub start_cursor: Option<OrganizationJoinRequestListCursor>,
    pub end_cursor: Option<OrganizationJoinRequestListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModel for OrganizationJoinRequestList {
    const NAME: ReadModelName = ReadModelName::new("organization_join_request_list");
}
