use banking_ledger_domain::core::{
    ChainNetwork, CurrencyAmount, CurrencyDecimals, OnchainTransactionId, TokenAddress,
    TokenOwnerAddress,
};
use banking_ledger_domain::deposit::DepositId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositSettlementVerifyRequest {
    pub deposit_id: DepositId,
    pub currency_decimals: CurrencyDecimals,
    pub chain_network: ChainNetwork,
    pub token_address: TokenAddress,
    pub token_owner_address: TokenOwnerAddress,
    pub amount: CurrencyAmount,
    pub transaction_id: OnchainTransactionId,
}
