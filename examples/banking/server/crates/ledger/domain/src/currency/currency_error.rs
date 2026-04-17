use appletheia::domain::AggregateError;
use thiserror::Error;

use super::CurrencyId;

/// Describes why a `Currency` aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyId>),

    #[error("currency is already defined")]
    AlreadyDefined,

    #[error("currency is inactive")]
    Inactive,

    #[error("currency is removed")]
    Removed,

    #[error("currency supply overflowed")]
    SupplyOverflow,

    #[error("currency supply is insufficient")]
    InsufficientSupply,
}
