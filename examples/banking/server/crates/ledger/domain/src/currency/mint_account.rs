use serde::{Deserialize, Serialize};

use super::{MintAccountAddress, PoolTokenAccountAddress};

/// Represents the on-chain mint account linked to a currency.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MintAccount {
    mint_account_address: MintAccountAddress,
    pool_token_account_address: PoolTokenAccountAddress,
}

impl MintAccount {
    /// Creates a currency mint account reference.
    pub fn new(
        mint_account_address: MintAccountAddress,
        pool_token_account_address: PoolTokenAccountAddress,
    ) -> Self {
        Self {
            mint_account_address,
            pool_token_account_address,
        }
    }

    /// Returns the mint account address.
    pub fn mint_account_address(&self) -> &MintAccountAddress {
        &self.mint_account_address
    }

    /// Returns the pool token account address.
    pub fn pool_token_account_address(&self) -> &PoolTokenAccountAddress {
        &self.pool_token_account_address
    }
}
