use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency_registrar_membership::CurrencyRegistrarMembershipId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyRegistrarMembershipCreateOutput {
    pub currency_registrar_membership_id: CurrencyRegistrarMembershipId,
}

impl CommandOutput for CurrencyRegistrarMembershipCreateOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
