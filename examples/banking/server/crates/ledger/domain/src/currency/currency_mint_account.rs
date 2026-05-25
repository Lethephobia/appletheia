use serde::{Deserialize, Serialize};

use super::{CurrencyMintAccountAddress, CurrencyMintTokenProgramId, CurrencyPoolAccountAddress};

/// Represents the on-chain mint account linked to a currency.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CurrencyMintAccount {
    address: CurrencyMintAccountAddress,
    pool_address: CurrencyPoolAccountAddress,
    token_program_id: CurrencyMintTokenProgramId,
}

impl CurrencyMintAccount {
    /// Creates a currency mint account reference.
    pub fn new(
        address: CurrencyMintAccountAddress,
        pool_address: CurrencyPoolAccountAddress,
        token_program_id: CurrencyMintTokenProgramId,
    ) -> Self {
        Self {
            address,
            pool_address,
            token_program_id,
        }
    }

    /// Returns the mint account address.
    pub fn address(&self) -> &CurrencyMintAccountAddress {
        &self.address
    }

    /// Returns the pool token account address.
    pub fn pool_address(&self) -> &CurrencyPoolAccountAddress {
        &self.pool_address
    }

    /// Returns the token program ID.
    pub fn token_program_id(&self) -> &CurrencyMintTokenProgramId {
        &self.token_program_id
    }
}
