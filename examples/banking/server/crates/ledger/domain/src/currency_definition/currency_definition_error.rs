use appletheia::domain::AggregateError;
use thiserror::Error;

use super::CurrencyDefinitionId;

/// Describes why a `CurrencyDefinition` aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyDefinitionError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyDefinitionId>),

    #[error("currency is already defined")]
    AlreadyDefined,

    #[error("currency is removed")]
    Removed,
}
