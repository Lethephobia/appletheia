use alloy::contract::Error as ContractError;
use alloy::primitives::SignatureError;
use alloy::signers::Error as SignerError;
use alloy::transports::TransportError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DefaultEthereumDepositSettlementTransactionPreparerError {
    #[error("Ethereum RPC failed")]
    Rpc(#[source] TransportError),
    #[error("Ethereum settlement contract call failed")]
    Contract(#[source] ContractError),
    #[error("operator signature failed")]
    Signer(#[source] SignerError),
    #[error("invalid EVM signature")]
    InvalidSignature(#[source] SignatureError),
    #[error("operator signature deadline overflowed")]
    DeadlineOverflow,
    #[error("operator signature TTL must produce a future deadline")]
    InvalidOperatorSignatureTtl,
    #[error("operator signature deadline is before the Unix epoch")]
    DeadlineBeforeUnixEpoch,
}
