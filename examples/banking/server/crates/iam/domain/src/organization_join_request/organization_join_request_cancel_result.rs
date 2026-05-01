use super::OrganizationJoinRequestCancelRejectionReason;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationJoinRequestCancelResult {
    Canceled,
    Rejected {
        reason: OrganizationJoinRequestCancelRejectionReason,
    },
}
