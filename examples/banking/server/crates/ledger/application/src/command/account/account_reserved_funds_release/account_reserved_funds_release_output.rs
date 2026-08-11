use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::account::AccountReservedFundsReleaseRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after releasing reserved funds in an account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountReservedFundsReleaseOutput {
    Released,
    Rejected {
        reason: AccountReservedFundsReleaseRejectionReason,
    },
}

impl CommandOutput for AccountReservedFundsReleaseOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
