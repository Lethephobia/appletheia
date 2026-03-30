use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use thiserror::Error;

/// Represents errors returned while closing an account.
#[derive(Debug, Error)]
pub enum AccountCloseCommandHandlerError {
    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("account was not found")]
    AccountNotFound,
}
