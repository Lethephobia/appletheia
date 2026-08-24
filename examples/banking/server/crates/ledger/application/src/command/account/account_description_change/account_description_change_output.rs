use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::account::AccountDescriptionChangeRejectionReason;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountDescriptionChangeOutput {
    Changed,
    Rejected {
        reason: AccountDescriptionChangeRejectionReason,
    },
}

impl CommandOutput for AccountDescriptionChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
