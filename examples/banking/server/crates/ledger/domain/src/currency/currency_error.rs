use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{CurrencyId, CurrencyStateError};

/// Describes why a Currency aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyId>),
    #[error(transparent)]
    State(#[from] CurrencyStateError),
    #[error("currency is already defined")]
    AlreadyDefined,
}
