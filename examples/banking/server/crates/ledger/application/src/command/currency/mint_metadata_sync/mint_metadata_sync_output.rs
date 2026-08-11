use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::MintMetadataSyncRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a currency mint metadata sync request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum MintMetadataSyncOutput {
    Synced,
    Rejected {
        reason: MintMetadataSyncRejectionReason,
    },
}

impl CommandOutput for MintMetadataSyncOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
