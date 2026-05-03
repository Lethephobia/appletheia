use banking_iam_domain::{
    OrganizationId, OrganizationJoinRequestId, OrganizationJoinRequestStatus, UserId,
};

/// Attributes required to upsert a normalized organization join request projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestProjectionUpsert {
    pub id: OrganizationJoinRequestId,
    pub organization_id: OrganizationId,
    pub requester_id: UserId,
    pub status: OrganizationJoinRequestStatus,
}
