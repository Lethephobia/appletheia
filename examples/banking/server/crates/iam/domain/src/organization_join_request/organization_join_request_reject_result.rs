use super::OrganizationJoinRequestRejectRejectionReason;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationJoinRequestRejectResult {
    Rejected,
    RejectionRejected {
        reason: OrganizationJoinRequestRejectRejectionReason,
    },
}
