use banking_ledger_domain::core::TokenOwnerAddress;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId, WalletBookmarkOwner,
};

/// Describes a wallet bookmark fragment item upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletBookmarkFragmentUpsert {
    pub id: WalletBookmarkId,
    pub owner: WalletBookmarkOwner,
    pub display_name: Option<WalletBookmarkDisplayName>,
    pub description: Option<WalletBookmarkDescription>,
    pub token_owner_address: TokenOwnerAddress,
}
