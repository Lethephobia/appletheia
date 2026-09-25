use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::OrganizationHandleChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after an organization operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationHandleChangeOutput {
    Changed,
    Rejected {
        reason: OrganizationHandleChangeRejectionReason,
    },
}

impl CommandOutput for OrganizationHandleChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
