use serde::{Deserialize, Serialize};

use super::{MintAccountDecimals, MintAccountSeed, TokenAmount};

/// Request to synchronize on-chain mint supply into the internal pool account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintSupplySyncRequest {
    seed: MintAccountSeed,
    decimals: MintAccountDecimals,
    target_supply: TokenAmount,
}

impl MintSupplySyncRequest {
    pub fn new(
        seed: MintAccountSeed,
        decimals: MintAccountDecimals,
        target_supply: TokenAmount,
    ) -> Self {
        Self {
            seed,
            decimals,
            target_supply,
        }
    }

    pub fn seed(&self) -> &MintAccountSeed {
        &self.seed
    }

    pub fn decimals(&self) -> MintAccountDecimals {
        self.decimals
    }

    pub fn target_supply(&self) -> TokenAmount {
        self.target_supply
    }
}
