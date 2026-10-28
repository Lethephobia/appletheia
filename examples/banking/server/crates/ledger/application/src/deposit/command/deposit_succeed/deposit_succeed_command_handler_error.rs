use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::deposit::{Deposit, DepositError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DepositSucceedCommandHandlerError {
    #[error("deposit repository failed")]
    DepositRepository(#[from] RepositoryError<Deposit>),

    #[error("deposit aggregate failed")]
    Deposit(#[from] DepositError),
}

impl Retryability for DepositSucceedCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::DepositRepository(error) => error.is_retryable(),
            Self::Deposit(_) => false,
        }
    }
}
