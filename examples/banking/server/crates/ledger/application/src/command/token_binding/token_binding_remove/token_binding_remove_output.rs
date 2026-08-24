use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::token_binding::{TokenBindingId, TokenBindingRemoveRejectionReason};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TokenBindingRemoveOutput {
    Removed {
        token_binding_id: TokenBindingId,
    },
    Rejected {
        token_binding_id: TokenBindingId,
        reason: TokenBindingRemoveRejectionReason,
    },
}

impl CommandOutput for TokenBindingRemoveOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
