use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency_registrar::{
    CurrencyRegistrarCreateRejectionReason, CurrencyRegistrarId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarCreateOutput {
    Created {
        currency_registrar_id: CurrencyRegistrarId,
    },
    Rejected {
        currency_registrar_id: CurrencyRegistrarId,
        reason: CurrencyRegistrarCreateRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarCreateOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
