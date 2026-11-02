use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::token_binding::TokenBindingId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenBindingUnbindOutput {
    pub token_binding_id: TokenBindingId,
}

impl CommandOutput for TokenBindingUnbindOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
