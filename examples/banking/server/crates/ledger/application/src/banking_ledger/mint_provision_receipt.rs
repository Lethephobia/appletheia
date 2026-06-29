use serde::{Deserialize, Serialize};

use super::{MintAccountAddress, PoolTokenAccountAddress};

/// Receipt returned after provisioning an on-chain mint.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintProvisionReceipt {
    mint_account_address: MintAccountAddress,
    pool_token_account_address: PoolTokenAccountAddress,
}

impl MintProvisionReceipt {
    pub fn new(
        mint_account_address: MintAccountAddress,
        pool_token_account_address: PoolTokenAccountAddress,
    ) -> Self {
        Self {
            mint_account_address,
            pool_token_account_address,
        }
    }

    pub fn mint_account_address(&self) -> &MintAccountAddress {
        &self.mint_account_address
    }

    pub fn pool_token_account_address(&self) -> &PoolTokenAccountAddress {
        &self.pool_token_account_address
    }
}
