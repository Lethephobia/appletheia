use appletheia::query;
use banking_iam_domain::UserId;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

use crate::read_model::{
    UserOrganizationInvitationListCriteria, UserOrganizationInvitationListCursor,
    UserOrganizationInvitationListSortKey,
};

/// Query parameters for user organization invitation list reads.
#[query(name = "user_organization_invitation_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserOrganizationInvitationListQuery {
    pub user_id: UserId,
    pub criteria: UserOrganizationInvitationListCriteria,
    pub cursor_options: Option<
        CursorOptions<UserOrganizationInvitationListSortKey, UserOrganizationInvitationListCursor>,
    >,
    pub limit: PageSize,
}
