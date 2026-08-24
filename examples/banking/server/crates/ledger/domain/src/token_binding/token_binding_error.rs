use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{TokenBindingId, TokenBindingStateError};

#[derive(Debug, Error)]
pub enum TokenBindingError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<TokenBindingId>),
    #[error(transparent)]
    State(#[from] TokenBindingStateError),
    #[error("token binding is already defined")]
    AlreadyDefined,
    #[error("token address does not match the selected chain")]
    ChainMismatch,
}
