use serde::{Deserialize, Serialize};

use super::{MintAccountSeed, MintMetadataDocument};

/// Request to publish off-chain mint metadata.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintMetadataPublishRequest {
    seed: MintAccountSeed,
    document: MintMetadataDocument,
}

impl MintMetadataPublishRequest {
    pub fn new(seed: MintAccountSeed, document: MintMetadataDocument) -> Self {
        Self { seed, document }
    }

    pub fn seed(&self) -> &MintAccountSeed {
        &self.seed
    }

    pub fn document(&self) -> &MintMetadataDocument {
        &self.document
    }
}
