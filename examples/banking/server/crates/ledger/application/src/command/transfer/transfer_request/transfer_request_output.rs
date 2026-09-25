use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::transfer::{TransferId, TransferRequestRejectionReason};
use serde::{Deserialize, Serialize};

/// The output returned after requesting a transfer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TransferRequestOutput {
    Requested {
        transfer_id: TransferId,
    },
    Rejected {
        transfer_id: TransferId,
        reason: TransferRequestRejectionReason,
    },
}

impl CommandOutput for TransferRequestOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
