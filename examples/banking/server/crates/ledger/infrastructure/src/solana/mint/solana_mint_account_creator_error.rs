use banking_ledger_application::{MintAccountAddressError, TokenProgramIdError};
use solana_client::client_error::ClientError as SolanaRpcClientError;
use solana_sdk::{program_error::ProgramError, pubkey::PubkeyError, signer::SignerError};
use thiserror::Error;

/// Represents Solana adapter errors while creating a mint account.
#[derive(Debug, Error)]
pub enum SolanaMintAccountCreatorError {
    #[error("Solana mint token program ID is invalid")]
    InvalidTokenProgramId(#[source] TokenProgramIdError),

    #[error("Solana mint account address could not be derived from seed")]
    MintAccountAddressDerivation(#[source] PubkeyError),

    #[error("Solana pool token account address could not be derived from seed")]
    PoolTokenAccountAddressDerivation(#[source] PubkeyError),

    #[error("Solana mint account address returned by the adapter is invalid")]
    MintAccountAddress(#[source] MintAccountAddressError),

    #[error(
        "Solana mint account already exists with unexpected owner: address={address}, owner={owner}, expected_owner={expected_owner}"
    )]
    MintAccountUnexpectedOwner {
        address: String,
        owner: String,
        expected_owner: String,
    },

    #[error(
        "Solana pool token account already exists with unexpected owner: address={address}, owner={owner}, expected_owner={expected_owner}"
    )]
    PoolTokenAccountUnexpectedOwner {
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

    #[error("Solana pool token account data is invalid: address={address}")]
    PoolTokenAccountInvalidData {
        address: String,
        #[source]
        source: ProgramError,
    },

    #[error("Solana mint account size could not be calculated")]
    MintAccountSize(#[source] ProgramError),

    #[error("Solana metadata size could not be calculated")]
    MetadataSize(#[source] ProgramError),

    #[error("Solana mint account size overflowed")]
    MintAccountSizeOverflow,

    #[error("Solana metadata pointer instruction could not be built")]
    MetadataPointerInstruction(#[source] ProgramError),

    #[error("Solana initialize mint instruction could not be built")]
    InitializeMintInstruction(#[source] ProgramError),

    #[error("Solana initialize pool account instruction could not be built")]
    InitializePoolAccountInstruction(#[source] ProgramError),

    #[error("Solana transaction signing failed")]
    Sign(#[from] SignerError),

    #[error("Solana RPC failed")]
    Rpc(#[from] SolanaRpcClientError),
}
