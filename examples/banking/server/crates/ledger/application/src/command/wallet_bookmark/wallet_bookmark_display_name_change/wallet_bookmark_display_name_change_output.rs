use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::wallet_bookmark::WalletBookmarkDisplayNameChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after changing a wallet bookmark display name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WalletBookmarkDisplayNameChangeOutput {
    Changed,
    Rejected {
        reason: WalletBookmarkDisplayNameChangeRejectionReason,
    },
}

impl CommandOutput for WalletBookmarkDisplayNameChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
