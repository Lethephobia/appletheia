use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_iam_domain::OrganizationJoinRequestId;

use super::{OrganizationJoinRequestListItemStatus, OrganizationJoinRequestListRequester};

/// One organization join request list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationJoinRequestListItem {
    pub join_request_id: OrganizationJoinRequestId,
    pub requester: OrganizationJoinRequestListRequester,
    pub status: OrganizationJoinRequestListItemStatus,
    pub created_at: EventOccurredAt,
}
