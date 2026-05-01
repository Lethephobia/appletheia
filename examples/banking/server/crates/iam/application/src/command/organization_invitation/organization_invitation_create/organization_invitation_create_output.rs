use banking_iam_domain::{
    OrganizationInvitationId, OrganizationInvitationIssueRejectionReason,
    OrganizationInvitationIssueResult,
};
use serde::{Deserialize, Serialize};

/// The output returned after issuing an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationInvitationIssueOutput {
    Issued {
        organization_invitation_id: OrganizationInvitationId,
    },
    Rejected {
        reason: OrganizationInvitationIssueRejectionReason,
    },
}

impl From<OrganizationInvitationIssueResult> for OrganizationInvitationIssueOutput {
    fn from(value: OrganizationInvitationIssueResult) -> Self {
        match value {
            OrganizationInvitationIssueResult::Issued {
                organization_invitation_id,
            } => Self::Issued {
                organization_invitation_id,
            },
            OrganizationInvitationIssueResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
