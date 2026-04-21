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

    #[error("transfer id is missing after request")]
    MissingTransferId,

    #[error("source account was not found")]
    SourceAccountNotFound,

    #[error("destination account was not found")]
    DestinationAccountNotFound,

    #[error("source and destination accounts use different currencies")]
    CurrencyMismatch,
}
