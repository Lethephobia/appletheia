use banking_ledger_domain::core::{
    ChainNetwork, CurrencyAmount, CurrencyDecimals, TokenAddress, TokenOwnerAddress,
};
use banking_ledger_domain::withdrawal::WithdrawalId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawalSettlementRequest {
    withdrawal_id: WithdrawalId,
    currency_decimals: CurrencyDecimals,
    chain_network: ChainNetwork,
    token_address: TokenAddress,
    token_owner_address: TokenOwnerAddress,
    amount: CurrencyAmount,
}

impl WithdrawalSettlementRequest {
    pub fn new(
        withdrawal_id: WithdrawalId,
        currency_decimals: CurrencyDecimals,
        chain_network: ChainNetwork,
        token_address: TokenAddress,
        token_owner_address: TokenOwnerAddress,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            withdrawal_id,
            currency_decimals,
            chain_network,
            token_address,
            token_owner_address,
            amount,
        }
    }

    pub fn withdrawal_id(&self) -> WithdrawalId {
        self.withdrawal_id
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
}
