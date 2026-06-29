use serde::{Deserialize, Serialize};

use super::{MintAccountDecimals, MintAccountMetadata, MintId};

/// Request to provision an on-chain mint for application use.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintProvisionRequest {
    mint_id: MintId,
    decimals: MintAccountDecimals,
    metadata: MintAccountMetadata,
}

impl MintProvisionRequest {
    pub fn new(
        mint_id: MintId,
        decimals: MintAccountDecimals,
        metadata: MintAccountMetadata,
    ) -> Self {
        Self {
            mint_id,
            decimals,
            metadata,
        }
    }

    pub fn mint_id(&self) -> &MintId {
        &self.mint_id
    }

    pub fn decimals(&self) -> MintAccountDecimals {
        self.decimals
    }

    pub fn metadata(&self) -> &MintAccountMetadata {
        &self.metadata
    }
}
