use serde::{Deserialize, Serialize};

use super::{MintId, MintMetadataDocument};

/// Request to publish off-chain mint metadata.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintMetadataPublishRequest {
    mint_id: MintId,
    document: MintMetadataDocument,
}

impl MintMetadataPublishRequest {
    pub fn new(mint_id: MintId, document: MintMetadataDocument) -> Self {
        Self { mint_id, document }
    }

    pub fn mint_id(&self) -> &MintId {
        &self.mint_id
    }

    pub fn document(&self) -> &MintMetadataDocument {
        &self.document
    }
}
