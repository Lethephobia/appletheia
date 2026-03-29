use appletheia::application::repository::RepositoryError;
use banking_iam_domain::RoleNameError;
use banking_ledger_domain::currency_definition::{CurrencyDefinition, CurrencyDefinitionError};
use thiserror::Error;

/// Represents errors returned while defining a currency definition.
#[derive(Debug, Error)]
pub enum CurrencyDefinitionDefineCommandHandlerError {
    #[error("currency definition repository failed")]
    CurrencyDefinitionRepository(#[from] RepositoryError<CurrencyDefinition>),

    #[error("currency definition aggregate failed")]
    CurrencyDefinition(#[from] CurrencyDefinitionError),

    #[error("role name is invalid")]
    RoleName(#[from] RoleNameError),

    #[error("currency definition id is missing after define")]
    MissingCurrencyDefinitionId,
}
