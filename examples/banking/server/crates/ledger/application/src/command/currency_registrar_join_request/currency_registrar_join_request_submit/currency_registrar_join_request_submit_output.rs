use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::{
    CurrencyRegistrarJoinRequestId, CurrencyRegistrarJoinRequestSubmitRejectionReason,
};
use serde::{Deserialize, Serialize};

/// The output returned after submitting an currency registrar join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarJoinRequestSubmitOutput {
    Submitted {
        currency_registrar_join_request_id: CurrencyRegistrarJoinRequestId,
    },
    Rejected {
        currency_registrar_join_request_id: CurrencyRegistrarJoinRequestId,
        reason: CurrencyRegistrarJoinRequestSubmitRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarJoinRequestSubmitOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
