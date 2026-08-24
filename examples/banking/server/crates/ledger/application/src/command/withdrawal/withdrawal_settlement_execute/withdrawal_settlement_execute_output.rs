use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::withdrawal::WithdrawalSettlementExecuteRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after an external withdrawal token transfer attempt is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WithdrawalSettlementExecuteOutput {
    Executed,
    Rejected {
        reason: WithdrawalSettlementExecuteRejectionReason,
    },
}

impl CommandOutput for WithdrawalSettlementExecuteOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
