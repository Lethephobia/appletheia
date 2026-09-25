use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::CurrencyRegistrarJoinRequestRejectRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after rejecting an currency registrar join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarJoinRequestRejectOutput {
    Rejected,
    RejectionRejected {
        reason: CurrencyRegistrarJoinRequestRejectRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarJoinRequestRejectOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
