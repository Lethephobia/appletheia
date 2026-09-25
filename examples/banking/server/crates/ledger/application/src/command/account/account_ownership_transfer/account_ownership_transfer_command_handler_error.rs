use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use thiserror::Error;

/// Represents errors returned while transferring account ownership.
#[derive(Debug, Error)]
pub enum AccountOwnershipTransferCommandHandlerError {
    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),
}

impl Retryability for AccountOwnershipTransferCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::AccountRepository(error) => error.is_retryable(),
            Self::Account(_) => false,
        }
    }
}
