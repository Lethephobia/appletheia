use appletheia::command;
use banking_ledger_domain::wallet_bookmark::{WalletBookmarkDisplayName, WalletBookmarkId};
use serde::{Deserialize, Serialize};

/// Changes a wallet bookmark display name.
#[command(name = "wallet_bookmark_set_display_name")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletBookmarkSetDisplayNameCommand {
    pub wallet_bookmark_id: WalletBookmarkId,
    pub display_name: Option<WalletBookmarkDisplayName>,
}
