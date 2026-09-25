use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::OrganizationMembershipRolesChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after changing organization membership roles.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipRolesChangeOutput {
    Changed,
    Rejected {
        reason: OrganizationMembershipRolesChangeRejectionReason,
    },
}

impl CommandOutput for OrganizationMembershipRolesChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
