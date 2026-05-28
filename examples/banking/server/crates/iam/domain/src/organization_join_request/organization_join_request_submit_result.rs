use super::{OrganizationJoinRequestId, OrganizationJoinRequestSubmitRejectionReason};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationJoinRequestSubmitResult {
    Submitted {
        organization_join_request_id: OrganizationJoinRequestId,
    },
    Rejected {
        organization_join_request_id: OrganizationJoinRequestId,
        reason: OrganizationJoinRequestSubmitRejectionReason,
    },
}
