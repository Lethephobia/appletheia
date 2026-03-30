use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency_definition::{CurrencyDefinition, CurrencyDefinitionError};
use thiserror::Error;

/// Represents errors returned while activating a currency definition.
#[derive(Debug, Error)]
pub enum CurrencyDefinitionActivateCommandHandlerError {
    #[error("currency definition repository failed")]
    CurrencyDefinitionRepository(#[from] RepositoryError<CurrencyDefinition>),

    #[error("currency definition aggregate failed")]
    CurrencyDefinition(#[from] CurrencyDefinitionError),

    #[error("currency definition was not found")]
    CurrencyDefinitionNotFound,
}
