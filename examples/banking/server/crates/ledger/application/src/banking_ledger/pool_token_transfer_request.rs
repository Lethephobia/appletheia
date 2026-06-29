use super::{
    MintAccountAddress, MintAccountDecimals, PoolTokenAccountAddress, PoolTokenTransferMarkerSeed,
    TokenAccountOwnerAddress, TokenAmount,
};

/// Input for an external pool token transfer execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolTokenTransferRequest {
    marker_seed: PoolTokenTransferMarkerSeed,
    mint_account_address: MintAccountAddress,
    pool_token_account_address: PoolTokenAccountAddress,
    destination_token_account_owner_address: TokenAccountOwnerAddress,
    amount: TokenAmount,
    decimals: MintAccountDecimals,
}

impl PoolTokenTransferRequest {
    pub fn new(
        marker_seed: PoolTokenTransferMarkerSeed,
        mint_account_address: MintAccountAddress,
        pool_token_account_address: PoolTokenAccountAddress,
        destination_token_account_owner_address: TokenAccountOwnerAddress,
        amount: TokenAmount,
        decimals: MintAccountDecimals,
    ) -> Self {
        Self {
            marker_seed,
            mint_account_address,
            pool_token_account_address,
            destination_token_account_owner_address,
            amount,
            decimals,
        }
    }

    pub fn marker_seed(&self) -> &PoolTokenTransferMarkerSeed {
        &self.marker_seed
    }

    pub fn mint_account_address(&self) -> &MintAccountAddress {
        &self.mint_account_address
    }

    pub fn pool_token_account_address(&self) -> &PoolTokenAccountAddress {
        &self.pool_token_account_address
    }

    pub fn destination_token_account_owner_address(&self) -> &TokenAccountOwnerAddress {
        &self.destination_token_account_owner_address
    }

    pub fn amount(&self) -> &TokenAmount {
        &self.amount
    }

    pub fn decimals(&self) -> &MintAccountDecimals {
        &self.decimals
    }
}
