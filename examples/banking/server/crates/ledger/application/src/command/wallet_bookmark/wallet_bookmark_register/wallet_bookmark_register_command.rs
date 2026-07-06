use appletheia::command;
use banking_ledger_domain::core::TokenAccountOwnerAddress;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkOwner,
};
use serde::{Deserialize, Serialize};

/// Registers a wallet bookmark.
#[command(name = "wallet_bookmark_register")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletBookmarkRegisterCommand {
    pub owner: WalletBookmarkOwner,
    pub display_name: Option<WalletBookmarkDisplayName>,
    pub description: Option<WalletBookmarkDescription>,
    pub token_account_owner_address: TokenAccountOwnerAddress,
}
