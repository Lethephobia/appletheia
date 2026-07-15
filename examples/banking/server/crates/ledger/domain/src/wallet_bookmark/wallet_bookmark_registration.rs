use crate::core::TokenAccountOwnerAddress;

use super::{WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkOwner};

/// Describes a wallet bookmark registration request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalletBookmarkRegistration {
    pub owner: WalletBookmarkOwner,
    pub display_name: Option<WalletBookmarkDisplayName>,
    pub description: Option<WalletBookmarkDescription>,
    pub token_account_owner_address: TokenAccountOwnerAddress,
}

impl WalletBookmarkRegistration {
    pub(super) fn into_parts(
        self,
    ) -> (
        WalletBookmarkOwner,
        Option<WalletBookmarkDisplayName>,
        Option<WalletBookmarkDescription>,
        TokenAccountOwnerAddress,
    ) {
        (
            self.owner,
            self.display_name,
            self.description,
            self.token_account_owner_address,
        )
    }
}
