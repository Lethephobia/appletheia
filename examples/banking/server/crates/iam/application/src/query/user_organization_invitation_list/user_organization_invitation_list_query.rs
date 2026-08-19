use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::query;
use banking_iam_domain::UserId;

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
    pub sort: Sort<UserOrganizationInvitationListSortKey>,
    pub page: CursorWindow<UserOrganizationInvitationListCursor>,
}
