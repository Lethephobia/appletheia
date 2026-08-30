use appletheia::application::Retryability;
use thiserror::Error;

use super::EthereumDepositSettlementTransactionPreparerError;
use crate::settlement::EthereumUserOperationPreparerError;

#[derive(Debug, Error)]
pub enum DepositSettlementPreparerError {
    #[error("deposit settlement values belong to different chains")]
    InconsistentChainValues,
    #[error("deposit amount cannot be represented exactly by the selected token")]
    InvalidAmount,
    #[error("an EVM deposit authorization is not accepted for a Solana deposit")]
    UnexpectedEvmAuthorization,
    #[error("Ethereum deposit settlement transaction preparation failed")]
    EthereumTransaction(#[from] EthereumDepositSettlementTransactionPreparerError),
    #[error("Ethereum UserOperation preparation failed")]
    EthereumUserOperation(#[from] EthereumUserOperationPreparerError),
    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for DepositSettlementPreparerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::EthereumTransaction(error) => error.is_retryable(),
            Self::EthereumUserOperation(error) => error.is_retryable(),
            Self::Backend(_) => true,
            Self::InconsistentChainValues
            | Self::InvalidAmount
            | Self::UnexpectedEvmAuthorization => false,
        }
    }
}
