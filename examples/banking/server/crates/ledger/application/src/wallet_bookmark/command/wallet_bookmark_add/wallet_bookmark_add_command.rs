use appletheia::command;
use banking_ledger_domain::core::TokenOwnerAddress;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkOwner,
};
use serde::{Deserialize, Serialize};

/// Adds a wallet bookmark.
#[command(name = "wallet_bookmark_add")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletBookmarkAddCommand {
    pub owner: WalletBookmarkOwner,
    pub display_name: Option<WalletBookmarkDisplayName>,
    pub description: Option<WalletBookmarkDescription>,
    pub token_owner_address: TokenOwnerAddress,
}
