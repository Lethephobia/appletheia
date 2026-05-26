use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use banking_ledger_domain::payout_destination::{PayoutDestination, PayoutDestinationError};
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalError};
use thiserror::Error;

use crate::mint::{
    MintAccountAddressError, PoolTokenAccountAddressError, PoolTokenTransferExecutorError,
    PoolTokenTransferMarkerSeedError, TokenAccountOwnerAddressError, TokenProgramIdError,
};

/// Represents errors returned while executing a withdrawal pool token transfer.
#[derive(Debug, Error)]
pub enum WithdrawalTokenTransferCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("payout destination repository failed")]
    PayoutDestinationRepository(#[from] RepositoryError<PayoutDestination>),

    #[error("payout destination aggregate failed")]
    PayoutDestination(#[from] PayoutDestinationError),

    #[error("withdrawal repository failed")]
    WithdrawalRepository(#[from] RepositoryError<Withdrawal>),

    #[error("withdrawal aggregate failed")]
    Withdrawal(#[from] WithdrawalError),

    #[error("mint account address is invalid")]
    MintAccountAddress(#[from] MintAccountAddressError),

    #[error("pool token account address is invalid")]
    PoolTokenAccountAddress(#[from] PoolTokenAccountAddressError),

    #[error("pool token transfer marker seed is invalid")]
    PoolTokenTransferMarkerSeed(#[from] PoolTokenTransferMarkerSeedError),

    #[error("token program ID is invalid")]
    TokenProgramId(#[from] TokenProgramIdError),

    #[error("token account owner address is invalid")]
    TokenAccountOwnerAddress(#[from] TokenAccountOwnerAddressError),

    #[error("pool token transfer executor failed")]
    PoolTokenTransferExecutor(#[from] PoolTokenTransferExecutorError),

    #[error("withdrawal was not found")]
    WithdrawalNotFound,

    #[error("payout destination was not found")]
    PayoutDestinationNotFound,

    #[error("currency was not found")]
    CurrencyNotFound,

    #[error("currency is not provisioned for on-chain transfer")]
    CurrencyUnprovisioned,

    #[error("pool token transfer executor returned an invalid on-chain transaction ID")]
    InvalidOnchainTransactionId,
}
