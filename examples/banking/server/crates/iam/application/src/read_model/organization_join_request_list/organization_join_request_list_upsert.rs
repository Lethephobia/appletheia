use banking_iam_domain::{OrganizationId, OrganizationJoinRequestId, UserId};

use super::OrganizationJoinRequestListItemStatus;

/// Describes an organization join request list item upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestListUpsert {
    pub join_request_id: OrganizationJoinRequestId,
    pub organization_id: OrganizationId,
    pub requester_user_id: UserId,
    pub status: OrganizationJoinRequestListItemStatus,
}
