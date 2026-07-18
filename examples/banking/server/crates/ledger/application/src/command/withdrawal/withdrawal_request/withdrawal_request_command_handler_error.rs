use appletheia::application::Retryability;

use crate::mint::TokenAccountOwnerAddressValidatorError;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use banking_ledger_domain::currency::{Currency, CurrencyError};
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalError};
use thiserror::Error;

/// Represents errors returned while requesting a withdrawal.
#[derive(Debug, Error)]
pub enum WithdrawalRequestCommandHandlerError {
    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("withdrawal repository failed")]
    WithdrawalRepository(#[from] RepositoryError<Withdrawal>),

    #[error("withdrawal aggregate failed")]
    Withdrawal(#[from] WithdrawalError),

    #[error("token account owner address validation failed")]
    TokenAccountOwnerAddressValidator(#[from] TokenAccountOwnerAddressValidatorError),
}

impl Retryability for WithdrawalRequestCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::AccountRepository(error) => error.is_retryable(),
            Self::Account(_) => false,
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::Currency(_) => false,
            Self::WithdrawalRepository(error) => error.is_retryable(),
            Self::Withdrawal(_) => false,
            Self::TokenAccountOwnerAddressValidator(error) => error.is_retryable(),
        }
    }
}
