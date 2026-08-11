use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::OrganizationMembershipRemoveRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after removing an organization membership from a user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserOrganizationMembershipRemoveOutput {
    Removed,
    Rejected {
        reason: OrganizationMembershipRemoveRejectionReason,
    },
}

impl CommandOutput for UserOrganizationMembershipRemoveOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
