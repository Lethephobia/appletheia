use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::query;
use banking_iam_domain::UserId;

use crate::read_model::{
    UserOrganizationMembershipListCursor, UserOrganizationMembershipListSortKey,
};

/// Query parameters for user organization membership list reads.
#[query(name = "user_organization_membership_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserOrganizationMembershipListQuery {
    pub user_id: UserId,
    pub sort: Sort<UserOrganizationMembershipListSortKey>,
    pub page: CursorWindow<UserOrganizationMembershipListCursor>,
}
