use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::{
    account::{Account, AccountError},
    currency::{Currency, CurrencyError},
    token_binding::{TokenBinding, TokenBindingError},
    withdrawal::{Withdrawal, WithdrawalError},
};
use thiserror::Error;

use crate::settlement::WithdrawalSettlementExecutorError;

#[derive(Debug, Error)]
pub enum WithdrawalSettlementExecuteCommandHandlerError {
    #[error("withdrawal repository failed")]
    WithdrawalRepository(#[from] RepositoryError<Withdrawal>),

    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("token binding repository failed")]
    TokenBindingRepository(#[from] RepositoryError<TokenBinding>),

    #[error("token binding aggregate failed")]
    TokenBinding(#[from] TokenBindingError),

    #[error("withdrawal aggregate failed")]
    Withdrawal(#[from] WithdrawalError),

    #[error("withdrawal settlement executor failed")]
    WithdrawalSettlementExecutor(#[from] WithdrawalSettlementExecutorError),
}

impl Retryability for WithdrawalSettlementExecuteCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::WithdrawalRepository(error) => error.is_retryable(),
            Self::AccountRepository(error) => error.is_retryable(),
            Self::Account(_) => false,
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::Currency(_) => false,
            Self::TokenBindingRepository(error) => error.is_retryable(),
            Self::TokenBinding(_) => false,
            Self::Withdrawal(_) => false,
            Self::WithdrawalSettlementExecutor(error) => error.is_retryable(),
        }
    }
}
