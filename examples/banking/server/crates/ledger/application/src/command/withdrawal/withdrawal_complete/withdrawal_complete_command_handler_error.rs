use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalError};
use thiserror::Error;

/// Represents errors returned while completing a withdrawal.
#[derive(Debug, Error)]
pub enum WithdrawalCompleteCommandHandlerError {
    #[error("withdrawal repository failed")]
    WithdrawalRepository(#[from] RepositoryError<Withdrawal>),

    #[error("withdrawal aggregate failed")]
    Withdrawal(#[from] WithdrawalError),

    #[error("withdrawal was not found")]
    WithdrawalNotFound,
}
