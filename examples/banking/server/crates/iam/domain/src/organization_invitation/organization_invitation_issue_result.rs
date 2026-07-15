use super::OrganizationInvitationIssueRejectionReason;

/// Describes the domain outcome of an organization invitation issue request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationInvitationIssueResult {
    Issued,
    Rejected {
        reason: OrganizationInvitationIssueRejectionReason,
    },
}
