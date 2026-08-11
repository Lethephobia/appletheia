use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::CurrencyImageChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after changing a currency image.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyImageChangeOutput {
    Changed,
    Rejected {
        reason: CurrencyImageChangeRejectionReason,
    },
}

impl CommandOutput for CurrencyImageChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
