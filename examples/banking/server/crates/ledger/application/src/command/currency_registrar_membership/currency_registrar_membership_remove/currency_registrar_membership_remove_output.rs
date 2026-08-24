use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency_registrar_membership::CurrencyRegistrarMembershipRemoveRejectionReason;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarMembershipRemoveOutput {
    Removed,
    Rejected {
        reason: CurrencyRegistrarMembershipRemoveRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarMembershipRemoveOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
