use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::CurrencyDescriptionChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after changing a currency description.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyDescriptionChangeOutput {
    Changed,
    Rejected {
        reason: CurrencyDescriptionChangeRejectionReason,
    },
}

impl CommandOutput for CurrencyDescriptionChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
