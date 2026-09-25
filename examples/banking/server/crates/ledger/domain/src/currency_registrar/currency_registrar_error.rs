use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    CurrencyRegistrarCreateRejectionReason, CurrencyRegistrarHandleChangeRejectionReason,
    CurrencyRegistrarId, CurrencyRegistrarStateError,
};

/// Describes why a CurrencyRegistrar aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyRegistrarId>),
    #[error(transparent)]
    State(#[from] CurrencyRegistrarStateError),
    #[error("currency registrar is already created")]
    AlreadyCreated,
    #[error("currency registrar creation rejected: {0:?}")]
    CreateRejected(CurrencyRegistrarCreateRejectionReason),
    #[error("currency registrar handle change rejected: {0:?}")]
    HandleChangeRejected(CurrencyRegistrarHandleChangeRejectionReason),
}
