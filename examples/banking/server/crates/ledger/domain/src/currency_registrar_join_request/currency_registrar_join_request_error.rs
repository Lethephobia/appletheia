use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{CurrencyRegistrarJoinRequestId, CurrencyRegistrarJoinRequestStateError};

/// Describes why an `CurrencyRegistrarJoinRequest` aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarJoinRequestError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyRegistrarJoinRequestId>),

    #[error(transparent)]
    State(#[from] CurrencyRegistrarJoinRequestStateError),

    #[error("currency registrar join request is already submitted")]
    AlreadySubmitted,
}
