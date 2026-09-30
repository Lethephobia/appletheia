use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::OrganizationInvitationId;
use serde::{Deserialize, Serialize};

/// The output returned after issuing an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationIssueOutput {
    pub organization_invitation_id: OrganizationInvitationId,
}

impl CommandOutput for OrganizationInvitationIssueOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
