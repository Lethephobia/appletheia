use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::OrganizationJoinRequestCancelRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after canceling an organization join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationJoinRequestCancelOutput {
    Canceled,
    Rejected {
        reason: OrganizationJoinRequestCancelRejectionReason,
    },
}

impl CommandOutput for OrganizationJoinRequestCancelOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
