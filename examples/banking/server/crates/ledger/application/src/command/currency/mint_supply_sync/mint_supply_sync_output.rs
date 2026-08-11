use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::MintSupplySyncRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a currency mint supply sync request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum MintSupplySyncOutput {
    Synced,
    Rejected {
        reason: MintSupplySyncRejectionReason,
    },
}

impl CommandOutput for MintSupplySyncOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
