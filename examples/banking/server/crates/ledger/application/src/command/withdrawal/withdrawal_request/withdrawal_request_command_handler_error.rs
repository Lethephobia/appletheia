use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use banking_ledger_domain::currency::{Currency, CurrencyError};
use banking_ledger_domain::payout_destination::{PayoutDestination, PayoutDestinationError};
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

    #[error("payout destination repository failed")]
    PayoutDestinationRepository(#[from] RepositoryError<PayoutDestination>),

    #[error("payout destination aggregate failed")]
    PayoutDestination(#[from] PayoutDestinationError),

    #[error("withdrawal repository failed")]
    WithdrawalRepository(#[from] RepositoryError<Withdrawal>),

    #[error("withdrawal aggregate failed")]
    Withdrawal(#[from] WithdrawalError),

    #[error("account was not found")]
    AccountNotFound,

    #[error("payout destination was not found")]
    PayoutDestinationNotFound,

    #[error("currency was not found")]
    CurrencyNotFound,
}
