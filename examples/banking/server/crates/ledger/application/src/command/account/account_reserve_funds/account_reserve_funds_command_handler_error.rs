use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use thiserror::Error;

/// Represents errors returned while reserving funds in an account.
#[derive(Debug, Error)]
pub enum AccountReserveFundsCommandHandlerError {
    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("account was not found")]
    AccountNotFound,
}
