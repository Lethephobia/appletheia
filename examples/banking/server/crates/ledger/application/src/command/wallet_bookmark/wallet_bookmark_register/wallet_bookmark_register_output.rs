use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkId, WalletBookmarkRegisterRejectionReason,
};
use serde::{Deserialize, Serialize};

/// Returned after a wallet bookmark registration request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WalletBookmarkRegisterOutput {
    Registered {
        wallet_bookmark_id: WalletBookmarkId,
    },
    Rejected {
        wallet_bookmark_id: WalletBookmarkId,
        reason: WalletBookmarkRegisterRejectionReason,
    },
}

impl CommandOutput for WalletBookmarkRegisterOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
