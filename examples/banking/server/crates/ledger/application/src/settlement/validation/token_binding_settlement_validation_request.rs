use banking_ledger_domain::currency::CurrencyDecimals;
use banking_ledger_domain::onchain::{ChainNetwork, TokenAddress};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenBindingSettlementValidationRequest {
    pub currency_decimals: CurrencyDecimals,
    pub chain_network: ChainNetwork,
    pub token_address: TokenAddress,
}
