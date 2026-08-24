use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::{CurrencyId, CurrencyLifecycleRejectionReason};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyActivateOutput {
    Activated {
        currency_id: CurrencyId,
    },
    Rejected {
        currency_id: CurrencyId,
        reason: CurrencyLifecycleRejectionReason,
    },
}

impl CommandOutput for CurrencyActivateOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
