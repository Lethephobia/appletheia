use appletheia::domain::{AggregateStateError, ReferenceValuesError};
use thiserror::Error;

/// Describes why a wallet bookmark state value cannot be handled.
#[derive(Debug, Error)]
pub enum WalletBookmarkStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),
}
