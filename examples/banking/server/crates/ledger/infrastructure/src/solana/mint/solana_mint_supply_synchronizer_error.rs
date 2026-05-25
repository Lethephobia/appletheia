use solana_client::client_error::ClientError as SolanaRpcClientError;
use solana_sdk::{program_error::ProgramError, pubkey::PubkeyError, signer::SignerError};
use thiserror::Error;

/// Represents Solana adapter errors while synchronizing mint supply.
#[derive(Debug, Error)]
pub enum SolanaMintSupplySynchronizerError {
    #[error("Solana mint account address could not be derived from seed")]
    MintAccountAddressDerivation(#[source] PubkeyError),

    #[error("Solana pool token account address could not be derived from seed")]
    PoolAccountAddressDerivation(#[source] PubkeyError),

    #[error(
        "Solana mint account already exists with unexpected owner: address={address}, owner={owner}, expected_owner={expected_owner}"
    )]
    MintAccountUnexpectedOwner {
        address: String,
        owner: String,
        expected_owner: String,
    },

    #[error("Solana mint account data is invalid: address={address}")]
    MintAccountInvalidData {
        address: String,
        #[source]
        source: ProgramError,
    },

    #[error("Solana target supply exceeds the token program limit")]
    TargetSupplyOverflow,

    #[error("Solana mint-to instruction could not be built")]
    MintToInstruction(#[source] ProgramError),

    #[error("Solana burn instruction could not be built")]
    BurnInstruction(#[source] ProgramError),

    #[error("Solana transaction signing failed")]
    Sign(#[from] SignerError),

    #[error("Solana RPC failed")]
    Rpc(#[from] SolanaRpcClientError),
}
