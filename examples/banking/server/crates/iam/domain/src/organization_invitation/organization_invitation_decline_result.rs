use super::OrganizationInvitationDeclineRejectionReason;

/// Describes the domain outcome of an organization invitation decline request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationInvitationDeclineResult {
    Declined,
    Rejected {
        reason: OrganizationInvitationDeclineRejectionReason,
    },
}
