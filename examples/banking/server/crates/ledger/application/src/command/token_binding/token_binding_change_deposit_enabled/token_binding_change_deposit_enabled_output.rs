use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::token_binding::TokenBindingId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenBindingChangeDepositEnabledOutput {
    pub token_binding_id: TokenBindingId,
    pub enabled: bool,
}

impl CommandOutput for TokenBindingChangeDepositEnabledOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
