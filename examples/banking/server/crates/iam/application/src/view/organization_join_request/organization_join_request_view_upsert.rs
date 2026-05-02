use banking_iam_domain::{
    OrganizationId, OrganizationJoinRequestId, OrganizationJoinRequestStatus, UserId,
};

/// Attributes required to upsert a normalized organization join request view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestViewUpsert {
    pub id: OrganizationJoinRequestId,
    pub organization_id: OrganizationId,
    pub requester_id: UserId,
    pub status: OrganizationJoinRequestStatus,
}
