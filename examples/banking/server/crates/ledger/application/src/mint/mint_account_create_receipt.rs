use serde::{Deserialize, Serialize};

use super::{MintAccountAddress, PoolTokenAccountAddress, TokenProgramId};

/// Receipt returned after creating or retrieving an on-chain mint account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintAccountCreateReceipt {
    mint_account_address: MintAccountAddress,
    pool_token_account_address: PoolTokenAccountAddress,
    token_program_id: TokenProgramId,
}

impl MintAccountCreateReceipt {
    pub fn new(
        mint_account_address: MintAccountAddress,
        pool_token_account_address: PoolTokenAccountAddress,
        token_program_id: TokenProgramId,
    ) -> Self {
        Self {
            mint_account_address,
            pool_token_account_address,
            token_program_id,
        }
    }

    pub fn mint_account_address(&self) -> &MintAccountAddress {
        &self.mint_account_address
    }

    pub fn pool_token_account_address(&self) -> &PoolTokenAccountAddress {
        &self.pool_token_account_address
    }

    pub fn token_program_id(&self) -> &TokenProgramId {
        &self.token_program_id
    }
}
