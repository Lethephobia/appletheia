use serde::{Deserialize, Serialize};

use super::{MintAccountAddress, TokenProgramId};

/// Receipt returned after creating or retrieving an on-chain mint account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintAccountCreateReceipt {
    address: MintAccountAddress,
    token_program_id: TokenProgramId,
}

impl MintAccountCreateReceipt {
    pub fn new(address: MintAccountAddress, token_program_id: TokenProgramId) -> Self {
        Self {
            address,
            token_program_id,
        }
    }

    pub fn address(&self) -> &MintAccountAddress {
        &self.address
    }

    pub fn token_program_id(&self) -> &TokenProgramId {
        &self.token_program_id
    }
}
