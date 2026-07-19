use banking_iam_domain::{OrganizationId, OrganizationJoinRequestId, UserId};

use super::UserOrganizationJoinRequestListItemStatus;

/// Describes a user organization join request list item upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserOrganizationJoinRequestListUpsert {
    pub join_request_id: OrganizationJoinRequestId,
    pub requester_user_id: UserId,
    pub organization_id: OrganizationId,
    pub status: UserOrganizationJoinRequestListItemStatus,
}
