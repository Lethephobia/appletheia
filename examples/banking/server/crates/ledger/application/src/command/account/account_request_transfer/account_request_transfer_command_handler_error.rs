use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use thiserror::Error;

/// Represents errors returned while requesting an account transfer.
#[derive(Debug, Error)]
pub enum AccountRequestTransferCommandHandlerError {
    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("source account was not found")]
    SourceAccountNotFound,

    #[error("destination account was not found")]
    DestinationAccountNotFound,

    #[error("source and destination accounts use different currency definitions")]
    CurrencyDefinitionMismatch,
}
