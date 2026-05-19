use super::{OrganizationJoinRequestId, OrganizationJoinRequestRequestRejectionReason};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationJoinRequestRequestResult {
    Requested {
        organization_join_request_id: OrganizationJoinRequestId,
    },
    Rejected {
        reason: OrganizationJoinRequestRequestRejectionReason,
    },
}
