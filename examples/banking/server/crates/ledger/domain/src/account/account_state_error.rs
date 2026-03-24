use appletheia::domain::AggregateStateError;
use thiserror::Error;

/// Describes why an account state value cannot be handled.
#[derive(Debug, Error)]
pub enum AccountStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),
}
