use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::CurrencyNameChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after changing a currency name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyNameChangeOutput {
    Changed,
    Rejected {
        reason: CurrencyNameChangeRejectionReason,
    },
}

impl CommandOutput for CurrencyNameChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
