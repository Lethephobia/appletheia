use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::OrganizationInvitationAcceptRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after accepting an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationInvitationAcceptOutput {
    Accepted,
    Rejected {
        reason: OrganizationInvitationAcceptRejectionReason,
    },
}

impl CommandOutput for OrganizationInvitationAcceptOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
