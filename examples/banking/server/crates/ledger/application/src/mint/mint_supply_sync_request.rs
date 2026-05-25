use serde::{Deserialize, Serialize};

use super::MintAccountSeed;

/// Request to synchronize on-chain mint supply into the internal pool account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintSupplySyncRequest {
    seed: MintAccountSeed,
    decimals: u8,
    target_supply: u128,
}

impl MintSupplySyncRequest {
    pub fn new(seed: MintAccountSeed, decimals: u8, target_supply: u128) -> Self {
        Self {
            seed,
            decimals,
            target_supply,
        }
    }

    pub fn seed(&self) -> &MintAccountSeed {
        &self.seed
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn target_supply(&self) -> u128 {
        self.target_supply
    }
}
