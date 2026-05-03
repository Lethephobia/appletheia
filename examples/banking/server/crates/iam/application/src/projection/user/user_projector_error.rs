use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::UserProjectionStoreError;

/// Represents errors returned while projecting user projections.
#[derive(Debug, Error)]
pub enum UserProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ProjectionStore(#[from] UserProjectionStoreError),
}
