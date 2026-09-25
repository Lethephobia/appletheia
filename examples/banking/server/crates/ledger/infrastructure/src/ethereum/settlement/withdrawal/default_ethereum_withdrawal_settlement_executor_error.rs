use alloy::contract::Error as ContractError;
use alloy::providers::PendingTransactionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DefaultEthereumWithdrawalSettlementExecutorError {
    #[error("Ethereum settlement contract call failed")]
    Contract(#[source] ContractError),
    #[error("Ethereum withdrawal transaction failed")]
    PendingTransaction(#[source] PendingTransactionError),
}
