use appletheia::domain::EventOccurredAt;
use banking_iam_domain::OrganizationJoinRequestId;

/// Cursor for organization join request list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationJoinRequestListCursor {
    pub created_at: EventOccurredAt,
    pub join_request_id: OrganizationJoinRequestId,
}
