use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency_registrar_membership::{
    CurrencyRegistrarMembershipCreateRejectionReason, CurrencyRegistrarMembershipId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarMembershipCreateOutput {
    Created {
        currency_registrar_membership_id: CurrencyRegistrarMembershipId,
    },
    Rejected {
        currency_registrar_membership_id: CurrencyRegistrarMembershipId,
        reason: CurrencyRegistrarMembershipCreateRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarMembershipCreateOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
