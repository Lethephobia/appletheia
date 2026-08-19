use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::query;
use banking_iam_domain::OrganizationId;

use crate::read_model::{
    OrganizationMemberListCriteria, OrganizationMemberListCursor, OrganizationMemberListSortKey,
};

/// Query parameters for organization member list reads.
#[query(name = "organization_member_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMemberListQuery {
    pub organization_id: OrganizationId,
    pub criteria: OrganizationMemberListCriteria,
    pub sort: Sort<OrganizationMemberListSortKey>,
    pub page: CursorWindow<OrganizationMemberListCursor>,
}
