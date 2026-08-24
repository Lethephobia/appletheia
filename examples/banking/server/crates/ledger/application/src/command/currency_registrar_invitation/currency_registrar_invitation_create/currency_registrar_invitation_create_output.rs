use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::{
    CurrencyRegistrarInvitationId, CurrencyRegistrarInvitationIssueRejectionReason,
};
use serde::{Deserialize, Serialize};

/// The output returned after issuing an currency registrar invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarInvitationIssueOutput {
    Issued {
        currency_registrar_invitation_id: CurrencyRegistrarInvitationId,
    },
    Rejected {
        currency_registrar_invitation_id: CurrencyRegistrarInvitationId,
        reason: CurrencyRegistrarInvitationIssueRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarInvitationIssueOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
