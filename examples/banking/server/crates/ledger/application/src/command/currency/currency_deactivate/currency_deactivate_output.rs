use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::CurrencyDeactivateRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a currency deactivation request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyDeactivateOutput {
    Deactivated,
    Rejected {
        reason: CurrencyDeactivateRejectionReason,
    },
}

impl CommandOutput for CurrencyDeactivateOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
