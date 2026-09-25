use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::{OrganizationMembershipCreateRejectionReason, OrganizationMembershipId};
use serde::{Deserialize, Serialize};

/// Returned after creating an organization membership.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipCreateOutput {
    Created {
        organization_membership_id: OrganizationMembershipId,
    },
    Rejected {
        organization_membership_id: OrganizationMembershipId,
        reason: OrganizationMembershipCreateRejectionReason,
    },
}

impl CommandOutput for OrganizationMembershipCreateOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
