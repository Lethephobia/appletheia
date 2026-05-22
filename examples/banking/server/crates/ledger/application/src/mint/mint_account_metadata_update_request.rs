use serde::{Deserialize, Serialize};

use super::{MintAccountMetadata, MintAccountSeed};

/// Request to update metadata for an existing on-chain mint account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintAccountMetadataUpdateRequest {
    seed: MintAccountSeed,
    metadata: MintAccountMetadata,
}

impl MintAccountMetadataUpdateRequest {
    pub fn new(seed: MintAccountSeed, metadata: MintAccountMetadata) -> Self {
        Self { seed, metadata }
    }

    pub fn seed(&self) -> &MintAccountSeed {
        &self.seed
    }

    pub fn metadata(&self) -> &MintAccountMetadata {
        &self.metadata
    }
}
