use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::owned_account_closure::OwnedAccountClosureId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedAccountClosureStartOutput {
    pub owned_account_closure_id: OwnedAccountClosureId,
}

impl CommandOutput for OwnedAccountClosureStartOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
