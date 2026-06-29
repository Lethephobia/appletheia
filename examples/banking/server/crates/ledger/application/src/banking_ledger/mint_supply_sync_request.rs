use serde::{Deserialize, Serialize};

use super::{MintAccountDecimals, MintId, TokenAmount};

/// Request to synchronize on-chain mint supply into the internal pool account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintSupplySyncRequest {
    mint_id: MintId,
    decimals: MintAccountDecimals,
    target_supply: TokenAmount,
}

impl MintSupplySyncRequest {
    pub fn new(mint_id: MintId, decimals: MintAccountDecimals, target_supply: TokenAmount) -> Self {
        Self {
            mint_id,
            decimals,
            target_supply,
        }
    }

    pub fn mint_id(&self) -> &MintId {
        &self.mint_id
    }

    pub fn decimals(&self) -> MintAccountDecimals {
        self.decimals
    }

    pub fn target_supply(&self) -> TokenAmount {
        self.target_supply
    }
}
