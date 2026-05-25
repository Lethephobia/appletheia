use serde::{Deserialize, Serialize};

use super::{MintAccountAddress, OnchainAccountAddress, TokenProgramId};

/// Receipt returned after creating or retrieving an on-chain mint account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintAccountCreateReceipt {
    address: MintAccountAddress,
    pool_address: OnchainAccountAddress,
    token_program_id: TokenProgramId,
}

impl MintAccountCreateReceipt {
    pub fn new(
        address: MintAccountAddress,
        pool_address: OnchainAccountAddress,
        token_program_id: TokenProgramId,
    ) -> Self {
        Self {
            address,
            pool_address,
            token_program_id,
        }
    }

    pub fn address(&self) -> &MintAccountAddress {
        &self.address
    }

    pub fn pool_address(&self) -> &OnchainAccountAddress {
        &self.pool_address
    }

    pub fn token_program_id(&self) -> &TokenProgramId {
        &self.token_program_id
    }
}
