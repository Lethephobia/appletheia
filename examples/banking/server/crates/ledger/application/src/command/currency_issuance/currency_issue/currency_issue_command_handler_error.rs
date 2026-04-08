use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use banking_ledger_domain::currency_definition::{CurrencyDefinition, CurrencyDefinitionError};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceError};
use thiserror::Error;

/// Represents errors returned while starting a currency issuance.
#[derive(Debug, Error)]
pub enum CurrencyIssueCommandHandlerError {
    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("currency definition repository failed")]
    CurrencyDefinitionRepository(#[from] RepositoryError<CurrencyDefinition>),

    #[error("currency issuance repository failed")]
    CurrencyIssuanceRepository(#[from] RepositoryError<CurrencyIssuance>),

    #[error("currency definition aggregate failed")]
    CurrencyDefinition(#[from] CurrencyDefinitionError),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("currency issuance aggregate failed")]
    CurrencyIssuance(#[from] CurrencyIssuanceError),

    #[error("destination account was not found")]
    DestinationAccountNotFound,

    #[error("currency definition was not found")]
    CurrencyDefinitionNotFound,

    #[error("destination account currency definition does not match")]
    CurrencyDefinitionMismatch,

    #[error("currency issuance id was missing")]
    MissingCurrencyIssuanceId,
}
