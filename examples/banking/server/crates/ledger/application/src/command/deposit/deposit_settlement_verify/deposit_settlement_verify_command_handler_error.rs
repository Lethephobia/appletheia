use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::{
    account::{Account, AccountError},
    currency::{Currency, CurrencyError},
    deposit::{Deposit, DepositError},
    token_binding::{TokenBinding, TokenBindingError},
};
use thiserror::Error;

use crate::settlement::DepositSettlementVerifierError;

/// Represents errors returned while recording a deposit settlement.
#[derive(Debug, Error)]
pub enum DepositSettlementVerifyCommandHandlerError {
    #[error("deposit repository failed")]
    DepositRepository(#[from] RepositoryError<Deposit>),

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

    #[error("deposit aggregate failed")]
    Deposit(#[from] DepositError),

    #[error("deposit settlement verification failed")]
    DepositSettlementVerifier(#[from] DepositSettlementVerifierError),
}

impl Retryability for DepositSettlementVerifyCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::DepositRepository(error) => error.is_retryable(),
            Self::AccountRepository(error) => error.is_retryable(),
            Self::Account(_) => false,
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::Currency(_) => false,
            Self::TokenBindingRepository(error) => error.is_retryable(),
            Self::TokenBinding(_) | Self::Deposit(_) => false,
            Self::DepositSettlementVerifier(error) => error.is_retryable(),
        }
    }
}
