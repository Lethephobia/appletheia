use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use banking_ledger_domain::currency::{Currency, CurrencyError};
use banking_ledger_domain::deposit::{Deposit, DepositError};
use thiserror::Error;

use crate::mint::{TokenAccountOwnerAddressValidatorError, TokenDepositPreparerError};

/// Represents errors returned while preparing a deposit token transfer.
#[derive(Debug, Error)]
pub enum DepositTokenTransferPrepareCommandHandlerError {
    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("deposit repository failed")]
    DepositRepository(#[from] RepositoryError<Deposit>),

    #[error("deposit aggregate failed")]
    Deposit(#[from] DepositError),

    #[error("pool token deposit preparer failed")]
    TokenDepositPreparer(#[from] TokenDepositPreparerError),

    #[error("token account owner address validation failed")]
    TokenAccountOwnerAddressValidator(#[from] TokenAccountOwnerAddressValidatorError),

    #[error("currency is not provisioned for on-chain deposit")]
    CurrencyUnprovisioned,
}
