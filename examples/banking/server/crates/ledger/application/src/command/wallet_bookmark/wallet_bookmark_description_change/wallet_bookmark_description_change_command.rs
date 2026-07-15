use appletheia::command;
use banking_ledger_domain::wallet_bookmark::{WalletBookmarkDescription, WalletBookmarkId};
use serde::{Deserialize, Serialize};

/// Changes a wallet bookmark description.
#[command(name = "wallet_bookmark_description_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletBookmarkDescriptionChangeCommand {
    pub wallet_bookmark_id: WalletBookmarkId,
    pub description: Option<WalletBookmarkDescription>,
}
