use banking_iam_domain::OrganizationInvitationId;
use serde::{Deserialize, Serialize};

/// The output returned after issuing an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationIssueOutput {
    pub organization_invitation_id: OrganizationInvitationId,
}

impl OrganizationInvitationIssueOutput {
    /// Creates a new organization-invitation-issue output.
    pub fn new(organization_invitation_id: OrganizationInvitationId) -> Self {
        Self {
            organization_invitation_id,
        }
    }
}
