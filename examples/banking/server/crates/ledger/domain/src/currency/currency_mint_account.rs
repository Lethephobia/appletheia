use serde::{Deserialize, Serialize};

use super::{CurrencyMintAccountAddress, CurrencyPoolTokenAccountAddress};

/// Represents the on-chain mint account linked to a currency.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CurrencyMintAccount {
    mint_account_address: CurrencyMintAccountAddress,
    pool_token_account_address: CurrencyPoolTokenAccountAddress,
}

impl CurrencyMintAccount {
    /// Creates a currency mint account reference.
    pub fn new(
        mint_account_address: CurrencyMintAccountAddress,
        pool_token_account_address: CurrencyPoolTokenAccountAddress,
    ) -> Self {
        Self {
            mint_account_address,
            pool_token_account_address,
        }
    }

    /// Returns the mint account address.
    pub fn mint_account_address(&self) -> &CurrencyMintAccountAddress {
        &self.mint_account_address
    }

    /// Returns the pool token account address.
    pub fn pool_token_account_address(&self) -> &CurrencyPoolTokenAccountAddress {
        &self.pool_token_account_address
    }
}
