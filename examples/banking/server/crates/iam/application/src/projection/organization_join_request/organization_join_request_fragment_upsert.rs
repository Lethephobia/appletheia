use banking_iam_domain::{
    OrganizationId, OrganizationJoinRequestId, OrganizationJoinRequestStatus, UserId,
};

/// Values used to create or replace an organization join request fragment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestFragmentUpsert {
    pub join_request_id: OrganizationJoinRequestId,
    pub organization_id: OrganizationId,
    pub requester_user_id: UserId,
    pub status: OrganizationJoinRequestStatus,
}
