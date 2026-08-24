use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use banking_ledger_domain::token_binding::{TokenBinding, TokenBindingError};
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalError};
use thiserror::Error;

/// Represents errors returned while requesting a withdrawal.
#[derive(Debug, Error)]
pub enum WithdrawalRequestCommandHandlerError {
    #[error("withdrawal repository failed")]
    WithdrawalRepository(#[from] RepositoryError<Withdrawal>),

    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("token binding repository failed")]
    TokenBindingRepository(#[from] RepositoryError<TokenBinding>),

    #[error("token binding aggregate failed")]
    TokenBinding(#[from] TokenBindingError),

    #[error("withdrawal aggregate failed")]
    Withdrawal(#[from] WithdrawalError),
}

impl Retryability for WithdrawalRequestCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::WithdrawalRepository(error) => error.is_retryable(),
            Self::AccountRepository(error) => error.is_retryable(),
            Self::TokenBindingRepository(error) => error.is_retryable(),
            Self::Account(_) | Self::TokenBinding(_) | Self::Withdrawal(_) => false,
        }
    }
}
