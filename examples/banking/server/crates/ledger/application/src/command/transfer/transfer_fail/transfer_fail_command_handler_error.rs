use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::transfer::{Transfer, TransferError};
use thiserror::Error;

/// Represents errors returned while failing a transfer.
#[derive(Debug, Error)]
pub enum TransferFailCommandHandlerError {
    #[error("transfer repository failed")]
    TransferRepository(#[from] RepositoryError<Transfer>),

    #[error("transfer aggregate failed")]
    Transfer(#[from] TransferError),

    #[error("transfer was not found")]
    TransferNotFound,
}
