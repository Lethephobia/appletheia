use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::OrganizationInvitationDeclineRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after declining an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationInvitationDeclineOutput {
    Declined,
    Rejected {
        reason: OrganizationInvitationDeclineRejectionReason,
    },
}

impl CommandOutput for OrganizationInvitationDeclineOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
