use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::token_binding::{
    TokenBindingEnablementChangeRejectionReason, TokenBindingId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TokenBindingDepositEnabledChangeOutput {
    Changed {
        token_binding_id: TokenBindingId,
        enabled: bool,
    },
    Rejected {
        token_binding_id: TokenBindingId,
        enabled: bool,
        reason: TokenBindingEnablementChangeRejectionReason,
    },
}

impl CommandOutput for TokenBindingDepositEnabledChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
