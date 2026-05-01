use super::OrganizationInvitationAcceptRejectionReason;

/// Describes the domain outcome of an organization invitation accept request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationInvitationAcceptResult {
    Accepted,
    Rejected {
        reason: OrganizationInvitationAcceptRejectionReason,
    },
}
