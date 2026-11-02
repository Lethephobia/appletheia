use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferRequestOutput {
    pub transfer_id: TransferId,
}

impl CommandOutput for TransferRequestOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
