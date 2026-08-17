use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::query;
use banking_iam_domain::OrganizationId;

use crate::read_model::{
    OrganizationInvitationListCriteria, OrganizationInvitationListCursor,
    OrganizationInvitationListSortKey,
};

/// Query parameters for organization invitation list reads.
#[query(name = "organization_invitation_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationInvitationListQuery {
    pub organization_id: OrganizationId,
    pub criteria: OrganizationInvitationListCriteria,
    pub sort: Sort<OrganizationInvitationListSortKey>,
    pub page: CursorPage<OrganizationInvitationListCursor>,
}
