use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::CurrencyRegistrarJoinRequestCancelRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after canceling an currency registrar join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarJoinRequestCancelOutput {
    Canceled,
    Rejected {
        reason: CurrencyRegistrarJoinRequestCancelRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarJoinRequestCancelOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
