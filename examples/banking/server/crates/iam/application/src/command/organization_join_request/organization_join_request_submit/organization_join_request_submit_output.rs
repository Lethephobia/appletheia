use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::OrganizationJoinRequestId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestSubmitOutput {
    pub organization_join_request_id: OrganizationJoinRequestId,
}

impl CommandOutput for OrganizationJoinRequestSubmitOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
