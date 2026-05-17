use serde::{Deserialize, Serialize};

use super::{MintAccountMetadata, MintAccountSeed, OnchainAccountAddress, TokenProgramId};

/// Request to create or retrieve an on-chain mint account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintAccountCreateRequest {
    seed: MintAccountSeed,
    decimals: u8,
    token_program_id: TokenProgramId,
    mint_authority: OnchainAccountAddress,
    freeze_authority: Option<OnchainAccountAddress>,
    metadata: MintAccountMetadata,
}

impl MintAccountCreateRequest {
    pub fn new(
        seed: MintAccountSeed,
        decimals: u8,
        token_program_id: TokenProgramId,
        mint_authority: OnchainAccountAddress,
        freeze_authority: Option<OnchainAccountAddress>,
        metadata: MintAccountMetadata,
    ) -> Self {
        Self {
            seed,
            decimals,
            token_program_id,
            mint_authority,
            freeze_authority,
            metadata,
        }
    }

    pub fn seed(&self) -> &MintAccountSeed {
        &self.seed
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn token_program_id(&self) -> &TokenProgramId {
        &self.token_program_id
    }

    pub fn mint_authority(&self) -> &OnchainAccountAddress {
        &self.mint_authority
    }

    pub fn freeze_authority(&self) -> Option<&OnchainAccountAddress> {
        self.freeze_authority.as_ref()
    }

    pub fn metadata(&self) -> &MintAccountMetadata {
        &self.metadata
    }
}
