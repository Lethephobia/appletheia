use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::CurrencySupplyReserveRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after reserving currency supply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencySupplyReserveOutput {
    Reserved,
    Rejected {
        reason: CurrencySupplyReserveRejectionReason,
    },
}

impl CommandOutput for CurrencySupplyReserveOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
