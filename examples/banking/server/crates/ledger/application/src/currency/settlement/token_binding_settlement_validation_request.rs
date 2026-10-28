use banking_ledger_domain::core::{ChainNetwork, CurrencyDecimals, TokenAddress};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenBindingSettlementValidationRequest {
    pub currency_decimals: CurrencyDecimals,
    pub chain_network: ChainNetwork,
    pub token_address: TokenAddress,
}
