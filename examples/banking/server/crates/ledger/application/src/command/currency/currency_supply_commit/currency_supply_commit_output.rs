use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::CurrencySupplyCommitRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after committing reserved currency supply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencySupplyCommitOutput {
    Committed,
    Rejected {
        reason: CurrencySupplyCommitRejectionReason,
    },
}

impl CommandOutput for CurrencySupplyCommitOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
