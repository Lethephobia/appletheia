use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::CurrencyRegistrarInvitationDeclineRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after declining an currency registrar invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarInvitationDeclineOutput {
    Declined,
    Rejected {
        reason: CurrencyRegistrarInvitationDeclineRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarInvitationDeclineOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
