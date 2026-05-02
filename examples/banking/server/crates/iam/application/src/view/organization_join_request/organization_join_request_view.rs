use banking_iam_domain::{
    OrganizationId, OrganizationJoinRequestId, OrganizationJoinRequestStatus, UserId,
};

/// Represents a normalized organization join request view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestView {
    pub id: OrganizationJoinRequestId,
    pub organization_id: OrganizationId,
    pub requester_id: UserId,
    pub status: OrganizationJoinRequestStatus,
}
