use serde::{Deserialize, Serialize};

use super::{MintAccountMetadata, MintId};

/// Request to update metadata for an existing on-chain mint account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintAccountMetadataUpdateRequest {
    mint_id: MintId,
    metadata: MintAccountMetadata,
}

impl MintAccountMetadataUpdateRequest {
    pub fn new(mint_id: MintId, metadata: MintAccountMetadata) -> Self {
        Self { mint_id, metadata }
    }

    pub fn mint_id(&self) -> &MintId {
        &self.mint_id
    }

    pub fn metadata(&self) -> &MintAccountMetadata {
        &self.metadata
    }
}
