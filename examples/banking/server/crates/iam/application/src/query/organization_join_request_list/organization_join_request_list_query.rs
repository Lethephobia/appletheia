use appletheia::query;
use banking_iam_domain::OrganizationId;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

use crate::read_model::{
    OrganizationJoinRequestListCriteria, OrganizationJoinRequestListCursor,
    OrganizationJoinRequestListSortKey,
};

/// Query parameters for organization join request list reads.
#[query(name = "organization_join_request_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestListQuery {
    pub organization_id: OrganizationId,
    pub criteria: OrganizationJoinRequestListCriteria,
    pub cursor_options: Option<
        CursorOptions<OrganizationJoinRequestListSortKey, OrganizationJoinRequestListCursor>,
    >,
    pub limit: PageSize,
}
