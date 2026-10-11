use appletheia::domain::{AggregateStateError, ReferenceValuesError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamMembershipCleanupStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),
}
