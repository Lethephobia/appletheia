use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyDescriptionChangeOutput {
    Changed { currency_id: CurrencyId },
}

impl CommandOutput for CurrencyDescriptionChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
