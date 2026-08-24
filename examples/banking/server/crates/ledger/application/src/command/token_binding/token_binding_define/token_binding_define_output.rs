use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::token_binding::{TokenBindingDefineRejectionReason, TokenBindingId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TokenBindingDefineOutput {
    Defined {
        token_binding_id: TokenBindingId,
    },
    Rejected {
        token_binding_id: TokenBindingId,
        reason: TokenBindingDefineRejectionReason,
    },
}

impl CommandOutput for TokenBindingDefineOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
