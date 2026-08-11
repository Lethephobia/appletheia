use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::CurrencySupplyReleaseRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after releasing reserved currency supply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencySupplyReleaseOutput {
    Released,
    Rejected {
        reason: CurrencySupplyReleaseRejectionReason,
    },
}

impl CommandOutput for CurrencySupplyReleaseOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
