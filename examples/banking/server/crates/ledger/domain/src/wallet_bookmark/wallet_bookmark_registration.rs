use crate::core::TokenOwnerAddress;

use super::{WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkOwner};

/// Describes a wallet bookmark registration request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalletBookmarkRegistration {
    pub owner: WalletBookmarkOwner,
    pub display_name: Option<WalletBookmarkDisplayName>,
    pub description: Option<WalletBookmarkDescription>,
    pub token_owner_address: TokenOwnerAddress,
}

impl WalletBookmarkRegistration {
    pub(super) fn into_parts(
        self,
    ) -> (
        WalletBookmarkOwner,
        Option<WalletBookmarkDisplayName>,
        Option<WalletBookmarkDescription>,
        TokenOwnerAddress,
    ) {
        (
            self.owner,
            self.display_name,
            self.description,
            self.token_owner_address,
        )
    }
}
