use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::CurrencyRegistrarJoinRequestApproveRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after approving an currency registrar join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarJoinRequestApproveOutput {
    Approved,
    Rejected {
        reason: CurrencyRegistrarJoinRequestApproveRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarJoinRequestApproveOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
