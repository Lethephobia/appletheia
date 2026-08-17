use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::query;
use banking_iam_domain::UserId;

use crate::read_model::{
    UserOrganizationJoinRequestListCriteria, UserOrganizationJoinRequestListCursor,
    UserOrganizationJoinRequestListSortKey,
};

/// Query parameters for user organization join request list reads.
#[query(name = "user_organization_join_request_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserOrganizationJoinRequestListQuery {
    pub user_id: UserId,
    pub criteria: UserOrganizationJoinRequestListCriteria,
    pub sort: Sort<UserOrganizationJoinRequestListSortKey>,
    pub page: CursorPage<UserOrganizationJoinRequestListCursor>,
}
