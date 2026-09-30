use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::CurrencyRegistrarInvitationId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarInvitationIssueOutput {
    pub currency_registrar_invitation_id: CurrencyRegistrarInvitationId,
}

impl CommandOutput for CurrencyRegistrarInvitationIssueOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
