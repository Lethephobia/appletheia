use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::query;
use banking_iam_domain::OrganizationId;

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
    pub sort: Sort<OrganizationJoinRequestListSortKey>,
    pub page: CursorPage<OrganizationJoinRequestListCursor>,
}
