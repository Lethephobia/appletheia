use serde::{Deserialize, Serialize};

use super::{MintAccountDecimals, MintAccountMetadata, MintAccountSeed};

/// Request to create or retrieve an on-chain mint account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintAccountCreateRequest {
    seed: MintAccountSeed,
    decimals: MintAccountDecimals,
    metadata: MintAccountMetadata,
}

impl MintAccountCreateRequest {
    pub fn new(
        seed: MintAccountSeed,
        decimals: MintAccountDecimals,
        metadata: MintAccountMetadata,
    ) -> Self {
        Self {
            seed,
            decimals,
            metadata,
        }
    }

    pub fn seed(&self) -> &MintAccountSeed {
        &self.seed
    }

    pub fn decimals(&self) -> MintAccountDecimals {
        self.decimals
    }

    pub fn metadata(&self) -> &MintAccountMetadata {
        &self.metadata
    }
}
