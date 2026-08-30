use banking_ledger_domain::core::{
    ChainNetwork, CurrencyAmount, CurrencyDecimals, TokenAddress, TokenOwnerAddress,
};
use banking_ledger_domain::deposit::DepositId;

use super::EvmDepositAuthorization;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositSettlementPrepareRequest {
    deposit_id: DepositId,
    currency_decimals: CurrencyDecimals,
    chain_network: ChainNetwork,
    token_address: TokenAddress,
    token_owner_address: TokenOwnerAddress,
    amount: CurrencyAmount,
    evm_authorization: Option<EvmDepositAuthorization>,
}

impl DepositSettlementPrepareRequest {
    pub fn new(
        deposit_id: DepositId,
        currency_decimals: CurrencyDecimals,
        chain_network: ChainNetwork,
        token_address: TokenAddress,
        token_owner_address: TokenOwnerAddress,
        amount: CurrencyAmount,
        evm_authorization: Option<EvmDepositAuthorization>,
    ) -> Self {
        Self {
            deposit_id,
            currency_decimals,
            chain_network,
            token_address,
            token_owner_address,
            amount,
            evm_authorization,
        }
    }

    pub fn deposit_id(&self) -> DepositId {
        self.deposit_id
    }

    pub const fn currency_decimals(&self) -> CurrencyDecimals {
        self.currency_decimals
    }

    pub const fn chain_network(&self) -> ChainNetwork {
        self.chain_network
    }

    pub fn token_address(&self) -> &TokenAddress {
        &self.token_address
    }

    pub fn token_owner_address(&self) -> &TokenOwnerAddress {
        &self.token_owner_address
    }

    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }

    pub const fn evm_authorization(&self) -> Option<EvmDepositAuthorization> {
        self.evm_authorization
    }
}
