use alloy::contract::Error as ContractError;
use alloy::transports::TransportError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DefaultEthereumDepositSettlementVerifierError {
    #[error("Ethereum RPC failed")]
    Rpc(#[source] TransportError),
    #[error("Ethereum settlement contract call failed")]
    Contract(#[source] ContractError),
    #[error("deposit transaction was not found")]
    TransactionNotFound,
    #[error("deposit transaction failed")]
    TransactionFailed,
    #[error("deposit transaction sender does not match the token owner")]
    UnexpectedSender,
    #[error("deposit transaction receiver does not match the settlement contract")]
    UnexpectedReceiver,
    #[error("recorded deposit settlement does not match the request")]
    SettlementMismatch,
}
