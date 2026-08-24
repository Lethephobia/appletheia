use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency_registrar::CurrencyRegistrarHandleChangeRejectionReason;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarHandleChangeOutput {
    Changed,
    Rejected {
        reason: CurrencyRegistrarHandleChangeRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarHandleChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
