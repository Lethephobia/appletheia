use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::wallet_bookmark::WalletBookmarkId;
use serde::{Deserialize, Serialize};

/// Returned after a wallet bookmark registration request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletBookmarkRegisterOutput {
    pub wallet_bookmark_id: WalletBookmarkId,
}

impl CommandOutput for WalletBookmarkRegisterOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
