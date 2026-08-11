use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency_issuance::CurrencyIssuanceCompleteRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after completing a currency issuance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyIssuanceCompleteOutput {
    Completed,
    Rejected {
        reason: CurrencyIssuanceCompleteRejectionReason,
    },
}

impl CommandOutput for CurrencyIssuanceCompleteOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
