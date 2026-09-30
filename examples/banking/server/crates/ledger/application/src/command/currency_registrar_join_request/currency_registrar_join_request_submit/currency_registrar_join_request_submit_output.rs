use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::CurrencyRegistrarJoinRequestId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarJoinRequestSubmitOutput {
    pub currency_registrar_join_request_id: CurrencyRegistrarJoinRequestId,
}

impl CommandOutput for CurrencyRegistrarJoinRequestSubmitOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
