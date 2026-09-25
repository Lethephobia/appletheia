use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::account::AccountNameChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after changing an account name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountNameChangeOutput {
    Changed,
    Rejected {
        reason: AccountNameChangeRejectionReason,
    },
}

impl CommandOutput for AccountNameChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
