use banking_ledger_domain::core::{CurrencyDecimals, EthereumNetwork, EvmTokenContractAddress};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumTokenBindingSettlementValidationRequest {
    currency_decimals: CurrencyDecimals,
    network: EthereumNetwork,
    token_address: EvmTokenContractAddress,
}

impl EthereumTokenBindingSettlementValidationRequest {
    pub fn new(
        currency_decimals: CurrencyDecimals,
        network: EthereumNetwork,
        token_address: EvmTokenContractAddress,
    ) -> Self {
        Self {
            currency_decimals,
            network,
            token_address,
        }
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
}
