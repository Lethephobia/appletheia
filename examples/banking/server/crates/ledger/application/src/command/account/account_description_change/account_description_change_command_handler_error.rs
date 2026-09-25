use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountDescriptionChangeCommandHandlerError {
    #[error("account repository failed")]
    Repository(#[from] RepositoryError<Account>),
    #[error("account aggregate failed")]
    Aggregate(#[from] AccountError),
}

impl Retryability for AccountDescriptionChangeCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Repository(error) => error.is_retryable(),
            Self::Aggregate(_) => false,
        }
    }
}
