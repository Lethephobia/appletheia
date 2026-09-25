use alloy::contract::Error as ContractError;
use alloy::transports::TransportError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DefaultEthereumTokenBindingSettlementValidatorError {
    #[error("Ethereum RPC failed")]
    Rpc(#[source] TransportError),
    #[error("Ethereum token contract call failed")]
    Contract(#[source] ContractError),
}
