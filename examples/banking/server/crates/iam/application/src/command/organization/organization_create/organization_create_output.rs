use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::{OrganizationCreateRejectionReason, OrganizationId};
use serde::{Deserialize, Serialize};

/// The output returned after creating an organization.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationCreateOutput {
    Created {
        organization_id: OrganizationId,
    },
    Rejected {
        organization_id: OrganizationId,
        reason: OrganizationCreateRejectionReason,
    },
}

impl CommandOutput for OrganizationCreateOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
