use banking_ledger_domain::wallet_bookmark::WalletBookmarkId;
use serde::{Deserialize, Serialize};

/// Returned after a wallet bookmark registration request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletBookmarkRegisterOutput {
    pub wallet_bookmark_id: WalletBookmarkId,
}

impl WalletBookmarkRegisterOutput {
    pub fn new(wallet_bookmark_id: WalletBookmarkId) -> Self {
        Self { wallet_bookmark_id }
    }
}
