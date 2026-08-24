use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use banking_ledger_domain::currency::{Currency, CurrencyError};
use banking_ledger_domain::deposit::{Deposit, DepositError};
use banking_ledger_domain::token_binding::{TokenBinding, TokenBindingError};
use thiserror::Error;

use crate::settlement::DepositSettlementPreparerError;

/// Represents errors returned while preparing a deposit settlement.
#[derive(Debug, Error)]
pub enum DepositSettlementPrepareCommandHandlerError {
    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("deposit repository failed")]
    DepositRepository(#[from] RepositoryError<Deposit>),

    #[error("deposit aggregate failed")]
    Deposit(#[from] DepositError),

    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("token binding repository failed")]
    TokenBindingRepository(#[from] RepositoryError<TokenBinding>),

    #[error("token binding aggregate failed")]
    TokenBinding(#[from] TokenBindingError),

    #[error("pool token deposit preparer failed")]
    DepositSettlementPreparer(#[from] DepositSettlementPreparerError),
}

impl Retryability for DepositSettlementPrepareCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::AccountRepository(error) => error.is_retryable(),
            Self::Account(_) => false,
            Self::DepositRepository(error) => error.is_retryable(),
            Self::Deposit(_) => false,
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::Currency(_) => false,
            Self::TokenBindingRepository(error) => error.is_retryable(),
            Self::TokenBinding(_) => false,
            Self::DepositSettlementPreparer(error) => error.is_retryable(),
        }
    }
}
