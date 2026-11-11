use banking_ledger_domain::currency::{CurrencyAmount, CurrencyDecimals};
use banking_ledger_domain::deposit::DepositId;
use banking_ledger_domain::onchain::{
    ChainNetwork, OnchainTransactionId, TokenAddress, TokenOwnerAddress,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositSettlementVerificationRequest {
    pub deposit_id: DepositId,
    pub currency_decimals: CurrencyDecimals,
    pub chain_network: ChainNetwork,
    pub token_address: TokenAddress,
    pub token_owner_address: TokenOwnerAddress,
    pub amount: CurrencyAmount,
    pub transaction_id: OnchainTransactionId,
}
