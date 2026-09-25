use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use banking_ledger_domain::transfer::{Transfer, TransferError};
use thiserror::Error;

/// Represents errors returned while requesting a transfer.
#[derive(Debug, Error)]
pub enum TransferRequestCommandHandlerError {
    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("transfer repository failed")]
    TransferRepository(#[from] RepositoryError<Transfer>),

    #[error("transfer aggregate failed")]
    Transfer(#[from] TransferError),
}

impl Retryability for TransferRequestCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::AccountRepository(error) => error.is_retryable(),
            Self::Account(_) => false,
            Self::TransferRepository(error) => error.is_retryable(),
            Self::Transfer(_) => false,
        }
    }
}
