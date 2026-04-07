use appletheia::application::repository::RepositoryError;
use appletheia::domain::AggregateId;
use banking_iam_domain::UserId;
use banking_ledger_domain::currency_definition::{CurrencyDefinition, CurrencyDefinitionError};
use thiserror::Error;

/// Represents errors returned while defining a currency definition.
#[derive(Debug, Error)]
pub enum CurrencyDefinitionDefineCommandHandlerError {
    #[error("currency definition repository failed")]
    CurrencyDefinitionRepository(#[from] RepositoryError<CurrencyDefinition>),

    #[error("currency definition aggregate failed")]
    CurrencyDefinition(#[from] CurrencyDefinitionError),

    #[error("currency definition id is missing after define")]
    MissingCurrencyDefinitionId,

    #[error("currency definition owner principal must be available")]
    OwnerRequiresPrincipal,

    #[error("currency definition owner principal must be a user")]
    OwnerRequiresUserPrincipal,

    #[error("currency definition owner principal contains an invalid user id")]
    InvalidOwnerUserId(#[source] <UserId as AggregateId>::Error),
}
