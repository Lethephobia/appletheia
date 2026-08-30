use alloy::contract::Error as ContractError;
use alloy::signers::Error as SignerError;
use alloy::transports::TransportError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DefaultEthereumUserOperationPreparerError {
    #[error("Ethereum RPC failed")]
    Rpc(#[source] TransportError),
    #[error("Ethereum account abstraction contract call failed")]
    Contract(#[source] ContractError),
    #[error("sponsorship signature failed")]
    Signer(#[source] SignerError),
    #[error("sponsorship signature deadline overflowed")]
    SponsorshipDeadlineOverflow,
    #[error("sponsorship signature TTL must produce a future deadline")]
    InvalidSponsorshipSignatureTtl,
    #[error("sponsorship signature deadline is before the Unix epoch")]
    SponsorshipDeadlineBeforeUnixEpoch,
    #[error("sponsorship signature deadline exceeds uint48")]
    SponsorshipDeadlineExceedsUint48,
    #[error("the sender has not delegated to the configured account implementation")]
    AccountNotDelegated,
    #[error("ERC-4337 gas value exceeds the supported 128-bit packed field")]
    GasLimitOverflow,
}
