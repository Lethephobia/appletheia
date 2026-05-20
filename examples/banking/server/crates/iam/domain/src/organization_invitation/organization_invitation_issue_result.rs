use super::{OrganizationInvitationId, OrganizationInvitationIssueRejectionReason};

/// Describes the domain outcome of an organization invitation issue request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationInvitationIssueResult {
    Issued {
        organization_invitation_id: OrganizationInvitationId,
    },
    Rejected {
        organization_invitation_id: OrganizationInvitationId,
        reason: OrganizationInvitationIssueRejectionReason,
    },
}
