use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_iam_domain::OrganizationJoinRequestId;

use super::{
    UserOrganizationJoinRequestListItemStatus, UserOrganizationJoinRequestListOrganization,
};

/// One user organization join request list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationJoinRequestListItem {
    pub join_request_id: OrganizationJoinRequestId,
    pub organization: UserOrganizationJoinRequestListOrganization,
    pub status: UserOrganizationJoinRequestListItemStatus,
    pub created_at: EventOccurredAt,
}
