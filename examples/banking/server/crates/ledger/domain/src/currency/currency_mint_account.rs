use serde::{Deserialize, Serialize};

use super::{CurrencyMintAccountAddress, CurrencyPoolTokenAccountAddress, CurrencyTokenProgramId};

/// Represents the on-chain mint account linked to a currency.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CurrencyMintAccount {
    mint_account_address: CurrencyMintAccountAddress,
    pool_token_account_address: CurrencyPoolTokenAccountAddress,
    token_program_id: CurrencyTokenProgramId,
}

impl CurrencyMintAccount {
    /// Creates a currency mint account reference.
    pub fn new(
        mint_account_address: CurrencyMintAccountAddress,
        pool_token_account_address: CurrencyPoolTokenAccountAddress,
        token_program_id: CurrencyTokenProgramId,
    ) -> Self {
        Self {
            mint_account_address,
            pool_token_account_address,
            token_program_id,
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

    /// Returns the token program ID.
    pub fn token_program_id(&self) -> &CurrencyTokenProgramId {
        &self.token_program_id
    }
}
