use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::OrganizationMembershipRemoveRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after removing an organization membership.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipRemoveOutput {
    Removed,
    Rejected {
        reason: OrganizationMembershipRemoveRejectionReason,
    },
}

impl CommandOutput for OrganizationMembershipRemoveOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
