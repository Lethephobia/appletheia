use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::CurrencyRemoveRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a currency removal request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRemoveOutput {
    Removed,
    Rejected {
        reason: CurrencyRemoveRejectionReason,
    },
}

impl CommandOutput for CurrencyRemoveOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
