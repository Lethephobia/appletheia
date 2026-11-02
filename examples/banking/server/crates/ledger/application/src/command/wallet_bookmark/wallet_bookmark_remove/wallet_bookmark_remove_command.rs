use appletheia::command;
use banking_ledger_domain::wallet_bookmark::WalletBookmarkId;
use serde::{Deserialize, Serialize};

/// Removes the specified wallet bookmark.
#[command(name = "wallet_bookmark_remove")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletBookmarkRemoveCommand {
    pub wallet_bookmark_id: WalletBookmarkId,
}
