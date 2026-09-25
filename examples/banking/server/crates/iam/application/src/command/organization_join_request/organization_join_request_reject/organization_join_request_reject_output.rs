use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::OrganizationJoinRequestRejectRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after rejecting an organization join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationJoinRequestRejectOutput {
    Rejected,
    RejectionRejected {
        reason: OrganizationJoinRequestRejectRejectionReason,
    },
}

impl CommandOutput for OrganizationJoinRequestRejectOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
