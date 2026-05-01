use super::OrganizationInvitationCancelRejectionReason;

/// Describes the domain outcome of an organization invitation cancel request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationInvitationCancelResult {
    Canceled,
    Rejected {
        reason: OrganizationInvitationCancelRejectionReason,
    },
}
