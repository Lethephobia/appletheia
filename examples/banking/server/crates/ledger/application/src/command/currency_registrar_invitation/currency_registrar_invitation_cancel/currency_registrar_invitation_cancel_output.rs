use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::CurrencyRegistrarInvitationCancelRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after canceling an currency registrar invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarInvitationCancelOutput {
    Canceled,
    Rejected {
        reason: CurrencyRegistrarInvitationCancelRejectionReason,
    },
}

impl CommandOutput for CurrencyRegistrarInvitationCancelOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
