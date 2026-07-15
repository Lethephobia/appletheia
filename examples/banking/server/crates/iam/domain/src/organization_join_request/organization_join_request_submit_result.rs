use super::OrganizationJoinRequestSubmitRejectionReason;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationJoinRequestSubmitResult {
    Submitted,
    Rejected {
        reason: OrganizationJoinRequestSubmitRejectionReason,
    },
}
