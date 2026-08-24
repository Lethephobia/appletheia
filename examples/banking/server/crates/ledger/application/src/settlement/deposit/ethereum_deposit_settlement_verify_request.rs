use banking_ledger_domain::core::{
    CurrencyAmount, CurrencyDecimals, EthereumNetwork, EvmTokenContractAddress, EvmTransactionHash,
};
use banking_ledger_domain::deposit::DepositId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumDepositSettlementVerifyRequest {
    deposit_id: DepositId,
    currency_decimals: CurrencyDecimals,
    network: EthereumNetwork,
    token_address: EvmTokenContractAddress,
    amount: CurrencyAmount,
    transaction_id: EvmTransactionHash,
}

impl EthereumDepositSettlementVerifyRequest {
    pub fn new(
        deposit_id: DepositId,
        currency_decimals: CurrencyDecimals,
        network: EthereumNetwork,
        token_address: EvmTokenContractAddress,
        amount: CurrencyAmount,
        transaction_id: EvmTransactionHash,
    ) -> Self {
        Self {
            deposit_id,
            currency_decimals,
            network,
            token_address,
            amount,
            transaction_id,
        }
    }

    pub fn deposit_id(&self) -> DepositId {
        self.deposit_id
    }

    pub const fn currency_decimals(&self) -> CurrencyDecimals {
        self.currency_decimals
    }

    pub const fn network(&self) -> EthereumNetwork {
        self.network
    }

    pub const fn token_address(&self) -> EvmTokenContractAddress {
        self.token_address
    }

    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }

    pub const fn transaction_id(&self) -> EvmTransactionHash {
        self.transaction_id
    }
}
