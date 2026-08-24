use banking_ledger_domain::core::{CurrencyDecimals, SolanaMintAccountAddress, SolanaNetwork};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaTokenBindingSettlementValidationRequest {
    currency_decimals: CurrencyDecimals,
    network: SolanaNetwork,
    token_address: SolanaMintAccountAddress,
}

impl SolanaTokenBindingSettlementValidationRequest {
    pub fn new(
        currency_decimals: CurrencyDecimals,
        network: SolanaNetwork,
        token_address: SolanaMintAccountAddress,
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

    pub const fn network(&self) -> SolanaNetwork {
        self.network
    }

    pub const fn token_address(&self) -> SolanaMintAccountAddress {
        self.token_address
    }
}
