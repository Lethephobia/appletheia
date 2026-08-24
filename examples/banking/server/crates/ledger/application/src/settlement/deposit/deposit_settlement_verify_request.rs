use banking_ledger_domain::core::{
    ChainNetwork, CurrencyAmount, CurrencyDecimals, OnchainTransactionId, TokenAddress,
};
use banking_ledger_domain::deposit::DepositId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositSettlementVerifyRequest {
    pub deposit_id: DepositId,
    pub currency_decimals: CurrencyDecimals,
    pub chain_network: ChainNetwork,
    pub token_address: TokenAddress,
    pub amount: CurrencyAmount,
    pub transaction_id: OnchainTransactionId,
}
