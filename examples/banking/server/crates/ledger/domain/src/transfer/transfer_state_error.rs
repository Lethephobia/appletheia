use appletheia::domain::AggregateStateError;
use thiserror::Error;

/// Describes why a transfer state value cannot be handled.
#[derive(Debug, Error)]
pub enum TransferStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),
}
