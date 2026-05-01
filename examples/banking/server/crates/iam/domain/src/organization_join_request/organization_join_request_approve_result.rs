use super::OrganizationJoinRequestApproveRejectionReason;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationJoinRequestApproveResult {
    Approved,
    Rejected {
        reason: OrganizationJoinRequestApproveRejectionReason,
    },
}
