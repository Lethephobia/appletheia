use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{CurrencyRegistrarId, CurrencyRegistrarStateError};

/// Describes why a CurrencyRegistrar aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyRegistrarId>),
    #[error(transparent)]
    State(#[from] CurrencyRegistrarStateError),
    #[error("currency registrar is already created")]
    AlreadyCreated,
}
