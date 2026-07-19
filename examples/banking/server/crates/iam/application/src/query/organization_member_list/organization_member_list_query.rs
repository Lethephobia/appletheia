use appletheia::query;
use banking_iam_domain::OrganizationId;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

use crate::read_model::{
    OrganizationMemberListCriteria, OrganizationMemberListCursor, OrganizationMemberListSortKey,
};

/// Query parameters for organization member list reads.
#[query(name = "organization_member_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMemberListQuery {
    pub organization_id: OrganizationId,
    pub criteria: OrganizationMemberListCriteria,
    pub cursor_options:
        Option<CursorOptions<OrganizationMemberListSortKey, OrganizationMemberListCursor>>,
    pub limit: PageSize,
}
