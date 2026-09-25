use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::CurrencyRegistrarInvitationAcceptRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after accepting an currency registrar invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarInvitationAcceptOutput {
    Accepted,
    Rejected {
        reason: CurrencyRegistrarInvitationAcceptRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarInvitationAcceptOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
