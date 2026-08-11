use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::{OrganizationInvitationId, OrganizationInvitationIssueRejectionReason};
use serde::{Deserialize, Serialize};

/// The output returned after issuing an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationInvitationIssueOutput {
    Issued {
        organization_invitation_id: OrganizationInvitationId,
    },
    Rejected {
        organization_invitation_id: OrganizationInvitationId,
        reason: OrganizationInvitationIssueRejectionReason,
    },
}

impl CommandOutput for OrganizationInvitationIssueOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
