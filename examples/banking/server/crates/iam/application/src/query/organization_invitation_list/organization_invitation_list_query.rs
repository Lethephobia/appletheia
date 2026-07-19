use appletheia::query;
use banking_iam_domain::OrganizationId;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

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
    pub cursor_options:
        Option<CursorOptions<OrganizationInvitationListSortKey, OrganizationInvitationListCursor>>,
    pub limit: PageSize,
}
